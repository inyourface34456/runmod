use runmod::{RunMod, RunVar};

fn main() {
    let mut val = RunMod::new(RunVar::F32(21.));
    let start = val.get_f32().unwrap();
    while val.get_f32().unwrap() == start {
        println!("val: {}", val.get_f32().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
