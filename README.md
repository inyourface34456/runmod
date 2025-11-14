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