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
        sec.ins(&[Lbl::Len(16)], || [0u32; 16])?;
        sec.ins(&[Lbl::Len(32)], || [0u32; 32])?;
        sec.ins(&[Lbl::Len(64)], || [0u32; 64])?;
        sec.ins(&[Lbl::Len(128)], || [0u32; 128])?;
        sec.ins(&[Lbl::Len(256)], || [0u32; 256])?;
        sec.ins(&[Lbl::Len(512)], || [0u32; 512])?;
        sec.ins(&[Lbl::Len(1024)], || [0u32; 1024])?;
        sec.ins(&[Lbl::Len(2048)], || [0u32; 2048])?;
        sec.ins(&[Lbl::Len(4096)], || [0u32; 4096])?;
        sec.ins(&[Lbl::Len(8192)], || [0u32; 8192])?;
        sec.ins(&[Lbl::Len(16384)], || [0u32; 16384])?;
        sec.ins(&[Lbl::Len(32768)], || [0u32; 32768])?;
        sec.ins(&[Lbl::Len(65536)], || [0u32; 65536])?;
        sec.ins(&[Lbl::Len(131072)], || [0u32; 131072])?;
    }
    {
        let sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz]);
        sec.ins(
            &[Lbl::Len(16)],
            || {
                let mut ret = Vec::<u32>::with_capacity(16);
                ret.resize(16, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(32)],
            || {
                let mut ret = Vec::<u32>::with_capacity(32);
                ret.resize(32, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(64)],
            || {
                let mut ret = Vec::<u32>::with_capacity(64);
                ret.resize(64, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(128)],
            || {
                let mut ret = Vec::<u32>::with_capacity(128);
                ret.resize(128, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(256)],
            || {
                let mut ret = Vec::<u32>::with_capacity(256);
                ret.resize(256, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(512)],
            || {
                let mut ret = Vec::<u32>::with_capacity(512);
                ret.resize(512, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(1024)],
            || {
                let mut ret = Vec::<u32>::with_capacity(1024);
                ret.resize(1024, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(2048)],
            || {
                let mut ret = Vec::<u32>::with_capacity(2048);
                ret.resize(2048, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(4096)],
            || {
                let mut ret = Vec::<u32>::with_capacity(4096);
                ret.resize(4096, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(8192)],
            || {
                let mut ret = Vec::<u32>::with_capacity(8192);
                ret.resize(8192, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(16384)],
            || {
                let mut ret = Vec::<u32>::with_capacity(16384);
                ret.resize(16384, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(32768)],
            || {
                let mut ret = Vec::<u32>::with_capacity(32768);
                ret.resize(32768, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(65536)],
            || {
                let mut ret = Vec::<u32>::with_capacity(65536);
                ret.resize(65536, 0);
                ret
            },
        )?;
        sec.ins(
            &[Lbl::Len(131072)],
            || {
                let mut ret = Vec::<u32>::with_capacity(131072);
                ret.resize(131072, 0);
                ret
            },
        )?;
    }
    {
        let sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr]);
        sec.ins(&[Lbl::Len(16)], || vec![0u32; 16])?;
        sec.ins(&[Lbl::Len(32)], || vec![0u32; 32])?;
        sec.ins(&[Lbl::Len(64)], || vec![0u32; 64])?;
        sec.ins(&[Lbl::Len(128)], || vec![0u32; 128])?;
        sec.ins(&[Lbl::Len(256)], || vec![0u32; 256])?;
        sec.ins(&[Lbl::Len(512)], || vec![0u32; 512])?;
        sec.ins(&[Lbl::Len(1024)], || vec![0u32; 1024])?;
        sec.ins(&[Lbl::Len(2048)], || vec![0u32; 2048])?;
        sec.ins(&[Lbl::Len(4096)], || vec![0u32; 4096])?;
        sec.ins(&[Lbl::Len(8192)], || vec![0u32; 8192])?;
        sec.ins(&[Lbl::Len(16384)], || vec![0u32; 16384])?;
        sec.ins(&[Lbl::Len(32768)], || vec![0u32; 32768])?;
        sec.ins(&[Lbl::Len(65536)], || vec![0u32; 65536])?;
        sec.ins(&[Lbl::Len(131072)], || vec![0u32; 131072])?;
    }
    Ok(ret)
}
