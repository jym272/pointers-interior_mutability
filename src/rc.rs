//reference counted, never provide an exclusive reference
//not Sync. no thread Safe -> only single thread can mutate
use crate::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        //SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        //We have an Rc, therefore the Box is not deallocated, so drop is safe.
        let inner = unsafe { self.inner.as_ref() };
        if inner.ref_count.get() == 1 {
            //we are the last Rc
            //deallocate the Box
            unsafe {
                drop(Box::from_raw(self.inner.as_ptr()));
            }
        } else {
            //we are not the last Rc
            //decrement the ref count
            inner.ref_count.set(inner.ref_count.get() - 1);
        }
    }
}

impl<T> Rc<T> {
    fn new(value: T) -> Self {
        // SAFETY: Box does not give us a null pointer, give us a heap allocated pointer
        let inner = Box::new(RcInner {
            value,
            ref_count: Cell::new(1),
        });
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        //unsafe keyword means -> for me, the programmer, inside is safe
        let inner = unsafe { self.inner.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);
        Rc { inner: self.inner }
    }
}
impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        //We have an Rc, therefore the Box is not deallocated, so deref is safe.
        &unsafe { self.inner.as_ref() }.value
    }
}
