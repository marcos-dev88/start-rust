fn main() {
    println!("Hello, world!");
    let s1: String = String::from("Rust is awesome!");
    let len = calculate_length(&s1);

    println!("The length of \"{}\" is {}.", s1, len);
    mut_reference();
    dangle_example();
}

fn calculate_length(s: &String) -> usize{
    return s.len();
}

fn mut_reference(){
    let mut s1: String = String::from("Hello");
   
    println!("value of s1 b4 the change: \"{}\"", s1);

    change(&mut s1);

    println!("value of s1 after change: \"{}\"", s1);
}

fn change(some_str: &mut String){
    some_str.push_str(", world");
}

fn dangle_example() {
    let s1 = d_e();

    println!("value of s1 -> '{}'", s1);
}

fn d_e() -> String {
    let s1 = String::from("Hello World");

    s1
}
