pub fn mutability() {
    println!("\n--- mutability() ---");

    let immutable = 1;
    println!("immutable={}", immutable);

    //immutable = 2; //not allowed because immutable by default
    let immutable = 2; //allowed because a is shadowed
    println!("immutable={}", immutable);

    let mut mutable = 1;
    println!("mutable={}", mutable);

    mutable = 2;
    println!("mutable={}", mutable);

    //mutable = "Mutable"; //mutation of type is not allowed!
}