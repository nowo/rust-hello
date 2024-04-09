//! # nowo
//!
//! 用于学习rust，以及crate包发布
//! 
//! 
//! 

pub mod foo {
    pub mod mode;
    pub mod size;
}

pub use crate::{
    foo::{
        mode,
        size::{self as FooSize, fn_file_url},
    },
    // module::test,
};

/// 两数相加方法
///
/// # 例子
///
/// **提示：** 运行`cargo test`会执行下面这段代码进行测试
/// ```rust
/// let arg = 5;
/// let answer = nowo::add(1, arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//         assert_ne!(result, 5);
//     }


// }
