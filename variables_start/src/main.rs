
fn main(){
    constants();
    shadowing();
}

fn constants() {
    const MY_FIRST_CONST: &str = "data";

    println!("my const -> {}", MY_FIRST_CONST)
}

fn shadowing() {

    // Shadowing enable us to use the same variable as created b4 to input a new data value
    let x = 5;

    let x = x + 1; //Creating a new variable, bcz it's using `let` keyword again

    {
        // Here is getting same variable and inserting a new value just inside this scope
        let x = x * 2;
        println!("Value of x inside of scope -> {}", x)
    }

    println!("value of original x -> {}", x)
}
