macro_rules! make_runner {
    ($func:ident, $varient: ident, $type:ty) => {
        /// Reads the source file andf gets a new value.
        /// I recommend that you disable autosave on your
        /// text editor/IDE (especily if it is high frequency),
        /// as this can cause unexpected panics.
        #[inline(always)]
        pub fn $func(&mut self) -> Option<$type> {
            let line = BufReader::new(File::open(&self.file_name).unwrap())
                .lines()
                .nth(self.line_number as usize - 1)
                .unwrap().unwrap();
            
            if let RunVar::$varient(x) = self.value {
                let num = lexical_core::parse(Self::extract_runvar_arg(&line).expect(&format!("line: {}", line)).as_bytes()).unwrap_or(x);
                if num != x {
                    self.value = RunVar::$varient(num);
                }
                Some(num)
            } else {
                None
            }
        }
    };
}

macro_rules! try_into_impl {
    ($type:ty, $varient:ident) => {
        ///the only time that this will fail is when doing something like the following:
        /// 
        /// ```
        /// let val = RunVar::I32(10);
        /// let val2: u32 = val.try_into().unwrap()
        /// ```
        impl TryInto<$type> for RunVar {
            type Error = ();
            
            fn try_into(self) -> Result<$type, Self::Error> {
                if let Self::$varient(x) = self {
                    Ok(x)
                } else {
                    Err(())
                }
            }
        }
    };
}

macro_rules! from_impl {
    ($type:ty, $varient:ident) => {
        impl From<$type> for RunVar {
            fn from(value: $type) -> Self {
                Self::$varient(value)
            }
        }
    };
}