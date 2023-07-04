fn main() {
    let str: String = String::from("Hello");

    // let s: &str = "a";

    println!("{}, world!", str);

    let t = (20, 10, 30);
    println!("{}", t.0);

    for i in 0..3 {
        println!("{}", i)
    }

}
