// Copyright 2016 coroutine-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use std::ops::Deref;
use std::os::raw::c_void;
use crate::stack::error::StackError;

mod error;
pub mod system;

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
    /// Creates a (non-owning) representation of some stack memory.
    ///
    /// It is unsafe because it is your reponsibility to make sure that `top` and `buttom` are valid
    /// addresses.
    #[inline]
    pub unsafe fn new(protected: bool, top: *mut c_void, bottom: *mut c_void) -> Stack {
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

    /// Allocates a new stack of `size`.
    fn allocate(mut size: usize, protected: bool) -> Result<Stack, StackError> {
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

unsafe impl Send for Stack {}

/// A very simple and straightforward implementation of `Stack`.
///
/// Allocates stack space using virtual memory, whose pages will
/// only be mapped to physical memory if they are used.
///
/// _As a general rule it is recommended to use `ProtectedFixedSizeStack` instead._
#[derive(Debug)]
pub struct FixedSizeStack(Stack);

impl FixedSizeStack {
    /// Allocates a new stack of **at least** `size` bytes.
    ///
    /// `size` is rounded up to a multiple of the size of a memory page.
    pub fn new(size: usize) -> Result<FixedSizeStack, StackError> {
        Stack::allocate(size, false).map(FixedSizeStack)
    }
}

impl Deref for FixedSizeStack {
    type Target = Stack;

    fn deref(&self) -> &Stack {
        &self.0
    }
}

impl Default for FixedSizeStack {
    fn default() -> FixedSizeStack {
        FixedSizeStack::new(Stack::default_size())
            .unwrap_or_else(|err| panic!("Failed to allocate FixedSizeStack with {:?}", err))
    }
}

impl Drop for FixedSizeStack {
    fn drop(&mut self) {
        unsafe {
            system::deallocate_stack(self.0.bottom(), self.0.len());
        }
    }
}

/// A more secure, but slightly slower version of `FixedSizeStack`.
///
/// Allocates stack space using virtual memory, whose pages will
/// only be mapped to physical memory if they are used.
///
/// The additional guard page is made protected and inaccessible.
/// Now if a stack overflow occurs it should (hopefully) hit this guard page and
/// cause a segmentation fault instead letting the memory being overwritten silently.
///
/// _As a general rule it is recommended to use **this** struct to create stack memory._
#[derive(Debug)]
pub struct ProtectedFixedSizeStack(Stack);

impl ProtectedFixedSizeStack {
    /// Allocates a new stack of **at least** `size` bytes + one additional guard page.
    ///
    /// `size` is rounded up to a multiple of the size of a memory page and
    /// does not include the size of the guard page itself.
    pub fn new(size: usize) -> Result<ProtectedFixedSizeStack, StackError> {
        Stack::allocate(size, true).map(ProtectedFixedSizeStack)
    }
}

impl Deref for ProtectedFixedSizeStack {
    type Target = Stack;

    fn deref(&self) -> &Stack {
        &self.0
    }
}

impl Default for ProtectedFixedSizeStack {
    fn default() -> ProtectedFixedSizeStack {
        ProtectedFixedSizeStack::new(Stack::default_size()).unwrap_or_else(|err| {
            panic!("Failed to allocate ProtectedFixedSizeStack with {:?}", err)
        })
    }
}

impl Drop for ProtectedFixedSizeStack {
    fn drop(&mut self) {
        let page_size = system::page_size();
        let guard = (self.0.bottom() as usize - page_size) as *mut c_void;
        let size_with_guard = self.0.len() + page_size;
        unsafe {
            system::deallocate_stack(guard, size_with_guard);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::write_bytes;

    use crate::stack::system;

    use super::*;

    #[test]
    fn stack_size_too_small() {
        let stack = FixedSizeStack::new(0).unwrap();
        assert_eq!(stack.len(), system::min_stack_size());

        unsafe { write_bytes(stack.bottom() as *mut u8, 0x1d, stack.len()) };

        let stack = ProtectedFixedSizeStack::new(0).unwrap();
        assert_eq!(stack.len(), system::min_stack_size());

        unsafe { write_bytes(stack.bottom() as *mut u8, 0x1d, stack.len()) };
    }

    #[test]
    fn stack_size_too_large() {
        let stack_size = system::max_stack_size() & !(system::page_size() - 1);

        match FixedSizeStack::new(stack_size) {
            Err(StackError::ExceedsMaximumSize(..)) => panic!(),
            _ => {}
        }

        let stack_size = stack_size + 1;

        match FixedSizeStack::new(stack_size) {
            Err(StackError::ExceedsMaximumSize(..)) => {}
            _ => panic!(),
        }
    }
}
