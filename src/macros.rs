macro_rules! make_runner {
    ($func:ident, $varient: ident, $type:ty) => {
        /// Reads the source file andf gets a new value.
        /// I recommend that you disable autosave on your
        /// text editor/IDE (especily if it is high frequency),
        /// as this can cause unexpected panics.
        /// 
        /// This will update every time you call it, witch means
        /// it will open the file, read it, parse the line, then close it.
        /// 
        /// When built in release mode (this only happened when debug 
        /// assertions are disabled), this will not open and read the
        /// file, and instead, this will just return the internel value, 
        /// making it as fast as possible. 
        #[inline(always)]
        pub fn $func(&mut self) -> Option<$type> {
            #[cfg(not(debug_assertions))]
            if let RunVar::$varient(x) = self.value {
                return Some(x)
            } else {
                return None
            }
            
            if let RunVar::$varient(x) = self.value {
                let line = BufReader::new(File::open(&self.file_name).unwrap())
                    .lines()
                    .nth(self.line_number as usize - 1)
                    .unwrap().unwrap();
                    
                let num = Self::extract_runvar_arg(&line).expect(&format!("line: {}", line)).parse().unwrap_or(x);
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

macro_rules! make_writer {
    ($func:ident, $varient: ident, $type:ty) => {
        ///Takes an `&mut RunMod`, and writes to the value inside only if you
        /// have the right type (indacted by the return value)
        pub fn $func(&mut self, new_val: $type) -> Option<$type> {
            if let RunVar::$varient(mut _x) = self.value {
                _x = new_val;
                Some(new_val)
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
        /// use runmod::RunVar;
        /// 
        /// let val = RunVar::I32(10);
        /// assert_eq!(TryInto::<i64>::try_into(val).is_err(), true);
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

// macro_rules! from_impl {
//     ($type:ty, $varient:ident) => {
//         impl From<$type> for RunVar {
//             fn from(value: $type) -> Self {
//                 Self::$varient(value)
//             }
//         }
//     };
// }