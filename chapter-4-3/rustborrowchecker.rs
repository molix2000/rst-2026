fn main() {
    // this should later be called multi_mut_example() //
    let mut x = 1;
    let mx1 = &mut x;
    //let mx2 = &mut x;
    // can't have a number of mutable references to the same object at the same time
    
    // but can have multiple immutable references to the same object at the same time
    // reason is cache coherence and race conditions.
    *mx1 = 2;
    // The line above is a reference to a location in memory, and the line below is a reference to the value at that location in memory.
    // Rust has pointers like *mut T *const T and they are regarded as unsafe. 
    // It also has smart pointers such  as Box<T> Rc<T> and Arc<T> , RefCell<T> and Mutex<T> which are safe to use.
    let mx2 = &x;
    println!("X = {}", mx1);
}
