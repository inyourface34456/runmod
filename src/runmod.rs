use std::panic::Location;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::Display;
use rustc_lexer::unescape::unescape_str;
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
    /// use runmod::{RunMod, RunVar};
    /// 
    /// let value = RunMod::new(RunVar::I32(10));
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
    
    /// This method will only panic if the line does not have at least two quotes (it will take all text between them)
    /// It will parce escape sequences with the rust compiler, so it should be accurate (i.e. \n will become a new line, etc)
    /// This is (suprisingly not that much slower then the number ones, only 0.002ms slower)
    /// I have not tested this with utf-16, so it may not work
    #[inline(always)]
    pub fn get_string(&mut self) -> Option<String> {
        let line = BufReader::new(File::open(&self.file_name).unwrap())
            .lines()
            .nth(self.line_number as usize - 1)
            .unwrap().unwrap();
        
        if let RunVar::STRING(x) = &self.value {
            let mut num = Self::extract_runvar_arg(&line).expect(&format!("line: {}", line)).parse().unwrap_or(x.clone());
            let idx = num.rfind('"').unwrap();
            num.truncate(idx);
            let idx = num.find('"').unwrap();
            num = (&num[idx+1..]).to_string();
            let mut out = String::new();
            unescape_str(&num, &mut |_, c| out.push(c.unwrap()));
            
            if num != out {
                self.value = RunVar::STRING(out.clone());
            }
            Some(out)
        } else {
            None
        }
    }
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