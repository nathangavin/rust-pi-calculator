fn main() {
    println!("Hello World!");
    let x: i32 = 5 + 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
                object="the laxy dog",
                subject="the quick brown fox",
                verb="jumps over");
    

    println!("base 10 repr:                 {}", 69420);
    println!("base 2 (binary) repr:         {:b}", 69420);
    println!("base 8 (octal) repr:          {:o}", 69420);
    println!("base 16 (hexidecimal) repr:   {:x}", 69420);
    println!("base 16 (hexidecimal) repr:   {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);


    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}