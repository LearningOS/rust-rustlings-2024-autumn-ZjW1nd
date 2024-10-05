// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// Rust 非常擅长与 C/C++ 以及其他静态编译语言共享 FFI 接口，
// languages, and it can even link within the code itself! It makes it through the extern
// 它甚至可以在代码内部进行链接！通过 extern 关键字实现这一点，
// block, just like the code below.
// 就像下面的代码一样。
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// `extern` 关键字后的短字符串表示外部导入函数将遵循的 ABI，
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// 在这个练习中，使用的是 "Rust"，而其他变体如 "C" 表示标准 C ABI，"stdcall" 表示 Windows ABI。
//"C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// 外部导入的函数在 extern 块中声明，以分号结束签名而不是花括号。
//mark the end of signature instead of curly braces. Some attributes can be applied to those
// 可以对这些函数声明应用一些属性来修改链接行为，例如 #[link_name = ".."] 来修改实际的符号名称。
//function declarations to modify the linking behavior, such as #[link_name = ".."] to
// 
//modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// 如果你想将符号导出到链接环境，可以在函数定义前加上 `extern` 关键字，并带有相同的 ABI 字符串。
//also be marked before a function definition with the same ABI string note. The default ABI
// Rust 函数的默认 ABI 实际上是 "Rust"，所以如果你想链接纯 Rust 函数，可以省略整个 extern 术语。
//for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// 
//the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// Rust 默认会对符号进行重整，就像 C++ 一样。要抑制这种行为并使这些函数可以通过名称访问，
//those functions addressable by name, the attribute #[no_mangle] can be applied.
// 可以应用属性 #[no_mangle]。
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// 在这个练习中，你的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。
//module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// `my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用相同的函数。
//line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}


mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
