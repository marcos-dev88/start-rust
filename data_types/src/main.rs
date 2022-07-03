fn main() {
 
    tuple_example_1();

    // let t = (400, 5.5, 88);
    let tuple2:(i32, f64, u8) = tuple_example_2((400, 5.5, 88));
    
    println!("tuple2 -> {:#?}", tuple2);

    tuple_example_3();
    println!("arrays:\n");
    arrays_example_1();
    arrays_example_2();
}


fn tuple_example_1(){ // Tuples are a kind of array, where you can put any kind of data inside that, and you can access their elements like below: 

    let tup: (i32, f64, u8) = (300, 1.8, 2);

    println!("data -> {}", tup.1)
}

fn tuple_example_2(t: (i32, f64, u8)) -> (i32, f64, u8) { // Example with function return
    return t
}

fn tuple_example_3(){

    let tup: (i16, i16, &str) = (30, 20, "Great job!");

    let (_x, y, _z) = tup;

    println!("value of y -> {}", y);
}

// ARRAYS:
fn arrays_example_1(){

    let a = [1, 2, 3, 4, 5];

    println!("array 'a' has those values -> {:#?}", a)
}

fn arrays_example_2(){

    //Defining an array with its size and type:

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("3º element of this array -> {}", a[2])
}
