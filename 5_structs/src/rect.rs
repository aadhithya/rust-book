// This is a struct
// Struct has named fields.
pub struct Rectangle {
    width: u32,
    height: u32,
}

// this is an implementation of the struct.
// It holds functions(here methods) for the struct.
impl Rectangle {
    // Every method of a struct has its first param as self (or) &self.
    // It can take ownership(rare cases) or borrow self.
    //  &self is a shorthand for: &Self.
    // methods are accesed with the `.` operator.
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    // functions that don't take self as first param arte called associated functions.
    // They are commonly used as constructors.
    // They can be used to return Self, which is of type of the struct.
    // these methods are accesed using the `::` operator.
    pub fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    pub fn rect(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
