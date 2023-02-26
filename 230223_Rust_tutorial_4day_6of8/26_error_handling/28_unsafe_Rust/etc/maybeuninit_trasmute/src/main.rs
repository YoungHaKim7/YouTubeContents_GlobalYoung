use std::mem::{forget, MaybeUninit, transmute};

// first extra part: We need a "guard" that drops all *initialized* elements
// on drop
struct Guard<'a> {
    // a mutable ref to access the array from
    array: &'a mut [MaybeUninit<MyType>; 256],
    // the index until which all elements are initialized
    index: usize,
}

impl Drop for Guard<'_> {
    // drop all elements that are initialized
    fn drop(&mut self) {
        for i in 0..self.index {
            unsafe {
                std::ptr::drop_in_place(self.array[i].as_mut_ptr());
            }
        }
    }
}

unsafe {
    let mut array: [MaybeUninit<MyType>; 256] =
        MaybeUninit::uninit().assume_init();

    // second extra part: here we initialize the guard. From here on, it
    // borrows our array mutably. All access will be done through the guard
    // (because the borrow checker won't let us access `array` directly
    // while it's mutably borrowed).
    let mut guard = Guard { array: &mut array, index: 0 };
    for i in 0..256 {
        guard.array[guard.index] = MaybeUninit::new(calculate_elem(i));
        // update the index so `drop` will include the newly created element.
        guard.index += 1;
    }
    // third extra part: forget the guard to avoid dropping the initialized
    // elements and also end the borrow.
    forget(guard);

    transmute::<_, [MyType; 256]>(array)
}
fn main () {
    println!("Hello, world!");
}
