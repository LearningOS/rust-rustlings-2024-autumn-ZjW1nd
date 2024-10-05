// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在构建包时，有些依赖项既不能在 `Cargo.toml` 中导入，也不能直接链接；一些预处理过程从代码生成到设置包特定配置各不相同。
//
// Cargo 并不旨在取代其他构建工具，但它确实通过名为 `build.rs` 的自定义构建脚本与它们集成。此文件通常放置在项目的根目录中，而在本例中则放置在此练习的同一目录中。
//
// 它可以用于：
//
// - 构建捆绑的 C 库。
// - 在主机系统上查找 C 库。
// - 从规范生成 Rust 模块。
// - 执行 crate 所需的任何平台特定配置。
//
// 在设置配置时，我们可以在构建脚本中使用 `println!` 告诉 Cargo 遵循一些指令。通用格式是：
//
//     println!("cargo:{}", your_command_in_string);
//
// 有关构建脚本的更多信息，请参阅官方 Cargo 书籍：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在此练习中，我们查找一个环境变量并期望它在一个范围内。你可以查看测试用例以了解详细信息。
//
// 你不应该修改此文件。修改同一目录中的 `build.rs` 以通过此练习。
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
