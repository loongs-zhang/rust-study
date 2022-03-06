use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr;
use crate::stack::Stack;

// must align with StackBoxHeader
const ALIGN: usize = std::mem::size_of::<StackBoxHeader>();
const HEADER_SIZE: usize = std::mem::size_of::<StackBoxHeader>() / std::mem::size_of::<usize>();

struct StackBoxHeader {
    // track the stack
    stack: Stack,
    // track how big the data is (in usize)
    data_size: usize,
    // non zero dealloc the stack
    need_drop: usize,
}

/// A pointer type for stack allocation.
pub struct StackBox<T> {
    // the stack memory
    ptr: ptr::NonNull<T>,
}

impl<T> StackBox<T> {
    /// create uninit stack box
    pub(crate) fn new_uninit(stack: &mut Stack, need_drop: usize) -> MaybeUninit<Self> {
        let offset = unsafe { &mut *stack.get_offset() };
        // alloc the data
        let layout = std::alloc::Layout::new::<T>();
        let align = std::cmp::max(layout.align(), ALIGN);
        let size = ((layout.size() + align - 1) & !(align - 1)) / std::mem::size_of::<usize>();
        let u_align = align / std::mem::size_of::<usize>();
        let pad_size = u_align - (*offset + size) % u_align;
        let data_size = size + pad_size;
        *offset += data_size;
        let ptr = unsafe { ptr::NonNull::new_unchecked(stack.end() as *mut T) };

        // init the header
        *offset += HEADER_SIZE;
        unsafe {
            let mut header = ptr::NonNull::new_unchecked(stack.end() as *mut StackBoxHeader);
            let header = header.as_mut();
            header.data_size = data_size;
            header.need_drop = need_drop;
            header.stack = stack.shadow_clone();
            std::mem::MaybeUninit::new(StackBox { ptr })
        }
    }

    fn get_header(&self) -> &StackBoxHeader {
        unsafe {
            let header = (self.ptr.as_ptr() as *mut usize).offset(0 - HEADER_SIZE as isize);
            &*(header as *const StackBoxHeader)
        }
    }

    /// move data into the box
    pub(crate) fn init(&mut self, data: T) {
        unsafe { ptr::write(self.ptr.as_ptr(), data); }
    }

    // get the stack ptr
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Constructs a StackBox from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    #[inline]
    pub(crate) fn from_raw(raw: *mut T) -> Self {
        unsafe {
            StackBox {
                ptr: ptr::NonNull::new_unchecked(raw),
            }
        }
    }
}

pub struct Func {
    data: *mut (),
    size: usize,
    offset: *mut usize,
    func: fn(*mut ()),
    drop: fn(*mut ()),
}

impl Func {
    pub fn call_once(mut self) {
        let data = self.data;
        self.data = ptr::null_mut();
        (self.func)(data);
    }
}

impl Drop for Func {
    fn drop(&mut self) {
        if !self.data.is_null() {
            (self.drop)(self.data);
        }
        unsafe { *self.offset -= self.size };
    }
}

impl<F: FnOnce()> StackBox<F> {
    fn call_once(data: *mut ()) {
        unsafe {
            let data = data as *mut F;
            let f = data.read();
            f();
        }
    }

    fn drop_inner(data: *mut ()) {
        unsafe {
            let data = data as *mut F;
            ptr::drop_in_place(data);
        }
    }

    /// create a functor on the stack
    pub(crate) fn new_fn_once(stack: &mut Stack, data: F) -> Func {
        unsafe {
            let mut d = Self::new_uninit(stack, 0).assume_init();
            d.init(data);
            let header = d.get_header();
            let f = Func {
                data: d.ptr.as_ptr() as *mut (),
                size: header.data_size + HEADER_SIZE,
                offset: stack.get_offset(),
                func: Self::call_once,
                drop: Self::drop_inner,
            };
            std::mem::forget(d);
            f
        }
    }
}

impl<T> Deref for StackBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr.as_ref() }
    }
}

impl<T> DerefMut for StackBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_mut() }
    }
}

impl<T> Drop for StackBox<T> {
    fn drop(&mut self) {
        let header = self.get_header();
        unsafe {
            *header.stack.get_offset() -= header.data_size + HEADER_SIZE;
            ptr::drop_in_place(self.ptr.as_ptr());
            if header.need_drop != 0 {
                header.stack.drop_stack();
            }
        }
    }
}
