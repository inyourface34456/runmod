use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum RunVar {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    USIZE(usize),
    ISIZE(isize)
}

impl Default for RunVar {
    fn default() -> Self {
        Self::I32(0)
    }
}

// from RunVar to number
try_into_impl!(i8, I8);
try_into_impl!(i16, I16);
try_into_impl!(i32, I32);
try_into_impl!(i64, I64);
try_into_impl!(i128, I128);
try_into_impl!(isize, ISIZE);
try_into_impl!(u8, U8);
try_into_impl!(u16, U16);
try_into_impl!(u32, U32);
try_into_impl!(u64, U64);
try_into_impl!(u128, U128);
try_into_impl!(usize, USIZE);
try_into_impl!(f32, F32);
try_into_impl!(f64, F64);

// from number to RunVar
// from_impl!(i8, I8);
// from_impl!(i16, I16);
// from_impl!(i32, I32);
// from_impl!(i64, I64);
// from_impl!(i128, I128);
// from_impl!(isize, ISIZE);
// from_impl!(u8, U8);
// from_impl!(u16, U16);
// from_impl!(u32, U32);
// from_impl!(u64, U64);
// from_impl!(u128, U128);
// from_impl!(usize, USIZE);
// from_impl!(f32, F32);
// from_impl!(f64, F64);

impl Display for RunVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(x) => write!(f, "{}", x),
            Self::I16(x) => write!(f, "{}", x),
            Self::I32(x) => write!(f, "{}", x),
            Self::I64(x) => write!(f, "{}", x),
            Self::I128(x) => write!(f, "{}", x),
            Self::U8(x) => write!(f, "{}", x),
            Self::U16(x) => write!(f, "{}", x),
            Self::U32(x) => write!(f, "{}", x),
            Self::U64(x) => write!(f, "{}", x),
            Self::U128(x) => write!(f, "{}", x),
            Self::F32(x) => write!(f, "{}", x),
            Self::F64(x) => write!(f, "{}", x),
            Self::USIZE(x) => write!(f, "{}", x),
            Self::ISIZE(x) => write!(f, "{}", x)
        }
    }
}