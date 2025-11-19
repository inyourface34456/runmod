/*!
This is a simple crate that will allow you to change values in your program and
have them update in real time when you change values in your source code.

For now only numbers types are suported (i.e., your ix, ux, and fx).  I do plan on adding
string suport at a later date if I ever get around to it.

EDIT: Strings are now suported (utf-8 only for now)

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
# How it works
Right now it uses `std::panic::Location::caller()` (an api I bet you never new exsisted) and 
the `#[track_caller]` macro (the only way this works) to get the file and location where the 
varible is made, and every time you call `.get_[type]` it reads the file, skips to the line 
with the varible decelration and uses regex to parse it.  Tis can cause panics, but only if 
you use a varible instead of a number like this:
```
use runmod::{RunMod, RunVar};

let val = 42;
let mut runvar = RunMod::new(RunVar::I32(val));
runvar.get_i32();
```

This fails due to rust parser not knowing what do to to convert a varible name into a number (i will accept
PRs that can add this functionality while maintaning speed).
*/

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
    fn get_u8() {
        let mut val = RunMod::new(RunVar::U8(42));
        assert_eq!(val.get_u8(), Some(42));
    }
    
    #[test]
    fn get_u16() {
        let mut val = RunMod::new(RunVar::U16(42));
        assert_eq!(val.get_u16(), Some(42));
    }
    
    #[test]
    fn get_u32() {
        let mut val = RunMod::new(RunVar::U32(42));
        assert_eq!(val.get_u32(), Some(42));
    }
    
    #[test]
    fn get_u64() {
        let mut val = RunMod::new(RunVar::U64(42));
        assert_eq!(val.get_u64(), Some(42));
    }
    
    #[test]
    fn get_u128() {
        let mut val = RunMod::new(RunVar::U128(42));
        assert_eq!(val.get_u128(), Some(42));
    }
    
    #[test]
    fn get_usize() {
        let mut val = RunMod::new(RunVar::USIZE(42));
        assert_eq!(val.get_usize(), Some(42));
    }
    
    #[test]
    fn get_i8() {
        let mut val = RunMod::new(RunVar::I8(42));
        assert_eq!(val.get_i8(), Some(42));
    }
    
    #[test]
    fn get_i16() {
        let mut val = RunMod::new(RunVar::I16(42));
        assert_eq!(val.get_i16(), Some(42));
    }
    
    #[test]
    fn get_i32() {
        let mut val = RunMod::new(RunVar::I32(42));
        assert_eq!(val.get_i32(), Some(42));
    }
    
    #[test]
    fn get_i64() {
        let mut val = RunMod::new(RunVar::I64(42));
        assert_eq!(val.get_i64(), Some(42));
    }
    
    #[test]
    fn get_i128() {
        let mut val = RunMod::new(RunVar::I128(42));
        assert_eq!(val.get_i128(), Some(42));
    }
    
    #[test]
    fn get_isize() {
        let mut val = RunMod::new(RunVar::ISIZE(42));
        assert_eq!(val.get_isize(), Some(42));
    }
    
    #[test]
    fn get_f64() {
        let mut val = RunMod::new(RunVar::F64(42.));
        assert_eq!(val.get_f64(), Some(42.));
    }
    
    #[test]
    fn get_f32() {
        let mut val = RunMod::new(RunVar::F32(42.));
        assert_eq!(val.get_f32(), Some(42.));
    }
    
    #[test]
    fn get_string() {
        let mut val = RunMod::new(RunVar::STRING("me when i when".into()));
        assert_eq!(val.get_int_string(), Some(String::from("me when i when")));
    }
    
    #[test]
    fn write_string() {
        let mut val = RunMod::new(RunVar::STRING("me when i when".into()));
        val.write_string("me when i dont");
        println!("{}", val);
        assert_eq!(val.get_int_string(), Some(String::from("me when i dont")));
    }
    
    #[test]
    fn write_i8() {
        let mut val = RunMod::new(RunVar::I8(1));
        val.write_i8(2);
        println!("{}", val);
        assert_eq!(val.get_int_i8(), Some(2));
    }
    
    #[test]
    fn write_i16() {
        let mut val = RunMod::new(RunVar::I16(1));
        val.write_i16(2);
        println!("{}", val);
        assert_eq!(val.get_int_i16(), Some(2));
    }
    
    #[test]
    fn write_i32() {
        let mut val = RunMod::new(RunVar::I32(1));
        val.write_i32(2);
        println!("{}", val);
        assert_eq!(val.get_int_i32(), Some(2));
    }
    
    #[test]
    fn write_i64() {
        let mut val = RunMod::new(RunVar::I64(1));
        val.write_i64(2);
        println!("{}", val);
        assert_eq!(val.get_int_i64(), Some(2));
    }
    
    #[test]
    fn write_i128() {
        let mut val = RunMod::new(RunVar::I128(1));
        val.write_i128(2);
        println!("{}", val);
        assert_eq!(val.get_int_i128(), Some(2));
    }
    
    #[test]
    fn write_isize() {
        let mut val = RunMod::new(RunVar::ISIZE(1));
        val.write_isize(2);
        println!("{}", val);
        assert_eq!(val.get_int_isize(), Some(2));
    }
    
    #[test]
    fn write_u8() {
        let mut val = RunMod::new(RunVar::U8(1));
        val.write_u8(2);
        println!("{}", val);
        assert_eq!(val.get_int_u8(), Some(2));
    }
    
    #[test]
    fn write_u16() {
        let mut val = RunMod::new(RunVar::U16(1));
        val.write_u16(2);
        println!("{}", val);
        assert_eq!(val.get_int_u16(), Some(2));
    }
    
    #[test]
    fn write_u32() {
        let mut val = RunMod::new(RunVar::U32(1));
        val.write_u32(2);
        println!("{}", val);
        assert_eq!(val.get_int_u32(), Some(2));
    }
    
    #[test]
    fn write_u64() {
        let mut val = RunMod::new(RunVar::U64(1));
        val.write_u64(2);
        println!("{}", val);
        assert_eq!(val.get_int_u64(), Some(2));
    }
    
    #[test]
    fn write_u128() {
        let mut val = RunMod::new(RunVar::U128(1));
        val.write_u128(2);
        println!("{}", val);
        assert_eq!(val.get_int_u128(), Some(2));
    }
    
    #[test]
    fn write_usize() {
        let mut val = RunMod::new(RunVar::USIZE(1));
        val.write_usize(2);
        println!("{}", val);
        assert_eq!(val.get_int_usize(), Some(2));
    }
    
    #[bench]
    fn getter(b: &mut test::Bencher) {
        let mut val = RunMod::new(RunVar::F64(100000000.));
        b.iter(|| {
            test::black_box(val.get_f32())
        });
    }
    
    #[bench]
    fn str_getter(b: &mut test::Bencher) {
        let mut val = RunMod::new(RunVar::STRING("100000000.".to_string()));
        b.iter(|| {
            test::black_box(val.get_string())
        });
    }
}