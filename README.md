This is a simple crate that will allow you to change values in your program and
have them update in real time when you change values in your source code.

For now only numbers types are suported (i.e., your ix, ux, and fx).  I do plan on adding
string suport at a later date if I ever get around to it.

EDIT: Strings are now suported (utf-8 only for now)

Here is a basic usage example:
```rust
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
```rust
use runmod::{RunMod, RunVar};

let val = 42;
let mut runvar = RunMod::new(RunVar::I32(val));
runvar.get_i32();
```

This fails due to lexical parser not knowing what do to to convert a string into a number (i will accept
PRs that can add this functionality while maintaning speed).