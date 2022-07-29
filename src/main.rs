fn main() {
    let x = 5;

    let x = x + 1;//并非为x赋值，这里重新定义了一个x

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    /////////////////////////////////////////////
    // The value of x in the inner scope is: 12//
    // The value of x is: 6                    //
    /////////////////////////////////////////////
}
