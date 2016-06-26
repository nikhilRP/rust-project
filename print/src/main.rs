fn main() {
    println!("{} is my age", 31);
    println!("{:?} is my age", 31);
    println!("{0} is my naam, {1} is my khaam", "Nikhil", "data munching");
    println!("{first} is my first name, {last} is my last name",
        first="Nikhil",
        last="podduturi"
    );
    println!("{} of {:b} people know binary, the other half don't", 1, 1);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0width$}", number=1, width=6);

    let pi = 3.141592;
    println!("{:.*}", 3, pi);
}
