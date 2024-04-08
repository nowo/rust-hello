// mod module {
//     pub mod test; // 内部模块
//     pub mod variable; // 内部模块
// }
// use crate::module::{test::plus_one as p_one, variable};
// use module::test::plus_one;

mod foo {
    pub mod mode;
    pub mod size;
}

// use crate::foo::mode::ARange;
// // use foo::size::fn_file_url;

// use foo::size::{self as FooSize, fn_file_url};

use crate::{
    foo::{mode,size::{self as FooSize, fn_file_url}},
    // module::test,
};

fn main() {

    println!("Hello, world!");
    let a = mode::ARange::new(100);
    println!("a=> {:#?}", a);
    fn_file_url(0);
    foo::size::fn_file_url(1);
    FooSize::fn_file_url(2);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        print!("{:?} ", i);
        *i += 50;
    }
    println!("v=> {:#?}", v);


}
