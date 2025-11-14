#![feature(test)]
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
    extern crate test;

    #[test]
    fn f32_test() {
        let mut val = RunMod::new(RunVar::I8(2));
        let _ = RunMod::new(10.into());
        while val.get_i8().unwrap() == 1 {
            println!("val: {}", val.get_i8().unwrap());
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
    }
    
    #[bench]
    fn getter(b: &mut test::Bencher) {
        let mut val = RunMod::new(RunVar::F64(100000000.));
        b.iter(|| {
            test::black_box(val.get_f32())
        });
    }
}
