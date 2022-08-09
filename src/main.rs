fn main() {
  // 1.定义一个引用：
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("1.The length of '{}' is {}.", s1, len);

  // 2.为可变变量定义一个可变引用：
  let mut s = String::from("hello");

  //change(&s1);
  change(&mut s);

  println!("2.{s}");

  // 3.数据竞争（data race）：
  let mut s = String::from("hello");

  // 可变引用的作用域只能出现它自己。
  // let r1 = &mut s;// 第一处可变引用
  let r1 = &s;
  // let r2 = &mut s;// 第二处可变引用
  let r2 = &s;
  //let r3=&mut s;// 第三处可变引用
  println!("3.1.{}, {}", r1, r2);
  // 此位置之后 r1 和 r2 不再使用

  // 的作用域在 println! 最后一次使用之后结束。
  // 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）：
  //  编译器在作用域结束之前判断不再使用的引用的能力。
  //  这里编译器认为 r1 和 r2 不再用于读取变量 s
  let r3=&mut s;
  println!("3.2.{}", r3);

  // 4.悬垂引用（Dangling References）：
  // let reference_to_nothing = dangle();
  let reference_to_nothing = no_dangle();
  println!("4.{reference_to_nothing}");

}

fn calculate_length(s: &String) -> usize {
  s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }//正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
// 
//   let s = String::from("hello"); // s 是一个新字符串
// 
//   &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
// // 危险！
fn no_dangle() -> String {
  let s = String::from("hello");

  s
}

//////////////////////////////////////////////////////////////////////////////
// 引用与借用：                                                             //
// 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该  //
//  地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。//
// 引用（默认）不允许修改引用的值。                                         //
// 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：              //
//  1.两个或更多指针同时访问同一数据。                                      //
//  2.至少有一个指针被用来写入数据。                                        //
//  3.没有同步数据访问的机制。                                              //
// 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）：                //
//  编译器在作用域结束之前判断不再使用的引用的能力。                        //
// 悬垂指针（dangling pointer）：                                           //
//  在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一  //
//  个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分 //
//  配给其它持有者。                                                        //
//////////////////////////////////////////////////////////////////////////////