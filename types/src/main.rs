
fn main() {

    // mutable , immutable değişkenler 
    let var: i32 = 19;      // var değişkeni immutable (değiştirilemez) 'dır. 
    //var=10;               // error : immutable 
    let mut var2: i16 = 10; // var2 değişkeni mutable (değiştirilebilir) 'dir.
    var2 = 101;
    println!("var2 : {}", var2);
    
    // constant değişken
    const PI: f64 = 3.14;   // const ifadesi ile oluşturulan PI değişkeninin değeri değiştirilemez 
    
    // temel veri tipleri 
    let x : i16 = 19;           // int
    let ch : char = 'M';        // char
    let flt: f32 = 1.008;       // float
    let y_n: bool = false;      // boolean
    
    // tuple
    let mytuple = (1, 'A', 3.12);
    let mytuple2 : (i32, i32, i32) = (1, 2, 3);
    let y: i32 = mytuple2.2;
    print!("y : {} \n", y);
    
    // array
    let arr: [i16;4] = [2, 3, 6, 1];
    let x = arr[0];
    
}
