//reference counted, never provide an exclusive reference
//not Sync. no thread Safe -> only single thread can mutate

//a mutable reference guarantees that no other thread can mutate the value, exclusive reference
//a mutable pointer, *mut T, don't carry the additional implication of exclusive ownership, so  it's allow to mutate the value
//SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
use crate::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}
//for support dyn sized types we use !Sized --> complicate implementation
struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: std::marker::PhantomData<RcInner<T>>,
    // Phantom data notice the compiler like we own
    // 'something' of type T (instead of just treat it as a pointer) so when we drop Rc,
    // we drop the inner value also,
}
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        //SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        //We have an Rc, therefore the Box is not deallocated, so drop is safe.
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.ref_count.get();
        if count == 1 {
            //we are the last Rc
            //deallocate the Box
            unsafe {
                drop(Box::from_raw(self.inner.as_ptr()));
            }
        } else {
            //we are not the last Rc
            //decrement the ref count
            inner.ref_count.set(count - 1);
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
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        //unsafe keyword means -> for me, the programmer, inside is safe
        let inner = unsafe { self.inner.as_ref() };
        inner.ref_count.set(inner.ref_count.get() + 1);
        Rc {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rc() {
        let rc = Rc::new(5);
        let rc2 = rc.clone();
        assert_eq!(*rc, 5);
        assert_eq!(*rc2, 5);
    }
    #[test]
    fn test_rc_1() {
        let value = 45;
        let (x, y);
        {
            let rc = Rc::new(&value);
            let rc2 = rc.clone();
            x = rc;
            y = rc2;
        }
        assert_eq!(*x, &value);
        assert_eq!(*y, &value);
    }
}
