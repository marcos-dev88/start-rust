fn main(){ 
    println!("Hello, world!");

let    can_enter: &str = if_statement(19);

println!("value: {}", can_enter);

if_variable();
if_variable_2();
loop_example_1();
loop_example_2();
loop_example_3();
loop_example_4();
loop_example_5();
}

fn if_statement(age: i8) -> &'static str{

    if age > 18 { 
        return "Enjoy";
    }

    return "You need to have 18+ to enter";
}

fn if_variable(){
    let condition: bool = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number)
}

fn if_variable_2(){
    let condition = true;

    let str_val: &str = if condition { "some" } else {  "else" };

    println!("The value of myStr is: {:?}",str_val)
}

fn loop_example_1(){
    // for repetion loop with intentionally infinite loop, you should use "loop"
    // of course, this code of block will be commented to don't generate a stackoverflow hehe

    //loop {
     //   println!("infinite...")
    //}
}

fn loop_example_2(){

    let mut i: i32 = 0;

    loop {

        if i == 10 {
            break;
        }
        i += 1;

        println!("value of i {}", i)
    }
}


fn loop_example_3() {

    let mut i: i32 = 0 ;

    // With while here, if condition stops to be true, the while calls a break to kill our loop'
    // This code below, stops when the 'i' variable turns to 10 
    while i != 10 {
        println!("value of i {}", i);

        i += 1;
    }
}

fn loop_example_4() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_example_5(){
    // this function rev() makes the reverse loop
    for n in (1..4).rev() {
        println!("{n}!");
    }
    println!("LIFTOFF!");
} 
