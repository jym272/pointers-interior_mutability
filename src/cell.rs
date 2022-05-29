//only with this can mutate from a shared reference to an exclusive reference
use std::cell::UnsafeCell;

pub(crate) struct Cell<T> {
    value: UnsafeCell<T>, //unsafeCell is notSync -> cell is notSync
}
// NOTSYNC is implied by UnsafeCell<T> ->
// impl<T> !Sync for Cell<T> {}

// Implementing Unsafe Sync for bad_cell test
// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    //new
    pub(crate) fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    pub(crate) fn set(&self, value: T) {
        //SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        //SAFETY: we know we're not invalidating any references, because we never give any out
        unsafe {
            *self.value.get() = value;
        }
    }
    pub(crate) fn get(&self) -> T
    where
        T: Copy, //only get method needs the copy trait
    {
        {
            //SAFETY: we know no-one else is modifying this value, since only this thread can mutate
            // (because !Sync), and it is executing this function instead.
            unsafe { *self.value.get() }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Cell;

    //two ref to the same cell and different threads can mutate the same cell using set
    //at the same time, what is the value of the cell? --> NOT OK!
    // #[test]
    // fn bad_cell() {
    //     use std::sync::Arc;
    //     use std::thread::spawn;
    //     let x = Arc::new(Cell::new(0));
    //
    //     let x_copy_1 = Arc::clone(&x);
    //     //line 10
    //     let thread_1 = spawn(move || {
    //         for _ in 0..100000 {
    //             let x = x_copy_1.get();
    //             x_copy_1.set(x + 1);
    //         }
    //     });
    //
    //     let x_copy_2 = Arc::clone(&x);
    //     let thread_2 = spawn(move || {
    //         for _ in 0..100000 {
    //             let x = x_copy_2.get();
    //             x_copy_2.set(x + 1);
    //         }
    //     });
    //     thread_2.join().unwrap();
    //     thread_1.join().unwrap();
    //     //the threads starts to race, some of the modifications end up being lost
    //     //both threads write before read again
    //     //cargo t --lib bad_cell
    //     assert_eq!(x.get(), 100000 * 2);
    // }

    // #[test]
    // fn bad_2() {
    //     let x = Cell::new(String::from("hello"));
    //     let first = x.get(); //point to hello
    //     x.set(String::new());
    //     x.set(String::from("world"));
    //     println!("{:?}", first);
    // }
}
