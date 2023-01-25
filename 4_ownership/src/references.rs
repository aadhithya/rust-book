pub fn do_something(s: &String) {
    // s is a reference to a string. References are immutable by default.
    println!("doing something to s: {s}");
} // s goes out of scope, but it is not dropped as
  // it does not have ownership of the string it refers to.

pub fn multiple_mut_references() {
    {
        let mut x1 = 5;
        let x2 = &mut x1; // This is allowed
                          // let x3 = &mut x1; // This is not allowed because rust does not allow data race!
        println!("x2: {x2}");
    }

    {
        let mut x1 = 15;
        x1 += 1;
        // Having multiple immutable references is allowed.
        // because the data is just being read.
        let x2 = &x1;
        let x3 = &x1;
        // let x4 = &mut x1; // This is not allowed because the immutable references don't expect the underlying data to change suddenly.
        println!("x1: {x1}, x2: {x2}, x3: {x3}");

        // Now mutable reference is allowed.
        // This is because scope of reference is until the last line it is used.
        let x4 = &mut x1;
        *x4 += 1;
        println!("x4: {x4}");
        println!("new x1: {x1}");
    }
}
