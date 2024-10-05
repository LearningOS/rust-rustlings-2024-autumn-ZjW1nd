// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");//挪到这里也行
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }// drop string2
    println!("The longest string is '{}'", result);//注释掉这一行可以通过
}// 这段代码的编译报错就是生命周期的意义
