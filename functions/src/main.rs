fn main() {
    println!("Hello, world!");
    sup_func();
    func_with_param(5);
    func_with_param_2(40, 'h');
    var_as_func();
    println!("add a value using plus one function: {}", plus_one(7));

    let (x,y) = tuple_example(7);

    println!("value of x: {} | value of y: {}", x, y);
}

// By convention all functions and variables are created with snake_case pattern 
fn sup_func(){

    println!("Hello I'm marcos\n");

}

fn func_with_param(x: i32){

    println!("Passed value in function's param is \"{}\"\n",x);
}

fn func_with_param_2(measure: i32, measure_unit_label: char){

    println!("The measurement is: {}{}", measure, measure_unit_label);
}

fn var_as_func(){

    let mut x: i32;
  x = 5; 
    x = {

        let y = 7;
        y + x // it's same as: return y + x
    };

    println!("value of x: {:#?}", x);
}

fn plus_one(x: i32) -> i32{
    return x+1;
}

fn tuple_example(x: i32) -> (i32, &'static str){

return (x, "Hello using tuples!");
}
