mod references;

fn main() {
    // string literal is always immutable and cannot be made mutable.
    // this is because string can have different sizes and therefore
    //  this cannot be known beforehand during compileation.
    let string = "hello, world!";
    println!("{}", string);

    // Too support string mutability, we have String. This
    // allocates space in the heap and is therefore growable.
    // String:from requests the allocator the memory it needs during runtime.
    let mut string2: String = String::from("Hello");

    string2.push_str(", World!");
    println!("{string2}");

    // rust always returns memory at the end of the variable's scope.
    {
        let mut inner_str = String::from("Hello World from inner scope..."); // inner_str variable scope begins.
        inner_str.push_str("yep");
        println!("{inner_str}");
    } // inner_str memory allocation is release.

    // println!("{inner_str}"); // This line will throw a compiler error because inner_str was released when we exited the scope.

    // Move
    {
        let s1 = String::from("Hello"); // We create String s1
        let s2 = s1; // We assign s1 to s2.
                     // In rust this operation is the move operation.
                     // s1 is invalidated after assigning it to s2.
                     // This is fundementally different to how other languages handle this.
                     // e.g. in python `s2 = s1` would do a shallow copy and both s1 and s2 would be accessible.
                     // This is not done in rust because it is a very expensive operation.

        // println!("{}", s1); // This line will throw an error.
        println!("s2 = s1, s2 = {s2}");
    }
    {
        let int1 = 32;
        let int2 = int1;

        println!("int1: {}, int2: {}", int1, int2);
        // This works because int, float and primitive types have a fixed size
        // and are stored on the stack. Therefore they are super cheap and quick to be copied.
    }

    // Ownership and functions.
    {
        // When we pass values to a function, the variable is either moved or copied
        // similar to assigning a variable to another.
        let s = String::from("Hello, String.");

        takes_ownership(s); // Here s is moved to the function
                            // and the function takes ownership of s.
                            // s is no longer valid in this scope here.
                            // println!("{s}"); // This line will throw an error as the ownership of s has changed.
        let x: i8 = 3;
        make_copy(x); //Here x is copied to the function and therefore x is still valid in this scope.

        println!("The value of x: {x}");
    }

    {
        let s = String::from("String says Hello!");
        references::do_something(&s);
        println!("Main still has ownership of string!: {s}");
        references::multiple_mut_references();
    }
    // Function return statements gives back ownership.
}

fn takes_ownership(string: String) {
    // string comes into scope through the move op.
    println!("From `takes_ownership`, string: {string}");
} // stirng goes out of scope and Drop is called.

fn make_copy(ix: i8) {
    // ix comes into scope through Copy.
    println!("The value of ix: {ix}");
} // ix goes out of scope.
