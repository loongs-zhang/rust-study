use std::mem::MaybeUninit;
// Copyright 2016 coroutine-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use std::os::raw::c_void;
use std::ptr;
use crate::stack::error::StackError;
use crate::stack::frame::StackBox;

mod error;
pub mod frame;
mod system;

/// Represents any kind of stack memory.
///
/// `FixedSizeStack` as well as `ProtectedFixedSizeStack`
/// can be used to allocate actual stack space.
#[derive(Debug)]
pub struct Stack {
    protected: bool,
    top: *mut c_void,
    bottom: *mut c_void,
}

impl Stack {
    /// Allocates a new stack of `size`.
    /// If protected=false, a very simple and straightforward implementation.
    ///
    /// Allocates stack space using virtual memory, whose pages will
    /// only be mapped to physical memory if they are used.
    ///
    /// If protected=true, a more secure, but slightly slower.
    ///
    /// Allocates stack space using virtual memory, whose pages will
    /// only be mapped to physical memory if they are used.
    ///
    /// The additional guard page is made protected and inaccessible.
    /// Now if a stack overflow occurs it should (hopefully) hit this guard page and
    /// cause a segmentation fault instead letting the memory being overwritten silently.
    pub fn new(mut size: usize, protected: bool) -> Result<Stack, StackError> {
        let page_size = system::page_size();
        let min_stack_size = system::min_stack_size();
        let max_stack_size = system::max_stack_size();
        let add_shift = if protected { 1 } else { 0 };
        let add = page_size << add_shift;

        if size < min_stack_size {
            size = min_stack_size;
        }

        size = (size - 1) & !(page_size - 1);

        if let Some(size) = size.checked_add(add) {
            if size <= max_stack_size {
                let mut ret = unsafe { system::allocate_stack(size) };

                if protected {
                    if let Ok(stack) = ret {
                        ret = unsafe { system::protect_stack(&stack) };
                    }
                }

                return ret.map_err(StackError::IoError);
            }
        }

        Err(StackError::ExceedsMaximumSize(max_stack_size - add))
    }

    /// Creates a (non-owning) representation of some stack memory.
    ///
    /// It is unsafe because it is your reponsibility to make sure that `top` and `buttom` are valid
    /// addresses.
    #[inline]
    pub(crate) unsafe fn create(protected: bool, top: *mut c_void, bottom: *mut c_void) -> Stack {
        debug_assert!(top >= bottom);
        Stack {
            protected,
            top,
            bottom,
        }
    }

    /// Returns the top of the stack from which on it grows downwards towards bottom().
    #[inline]
    pub fn top(&self) -> *mut c_void {
        self.top
    }

    /// Returns the bottom of the stack and thus it's end.
    #[inline]
    pub fn bottom(&self) -> *mut c_void {
        self.bottom
    }

    /// Returns the size of the stack between top() and bottom().
    #[inline]
    pub fn len(&self) -> usize {
        self.top as usize - self.bottom as usize
    }

    /// Returns the minimal stack size allowed by the current platform.
    #[inline]
    pub fn min_size() -> usize {
        system::min_stack_size()
    }

    /// Returns the maximum stack size allowed by the current platform.
    #[inline]
    pub fn max_size() -> usize {
        system::max_stack_size()
    }

    /// Returns a implementation defined default stack size.
    ///
    /// This value can vary greatly between platforms, but is usually only a couple
    /// memory pages in size and enough for most use-cases with little recursion.
    /// It's usually a better idea to specifiy an explicit stack size instead.
    #[inline]
    pub fn default_size() -> usize {
        system::default_stack_size()
    }

    // get offset
    pub fn get_offset(&self) -> *mut usize {
        unsafe { (self.top as *mut usize).offset(-1) }
    }

    /// Point to the high end of the allocated stack
    pub fn end(&self) -> *mut usize {
        let offset = self.get_offset();
        unsafe { (self.top as *mut usize).offset(0 - *offset as isize) }
    }

    fn shadow_clone(&self) -> Self {
        Stack {
            protected: self.protected,
            top: self.top,
            bottom: self.bottom,
        }
    }

    // dealloc the stack
    fn drop_stack(&self) {
        if self.len() == 0 {
            return;
        }
        let page_size = system::page_size();
        let guard = if self.protected {
            (self.bottom() as usize - page_size) as *mut c_void
        } else {
            self.bottom()
        };
        let size_with_guard = if self.protected {
            self.len() + page_size
        } else {
            self.len()
        };
        unsafe {
            system::deallocate_stack(guard, size_with_guard);
        }
    }

    /// alloc buffer on this stack
    pub fn alloc_uninit_box<T>(&mut self) -> MaybeUninit<StackBox<T>> {
        // the first obj should set need drop to non zero
        StackBox::<T>::new_unit(self, 1)
    }

    /// get the stack cap
    #[inline]
    pub fn size(&self) -> usize {
        self.len() / std::mem::size_of::<usize>()
    }

    /// get used stack size
    pub fn get_used_size(&self) -> usize {
        let mut offset: usize = 0;
        unsafe {
            let mut magic: usize = 0xEE;
            ptr::write_bytes(&mut magic, 0xEE, 1);
            let mut ptr = self.bottom() as *mut usize;
            while *ptr == magic {
                offset += 1;
                ptr = ptr.offset(1);
            }
        }
        let cap = self.size();
        cap - offset
    }

    /// expand stack size
    fn expand(old_stack: Stack, ratio: usize) -> Result<Stack, StackError> {
        if ratio <= 1 {
            return Ok(old_stack);
        }
        let current_size = old_stack.len();
        let max_stack_size = system::max_stack_size();
        let new_size = current_size * ratio;
        if new_size >= max_stack_size {
            return Err(StackError::ExceedsMaximumSize(max_stack_size));
        }
        match unsafe { system::allocate_stack(new_size) } {
            Ok(new_stack) => {
                unsafe {
                    // data migration
                    system::copy_stack(&new_stack, &old_stack);
                    // deallocate old stack
                    system::deallocate_stack(old_stack.bottom, current_size);
                    if old_stack.protected {
                        return system::protect_stack(&new_stack).map_err(StackError::IoError);
                    }
                };
                Ok(new_stack)
            }
            Err(e) => Err(StackError::IoError(e)),
        }
    }

    /// reduce stack size
    fn reduce(old_stack: Stack, ratio: usize) -> Result<(), StackError> {
        if ratio <= 1 {
            return Ok(());
        }
        let current_size = old_stack.len();
        let new_size = current_size / ratio;
        unsafe { system::reduce_stack(&old_stack, new_size) }
        return Ok(());
    }
}

impl Default for Stack {
    fn default() -> Stack {
        Stack::new(Stack::default_size(), true)
            .unwrap_or_else(|err| panic!("Failed to allocate Stack with {:?}", err))
    }
}

unsafe impl Send for Stack {}

#[cfg(test)]
mod tests {
    use std::ptr::write_bytes;

    use crate::stack::system;

    use super::*;

    #[test]
    fn stack_size_too_small() {
        let stack = Stack::new(0, false).unwrap();
        assert_eq!(stack.len(), system::min_stack_size());

        unsafe { write_bytes(stack.bottom() as *mut u8, 0x1d, stack.len()) };

        let stack = Stack::new(0, true).unwrap();
        assert_eq!(stack.len(), system::min_stack_size());

        unsafe { write_bytes(stack.bottom() as *mut u8, 0x1d, stack.len()) };
    }

    #[test]
    fn stack_size_too_large() {
        let stack_size = system::max_stack_size() & !(system::page_size() - 1);
        match Stack::new(stack_size, false) {
            Err(StackError::ExceedsMaximumSize(..)) => panic!(),
            _ => {}
        }

        let stack_size = stack_size + 1;
        match Stack::new(stack_size, false) {
            Err(StackError::ExceedsMaximumSize(..)) => {}
            _ => panic!(),
        }
    }
}
