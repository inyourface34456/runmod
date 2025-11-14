/*!
This is a simple crate that will allow you to change values in your program and
have them update in real time when you change values in your source code.

For now only number5 types are suported (i.e., your ix, ux, and fx).  I do plan on adding
string suport at a later date if I ever get around to it.

Here is a basic usage example:
```
use runmod::{RunMod, RunVar};

fn main() {
    let mut val = RunMod::new(RunVar::I32(42));
    while val.get_i32().unwrap() == 42 {
        println!("val: {}", val.get_i32().unwrap());
        // this will run untill you change the value in the program
        break // if this is not here, then rustdoc will not exit
    }
}
```
*/

// #![feature(test)]
#[macro_use]
mod macros;
mod runvar;
mod runmod;

use std::sync::LazyLock;
use regex::Regex;
pub use runvar::RunVar;
pub use runmod::RunMod;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"RunVar::[A-Za-z_][A-Za-z0-9_]*\(\s*([^\)]+)\s*\)").unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;
    // extern crate test;

    #[test]
    fn f32_test() {
        let mut val = RunMod::new(RunVar::I8(2));
        while val.get_i8().unwrap() == 1 {
            println!("val: {}", val.get_i8().unwrap());
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
    }
    
    // #[bench]
    // fn getter(b: &mut test::Bencher) {
    //     let mut val = RunMod::new(RunVar::F64(100000000.));
    //     b.iter(|| {
    //         test::black_box(val.get_f32())
    //     });
    // }
}
