// Introduction to structs.

pub struct User {
    pub name: String,
    pub active: bool,
    pub email: String,
}

pub struct Color(pub u8, pub u8, pub u8);

pub fn build_user(name: String, email: String, active: bool) -> User {
    User {
        name,
        email,
        active,
    }
}
