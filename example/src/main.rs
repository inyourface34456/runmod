use runmod::{RunMod, RunVar};

fn main() {
    let mut val = RunMod::new(RunVar::F32(35.));
    let start = val.get_f32().unwrap();
    while val.get_f32().unwrap() == start {
        println!("val: {}", val.get_f32().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(16));
        break
    }
    let mut val2 = RunMod::new(RunVar::STRING("hewwo\"".into()));
    println!("{}", val2.get_string().unwrap());
}
