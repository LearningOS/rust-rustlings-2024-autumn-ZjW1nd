// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
// 宏要展开，要定义在使用它的地方前面

fn main() {
    my_macro!();
}