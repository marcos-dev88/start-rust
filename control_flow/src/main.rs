fn main() {
    println!("Hello, world!");

let    can_enter: &str = if_statement(19);

println!("value: {}", can_enter);
}

fn if_statement(age: i8) -> &'static str{

    if age > 18 { 
        return "Enjoy";
    }

    return "You need to have 18+ to enter";
}
