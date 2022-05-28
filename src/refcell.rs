// -> exclusive ref -> can mutate / shared ref -> can't mutate
//-> borrow checking -> compile time
// ref cell -> safe dynamic checked borrowing -> good for graphs and trees
use crate::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

struct RefCell<T> {
    value: UnsafeCell<T>, // -> is !Sync
    state: Cell<RefState>,
}
struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
impl<T> Drop for Ref<'_, T> {
    //when you borrow a refcell(must be in shared state), then when you drop the ref:
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Exclusive => unreachable!(), //must be shared
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}
impl<T> Drop for RefMut<'_, T> {
    //we have an exclusive ref
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

//dereference into the inner type of T
impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY: a Ref is only created if no exclusive references have been given out.
        //Once it's given out, state is set to Shared(1), so no Exclusive refs can be given out.
        //So deReferencing a  shared ref is safe.
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY: see deref_mut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    //true for mutable refs
    fn deref_mut(&mut self) -> &mut Self::Target {
        //SAFETY: a RefMut is only created if no other references have been given out.
        //Once it's given out, state is set to Exclusive, so no other refs can be given out in the future.
        //So we have an exclusive lease on the inner value, so deReferencing a mutable ref is safe.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }
    //"Some" at least an exclusive reference borrow already giving up the mutability
    fn borrow(&self) -> Option<Ref<'_, T>> {
        //mutate through a shared reference
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: no exclusive references have been given out since state would be Exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Shared(ref_count) => {
                self.state.set(RefState::Shared(ref_count + 1));
                // SAFETY: no exclusive references have been given out since state would be Exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }
    fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state would be Exclusive or Shared.
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}
