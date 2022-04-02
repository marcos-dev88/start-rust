fn main() {
 
    tuple_example_1();

    // let t = (400, 5.5, 88);
    let tuple2:(i32, f64, u8) = tuple_example_2((400, 5.5, 88));
    
    println!("tuple2 -> {:#?}", tuple2)
}


fn tuple_example_1(){ // Tuples are a kind of array, where you can put any kind of data inside that, and you can access their elements like below:

    let tup: (i32, f64, u8) = (300, 3.4, 8); 
    println!("data -> {}", tup.1)
}

fn tuple_example_2(t: (i32, f64, u8)) -> (i32, f64, u8) { // Example with function return
    return t
}
