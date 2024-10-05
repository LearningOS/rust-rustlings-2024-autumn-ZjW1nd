// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
// Rust 中的 `unsafe` 作为一种契约。

// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
// 当 `unsafe` 标记在一个项目声明上时，比如一个函数、一个 trait 等，
// 它声明了一个契约。然而，契约的内容不能仅通过一个单一的关键字来表达。
// 因此，你有责任在项目的文档注释中的 `# Safety` 部分手动说明它。

// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
// 当 `unsafe` 标记在由大括号包围的代码块上时，
// 它声明了对某些契约的遵守，比如某些指针参数的有效性，某些内存地址的所有权。
// 然而，和上面的文字一样，你仍然需要在代码块的注释中说明如何遵守契约。

// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
// 注意：所有的注释都是为了代码的可读性和可维护性，
// 而 Rust 编译器将代码健全性的信任交给你自己！
// 如果你不能证明自己代码的内存安全性和健全性，请退一步，使用安全代码代替！
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        //todo!("Your code goes here")
        // SAFETY : balabalabala
        let p = address as *mut u32;
        *p = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
