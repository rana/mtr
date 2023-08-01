#![allow(clippy::slow_vector_initialization)]
use core::fmt;
use core::hash::Hash;
use core::str;
use crate::ben::*;
/// Benchmark labels.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Lbl {
    Alc,
    Arr,
    Asc,
    Cap,
    Dsc,
    Lop,
    Mat,
    Mcr,
    Mdn,
    #[default]
    Raw,
    Rd,
    Rsz,
    Ser,
    Unr,
    Vec,
    Wrt,
    Len(u32),
    Prm(u32),
}
impl fmt::Display for Lbl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lbl::Alc => write!(f, "alc"),
            Lbl::Arr => write!(f, "arr"),
            Lbl::Asc => write!(f, "asc"),
            Lbl::Cap => write!(f, "cap"),
            Lbl::Dsc => write!(f, "dsc"),
            Lbl::Lop => write!(f, "lop"),
            Lbl::Mat => write!(f, "mat"),
            Lbl::Mcr => write!(f, "mcr"),
            Lbl::Mdn => write!(f, "mdn"),
            Lbl::Raw => write!(f, "raw"),
            Lbl::Rd => write!(f, "rd"),
            Lbl::Rsz => write!(f, "rsz"),
            Lbl::Ser => write!(f, "ser"),
            Lbl::Unr => write!(f, "unr"),
            Lbl::Vec => write!(f, "vec"),
            Lbl::Wrt => write!(f, "wrt"),
            Lbl::Len(x) => {
                if f.alternate() { write!(f, "len") } else { write!(f, "len({})", x) }
            }
            Lbl::Prm(x) => {
                if f.alternate() { write!(f, "prm") } else { write!(f, "prm({})", x) }
            }
        }
    }
}
impl str::FromStr for Lbl {
    type Err = String;
    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "alc" => Ok(Lbl::Alc),
            "arr" => Ok(Lbl::Arr),
            "asc" => Ok(Lbl::Asc),
            "cap" => Ok(Lbl::Cap),
            "dsc" => Ok(Lbl::Dsc),
            "lop" => Ok(Lbl::Lop),
            "mat" => Ok(Lbl::Mat),
            "mcr" => Ok(Lbl::Mcr),
            "mdn" => Ok(Lbl::Mdn),
            "raw" => Ok(Lbl::Raw),
            "rd" => Ok(Lbl::Rd),
            "rsz" => Ok(Lbl::Rsz),
            "ser" => Ok(Lbl::Ser),
            "unr" => Ok(Lbl::Unr),
            "vec" => Ok(Lbl::Vec),
            "wrt" => Ok(Lbl::Wrt),
            "len" => Ok(Lbl::Len(0)),
            "prm" => Ok(Lbl::Prm(0)),
            _ => Err(format!("invalid Lbl: {s}")),
        }
    }
}
impl EnumStructVal for Lbl {
    fn val(&self) -> Result<u64> {
        match *self {
            Lbl::Len(x) => Ok(x as u64),
            Lbl::Prm(x) => Ok(x as u64),
            _ => Err("label doesn't have a struct value".to_string()),
        }
    }
}
impl Label for Lbl {}
/// Returns a populated set of `mtr` benchmark functions.
pub fn new_mtr_set() -> Result<Set<Lbl>> {
    let ret = Set::new();
    {
        let sec = ret.sec(&[Lbl::Alc, Lbl::Arr]);
        sec.ins(&[Lbl::Len(4)], || [0u32; 4])?;
        sec.ins(&[Lbl::Len(8)], || [0u32; 8])?;
        sec.ins(&[Lbl::Len(16)], || [0u32; 16])?;
    }
    {
        let sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz]);
        sec.ins(
            &[Lbl::Len(4)],
            || {
                let mut ret = Vec::<u32>::with_capacity(4);
                ret.resize(4, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(8)],
            || {
                let mut ret = Vec::<u32>::with_capacity(8);
                ret.resize(8, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(16)],
            || {
                let mut ret = Vec::<u32>::with_capacity(16);
                ret.resize(16, 0);
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr]);
        sec.ins(&[Lbl::Len(4)], || vec![0u32; 4])?;
        sec.ins(&[Lbl::Len(8)], || vec![0u32; 8])?;
        sec.ins(&[Lbl::Len(16)], || vec![0u32; 16])?;
    }
    Ok(ret)
}
