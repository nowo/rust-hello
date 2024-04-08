// use super::module; // 导入父模块

pub fn fn_file_url(x: i32) {
    // super::size::fn_file_url(); // 调用父模块的函数
    println!("fn_file_url({:#?}); /src/foo/size.rs", x);
}
