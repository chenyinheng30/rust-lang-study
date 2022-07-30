fn main() {
    println!("Hello, world!");

    another_function_void();

    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(x);

    println!("The value of x is: {x}");

    // fn main END OF HERE.
    ///////////////////////////////////////////////////////////////////
    // Rust 是一门基于表达式（expression-based）的语言               //
    // 函数体由一系列的语句和一个可选的结尾表达式构成。              //
    // 语句（Statements）是执行一些操作但不返回值的指令。            //
    // 下面是语句：                                                  //
    // fn main() {                                                   //
    //   let y = 6;                                                  //
    // }                                                             //
    // 表达式（Expressions）计算并产生一个值。                       //
    // 下面是表达式：                                                //
    // {                                                             //
    //   let x = 3;                                                  //
    //   x + 1                                                       //
    // }                                                             //
    // 宏调用也是一个表达式。                                        //
    // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。//
    // 下面的函数因为没有返回值在编译时报错：                        //
    // fn plus_one(x: i32) -> i32 {                                  //
    //     x + 1;                                                    //
    // }                                                             //
    ///////////////////////////////////////////////////////////////////
}

////////////////////////////////////////////
// Rust 的函数采用蛇形命名法(snake case): //
// 所有字母都是小写并使用下划线分隔单词。 //
////////////////////////////////////////////

fn another_function_void() {
    println!("Another function.");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5           //return 5
}

fn plus_one(x: i32) -> i32 {
    x + 1       //return x + 1
}