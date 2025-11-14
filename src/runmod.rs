use std::panic::Location;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::Display;
use crate::RunVar;
use crate::RE;

#[derive(Clone)]
pub struct RunMod {
    value: RunVar,
    file_name: String,
    line_number: u32,
}

impl RunMod {
    ///Makes a new RunMod
    /// 
    /// This method will never panic or fail
    /// 
    /// # Examples
    /// 
    /// ```
    /// use runmod::{RunMod, RunVer}
    /// 
    /// let value = RunMod::new(RunVar::I32(10));
    /// let value2 = RunMod::new(10.into());
    /// ```
    #[track_caller]
    pub fn new(val: RunVar) -> Self {
        let loc = Location::caller();

        Self {
            value: val,
            file_name: loc.file().to_string(),
            line_number: loc.line(),
        }
    }
    
    
    fn extract_runvar_arg(line: &str) -> Option<String> {
        RE.captures(line).map(|caps| caps[1].trim().to_string())
    }
    
    make_runner!(get_i8, I8, i8);
    make_runner!(get_i16, I16, i16);
    make_runner!(get_i32, I32, i32);
    make_runner!(get_i64, I64, i64);
    make_runner!(get_i128, I128, i128);
    make_runner!(get_isize, ISIZE, isize);
    make_runner!(get_u8, U8, u8);
    make_runner!(get_u16, U16, u16);
    make_runner!(get_u32, U32, u32);
    make_runner!(get_u64, U64, u64);
    make_runner!(get_u128, U128, u128);
    make_runner!(get_usize, USIZE, usize);
    make_runner!(get_f32, F32, f32);
    make_runner!(get_f64, F64, f64);
}

impl From<RunVar> for RunMod {
    #[track_caller]
    fn from(value: RunVar) -> Self {
        let loc = Location::caller();
        
        Self {
            value,
            line_number: loc.line(),
            file_name: loc.file().to_string(),
        }
    }
}

impl Display for RunMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}