mod i2structs;
mod rect;

fn intro_to_structs() {
    let user_1 = i2structs::build_user(String::from("Adi"), String::from("adi@adi.in"), true);
    let user_2 = i2structs::User {
        name: String::from("aadhi"),
        ..user_1
    };

    println!(
        "name: {}, email: {}, active: {}",
        user_2.name, user_2.email, user_2.active
    );

    println!("{}", user_1.name);

    // println!("{}", user_1.email);    // This throws an error because email and active was moved to user_2.

    let my_color = i2structs::Color(128, 0, 179); // Tuple Structs are Tuples with a name.

    println!(
        "My Colour: ({}, {}, {})",
        my_color.0, my_color.1, my_color.2
    );
}

fn area_rectangle() {
    let rectangle = rect::Rectangle::rect(32, 54);

    println!(
        "Area of rectangle with width {} and height {}: {} sq. px.",
        rectangle.width(),
        rectangle.height(),
        rectangle.area()
    );

    let square = rect::Rectangle::square(15);

    println!(
        "Area of square with side {}: {} sq. px",
        square.width(),
        square.area()
    );
}

fn main() {
    intro_to_structs();
    area_rectangle();
}
