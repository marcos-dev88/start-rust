
fn main(){
    constants();
    shadowing();
    shadowing2();
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

fn shadowing2() {

    let mut _spaces = "     "; // How it is an unused variable intentionally, then we must to use a '_'

   // spaces = spaces.len(); # it was not possible because we are not able to change a variable
   // type with mut
   // even though we're able to do that with shaddowing:

    let spaces2 = "     ";
    let spaces2 = spaces2.len();

    println!("value of spaces -> {:#?}", spaces2)
}
