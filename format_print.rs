

// https://rust-book.junmajinlong.com/


/*
 * Rust是基于表达式的语言，几乎所有代码都可以看作是表达式。
 * 在Rust中，可以在表达式结尾加上分号;来将表达式转换为【语句】
 * 表达式有返回值，语句没有返回值或者不关心其返回值
 * 语句 -> 用于声明或定义的代码都是语句
 *   let声明变量、fn定义函数、struct声明结构体等
 */


/*
 * fn关键字、函数名、函数参数及其类型、返回值类型组成函数签名。例如fn fname(a: i32, b: i32)->i32是一个函数签名。
 */
fn lqs(a: i32,b: i32) -> i32{
    a + b
}

fn main(){
    let num = 3 + 4;
    println!("{}",num);

    let x = if true{
        println!("true");
        4
    }else{
        println!("false");
        3
    };
    println!("{}",x);

    let name = "jumin";
    println!("{}",name);


    let _male = "lqs"; // 同名变量被遮盖，内存地址不同
    println!("{}",_male);
    let _male = "yz";
    println!("{}",_male);





    /*
     * 变量初始化后，默认不允许再修改该变量
     * 想要修改变量的值，需要在声明变量时加上mut标记(mutable)表示该变量是可修改的
     */
    let mut gen = "lqs";
    println!("{}", gen);
    gen = "eyu";
    println!("{}", gen);

    /*
     * Rust是静态语言，声明变量时需指定该变量将要保存的值的数据类型，这样编译器编译时才知道为该变量将要保存的数据分配多少内存、允许存放什么类型的数据以及如何存放数据。但Rust编译器会根据所保存的值来推导变量的数据类型，推导得到确定的数据类型之后(比如第一次为该变量赋值之后)，就不再允许存放其他类型的数据
     */

    /*
     * 当Rust无法推导类型时，或者声明变量时就明确知道该变量要保存声明类型的数据时，可明确指定该变量的数据类型
     */
    let age: i32 = 32;
    println!("{:.}",age);

    /*
     *可以使用tuple的方式同时为多个变量赋值，并且可以使用下划线_占位表示忽略某个变量的赋值过程
     */
    let (x,y, _) = (1,2,3);
    println!("{}",x);

    let s1 = "juminfo";
    let s2 = s1;
    println!("{}",s1);

    let ret = lqs(2,3);
    println!("{}", ret);
}



