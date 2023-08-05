use convert_case::{self, Case, Casing};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rand::Rng;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

/// Runs the build script.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    write_all_files("./src/")
}

/// Writes all files to a directory.
pub fn write_all_files(dir: &str) -> std::io::Result<()> {
    let pth = Path::new(dir);
    fs::create_dir_all(pth)?;

    write_one_fle(emit_main_fle(), &pth.join("main.rs"))?;
    write_one_fle(emit_bens_fle(), &pth.join("bens.rs"))?;

    Ok(())
}

/// Writes a token stream to a file.
pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(fle_pth, fmt)
}

/// Emits a token stream for the main file.
pub fn emit_main_fle() -> TokenStream {
    let tok_fns = [emit_main_fn];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` function.
pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(dead_code)]
        mod bens;
        use ben::*;
        use bens::*;
        pub fn main() -> Result<()> {
            Cli::prs_and_qry(new_mtr_set()?)?;
            Ok(())
        }
    });

    stm
}

/// Emits a token stream for the `bens` module.
pub fn emit_bens_fle() -> TokenStream {
    let tok_fns = [
        emit_bens_imports,
        emit_bens_lbl_enum,
        emit_bens_lbl_impl_display,
        emit_bens_lbl_impl_fromstr,
        emit_bens_lbl_impl_enumstructval,
        emit_bens_lbl_impl_label,
        emit_bens_new_mtr_set,
    ];
    let ret = tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    });

    ret
}

/// Emits a token stream for the `main` imports.
pub fn emit_bens_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(clippy::slow_vector_initialization)]
        #![allow(clippy::needless_range_loop)]
        use ben::*;
        use core::fmt;
        use core::hash::Hash;
        use core::str;
        use rand::seq::SliceRandom;
        use rand::thread_rng;
    });

    stm
}

/// Returns label strings for all enum cases.
pub fn lbl_strs_all() -> Vec<&'static str> {
    let mut ret = lbl_strs_plain();
    ret.extend(lbl_strs_struct_u32());
    ret
}

/// Returns label strings mapping to a plain enum cases.
pub fn lbl_strs_plain() -> Vec<&'static str> {
    vec![
        "acm", "add", "alc", "arr", "chk", "cnt", "cst", "idx", "into_itr", "itr", "lop", "mat",
        "mcr", "none", "one", "ptr", "raw", "rnd", "rd", "rsz", "seq", "u8", "unchk", "usize",
        "val",  "vec",
    ]
}
/// Returns label strings which map to struct u32 cases of an enum.
pub fn lbl_strs_struct_u32() -> Vec<&'static str> {
    vec!["len", "unr", "var"]
}
pub const LBL_NAM: &str = "Lbl";
pub const LBL_VAL_DFLT: &str = "raw";

/// Emits a token stream for the `Lbl` enum.
pub fn emit_bens_lbl_enum() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    // enum: start
    stm.extend(quote! {
        /// Benchmark labels.
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
        pub enum #idn_lbl
    });

    // enum: inner
    let mut stm_inr = TokenStream::new();

    for lbl_str in lbl_strs_plain() {
        if lbl_str == LBL_VAL_DFLT {
            stm_inr.extend(quote! {
                #[default]
            });
        }
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        stm_inr.extend(quote! {
            #idn,
        });
    }
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        stm_inr.extend(quote! {
            #idn(u32),
        });
    }

    // enum: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for `Lbl` implementing `Display`.
pub fn emit_bens_lbl_impl_display() -> TokenStream {
    let mut stm_0 = TokenStream::new();
    let mut stm_1 = TokenStream::new();
    let mut stm_2 = TokenStream::new();
    let mut stm_3 = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm_0.extend(quote! { impl fmt::Display for #idn_lbl });
    stm_1.extend(quote! { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result });
    stm_2.extend(quote! { match *self });
    for lbl_str in lbl_strs_plain() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        let lit = Literal::string(lbl_str);
        stm_3.extend(quote! {
            #idn_lbl::#idn => write!(f, #lit),
        });
    }
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        let mut tmp = String::from(lbl_str);
        tmp.push_str("({})");
        let lit = Literal::string(tmp.as_str());
        let lit_alt = Literal::string(lbl_str);
        stm_3.extend(quote! {
            #idn_lbl::#idn(x) => {
                if f.alternate() {
                    write!(f, #lit_alt)
                } else {
                    write!(f, #lit, x)
                }
            },
        });
    }
    stm_2.extend(quote! {
        {
            #stm_3
        }
    });
    stm_1.extend(quote! {
        {
            #stm_2
        }
    });
    stm_0.extend(quote! {
        {
            #stm_1
        }
    });

    stm_0
}

/// Emits a token stream for `Lbl` implementing `FromStr`.
pub fn emit_bens_lbl_impl_fromstr() -> TokenStream {
    let mut stm_0 = TokenStream::new();
    let mut stm_1 = TokenStream::new();
    let mut stm_2 = TokenStream::new();
    let mut stm_3 = TokenStream::new();
    let mut stm_3_inr = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm_0.extend(quote! { impl str::FromStr for #idn_lbl });
    stm_1.extend(quote! { fn from_str(s: &str) -> Result<Self> });
    stm_2.extend(quote! { match s.as_str() });
    for lbl_str in lbl_strs_plain() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        let lit = Literal::string(lbl_str);
        stm_3.extend(quote! {
            #lit => Ok(#idn_lbl::#idn),
        });
    }
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        let lit = Literal::string(lbl_str);
        stm_3.extend(quote! {
            #lit => Ok(#idn_lbl::#idn(0)),
        });
        stm_3_inr.extend(quote! {
            #lit => Ok(#idn_lbl::#idn(v)),
        });
    }
    stm_3.extend(quote! {
        _ => {
            match s.find('[') {
                None => Err(format!("invalid Lbl: {}", s)),
                Some(idx) => {
                    // Parse the struct u32 value.
                    let v_str = &s[idx+1..s.len()-1];
                    match v_str.parse::<u32>() {
                        Err(e) => Err(format!("invalid Lbl: {}; {}", s, e)),
                        Ok(v) => {
                            let s2 = &s[..idx];
                            match s2 {
                                #stm_3_inr
                                _ => Err(format!("invalid Lbl: {}; {}", s, s2)),
                            }
                        },
                    }
                }
            }
        }
    });
    stm_2.extend(quote! {
        {

            #stm_3
        }
    });
    stm_1.extend(quote! {
        {
            let s = s.trim().to_lowercase();
            #stm_2
        }
    });
    stm_0.extend(quote! {
        {
            type Err = String;
            #stm_1
        }
    });

    stm_0
}

/// Emits a token stream for `Lbl` implementing `EnumStructVal`.
pub fn emit_bens_lbl_impl_enumstructval() -> TokenStream {
    let mut stm_0 = TokenStream::new();
    let mut stm_1 = TokenStream::new();
    let mut stm_2 = TokenStream::new();
    let mut stm_3 = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm_0.extend(quote! { impl EnumStructVal for #idn_lbl });
    stm_1.extend(quote! { fn val(&self) -> Result<u32> });
    stm_2.extend(quote! { match *self });
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        stm_3.extend(quote! {
            #idn_lbl::#idn(x) => Ok(x),
        });
    }
    stm_3.extend(quote! { _ => Err("label doesn't have a struct value".to_string()), });
    stm_2.extend(quote! {
        {
            #stm_3
        }
    });
    stm_1.extend(quote! {
        {
            #stm_2
        }
    });
    stm_0.extend(quote! {
        {
            #stm_1
        }
    });

    stm_0
}

/// Emits a token stream for `Lbl` implementing `Label`.
pub fn emit_bens_lbl_impl_label() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm.extend(quote! {
        impl Label for #idn_lbl { }
    });

    stm
}

/// Emits a token stream for the `new_mtr_set` function.
pub fn emit_bens_new_mtr_set() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    // fn: start
    stm.extend(quote! {
        /// Returns a populated set of `mtr` benchmark functions.
        pub fn new_mtr_set() -> Result<Set<#idn_lbl>>
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let tok_bens = [
        emit_bens_alc_arr,
        emit_bens_alc_vec_rsz,
        emit_bens_alc_vec_mcr,
        emit_bens_rd_arr_seq,
        emit_bens_rd_mat_seq,
        emit_bens_rd_arr_rnd,
        emit_bens_rd_mat_rnd,
        emit_bens_lop_idx_chk,
        emit_bens_lop_idx_unchk,
        emit_bens_lop_itr,
        emit_bens_lop_into_itr,
        emit_bens_cst_u8,
        emit_bens_cst_usize,
        emit_bens_acm_rd_ptr,
        emit_bens_acm_rd_val,
        emit_bens_acm_add_cnt,
        emit_bens_acm_add_one,
        emit_bens_acm_unr_0,
        emit_bens_acm_unr_8_var_1,
        emit_bens_acm_unr_8_var_8,
        emit_bens_acm_unr_16_var_16,
    ];
    tok_bens
        .iter()
        .for_each(|tok_ben| stm_inr.extend(tok_ben()));

    // fn: end
    stm.extend(quote! {
        {
            let ret = Set::new();
            #stm_inr
            Ok(ret)
        }
    });

    stm
}

pub static ALC_RNG: Range<u32> = 4..18;

/// Emits a token stream for the `alc_arr` statements.
pub fn emit_bens_alc_arr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Alc, Lbl::Arr]);
    });
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins(&[Lbl::Len(#lit_len)], || [0u32; #lit_len])?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `alc_vec_rsz` statements.
pub fn emit_bens_alc_vec_rsz() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz]);
    });
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins(&[Lbl::Len(#lit_len)], || {
                let mut ret = Vec::<u32>::with_capacity(#lit_len);
                ret.resize(#lit_len, 0);
                ret
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `alc_vec_mcr` statements.
pub fn emit_bens_alc_vec_mcr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr]);
    });
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins(&[Lbl::Len(#lit_len)], || vec![0u32; #lit_len])?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub static RD_RNG: Range<u32> = 4..12;

/// Emits a token stream for the `rd_arr_seq` statements.
pub fn emit_bens_rd_arr_seq() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Rd, Lbl::Arr, Lbl::Seq]);
    });
    let mut rng = rand::thread_rng();
    for len in RD_RNG.clone().map(|x| 2u32.pow(x)) {
        // Create an array with random elements.
        let mut stm_arr = TokenStream::new();
        for _ in 0..len {
            let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
            stm_arr.extend(quote! { #lit_ret_n, });
        }

        // Read each element from an array in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins(&[Lbl::Len(#lit_len)], || {
                let arr = [#stm_arr];
                let mut ret = [0u32; 1];
                for idx in 0..#lit_len {
                    ret[0] = arr[idx];
                }
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `rd_mat_seq` statements.
pub fn emit_bens_rd_mat_seq() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Rd, Lbl::Mat, Lbl::Seq]);
    });
    let mut rng = rand::thread_rng();
    for len in RD_RNG.clone().map(|x| 2u32.pow(x)) {
        // Create match arms which return a random u32.
        let mut stm_arm = TokenStream::new();
        for idx in 0..len {
            let lit_idx = Literal::u32_unsuffixed(idx);
            let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
            stm_arm.extend(quote! {
                #lit_idx => #lit_ret_n,
            });
        }

        // Read each element from a match in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins(&[Lbl::Len(#lit_len)], || {
                let mut ret = [0u32; 1];
                for idx in 0..#lit_len {
                    ret[0] = match idx {
                        #stm_arm
                        _ => panic!("uh oh, no no: beyond the match limit"),
                    }
                }
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `rd_arr_rnd` statements.
pub fn emit_bens_rd_arr_rnd() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Rd, Lbl::Arr, Lbl::Rnd]);

    });
    let mut rng = rand::thread_rng();
    for len in RD_RNG.clone().map(|x| 2u32.pow(x)) {
        // Create an array with random elements.
        let mut stm_arr = TokenStream::new();
        for _ in 0..len {
            let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
            stm_arr.extend(quote! { #lit_ret_n, });
        }

        // Read each element from an array in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let arr = [#stm_arr];
                let mut idxs: Vec<usize> = (0..#lit_len).collect();
                let mut rng = thread_rng();
                idxs.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                for idx in idxs {
                    ret[0] = arr[idx];
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `rd_mat_rnd` statements.
pub fn emit_bens_rd_mat_rnd() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Rd, Lbl::Mat, Lbl::Rnd]);
    });
    let mut rng = rand::thread_rng();
    for len in RD_RNG.clone().map(|x| 2u32.pow(x)) {
        // Create match arms which return a random u32.
        let mut stm_arm = TokenStream::new();
        for idx in 0..len {
            let lit_idx = Literal::u32_unsuffixed(idx);
            let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
            stm_arm.extend(quote! {
                #lit_idx => #lit_ret_n,
            });
        }

        // Read each element from a match in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut idxs: Vec<usize> = (0..#lit_len).collect();
                let mut rng = thread_rng();
                idxs.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                for idx in idxs {
                    ret[0] = match idx {
                        #stm_arm
                        _ => panic!("uh oh, no no: beyond the match limit"),
                    }
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub static LOP_RNG: Range<u32> = 4..18;

/// Emits a token stream for the `lop_idx_chk` statements.
pub fn emit_bens_lop_idx_chk() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Idx, Lbl::Chk]);
    });
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                for idx in 0..#lit_len {
                    ret[0] = vals[idx];
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `lop_idx_unchk` statements.
pub fn emit_bens_lop_idx_unchk() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Idx, Lbl::Unchk]);
    });
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                unsafe {
                    for idx in 0..#lit_len {
                        ret[0] = *vals.get_unchecked(idx);
                    }
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `lop_itr` statements.
pub fn emit_bens_lop_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Itr]);
    });
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    ret[0] = *val;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `lop_itr` statements.
pub fn emit_bens_lop_into_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::IntoItr]);
    });
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                tme.borrow_mut().start();
                for val in vals.into_iter() {
                    ret[0] = val;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub static CST_RNG: Range<u32> = 4..18;

/// Emits a token stream for the `cst_u8` statements.
pub fn emit_bens_cst_u8() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Cst, Lbl::U8]);
    });
    for len in CST_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    ret[0] += ((*val > 0xFF) as u8 + (*val > 0xFFFF) as u8 + (*val > 0xFFFFFF) as u8) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `cst_usize` statements.
pub fn emit_bens_cst_usize() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Cst, Lbl::Usize]);
    });
    for len in CST_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    ret[0] += (*val > 0xFF) as usize + (*val > 0xFFFF) as usize + (*val > 0xFFFFFF) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

pub static ACM_RNG: Range<u32> = 4..18;

/// Emits a token stream for the `acm_rd_ptr` statements.
pub fn emit_bens_acm_rd_ptr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Rd, Lbl::Ptr]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    ret[0] += (*val > 0xFF) as usize + (*val > 0xFFFF) as usize + (*val > 0xFFFFFF) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_rd_val` statements.
pub fn emit_bens_acm_rd_val() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Rd, Lbl::Val]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    let val = *val;
                    ret[0] += (val > 0xFF) as usize + (val > 0xFFFF) as usize + (val > 0xFFFFFF) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_add_cnt` statements.
pub fn emit_bens_acm_add_cnt() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Add, Lbl::Cnt]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                ret[0] = vals.len();
                for val in vals.iter() {
                    let val = *val;
                    ret[0] += (val > 0xFF) as usize + (val > 0xFFFF) as usize + (val > 0xFFFFFF) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_add_one` statements.
pub fn emit_bens_acm_add_one() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Add, Lbl::One]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0usize; 1];
                tme.borrow_mut().start();
                for val in vals.iter() {
                    let val = *val;
                    ret[0] += 1usize + (val > 0xFF) as usize + (val > 0xFFFF) as usize + (val > 0xFFFFFF) as usize;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_unr_0` statements.
pub fn emit_bens_acm_unr_0() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Acm, Lbl::Unr(0)]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                let mut n: usize = 0;
                tme.borrow_mut().start();
                while n < #lit_len {
                    ret[0] += vals[n];
                    n += 1;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_unr_8_one_var_1` statements.
pub fn emit_bens_acm_unr_8_var_1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(1)]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 1];
                let mut n: usize = 0;
                tme.borrow_mut().start();
                while n < #lit_len {
                    ret[0] += vals[n];
                    ret[0] += vals[n + 1];
                    ret[0] += vals[n + 2];
                    ret[0] += vals[n + 3];
                    ret[0] += vals[n + 4];
                    ret[0] += vals[n + 5];
                    ret[0] += vals[n + 6];
                    ret[0] += vals[n + 7];
                    n += 8;
                }
                tme.borrow_mut().stop();
                ret[0]
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_unr_8_var_8` statements.
pub fn emit_bens_acm_unr_8_var_8() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 8];
                let mut n: usize = 0;
                tme.borrow_mut().start();
                while n < #lit_len {
                    ret[0] += vals[n];
                    ret[1] += vals[n + 1];
                    ret[2] += vals[n + 2];
                    ret[3] += vals[n + 3];
                    ret[4] += vals[n + 4];
                    ret[5] += vals[n + 5];
                    ret[6] += vals[n + 6];
                    ret[7] += vals[n + 7];
                    n += 8;
                }
                let ret_all = ret[0] + ret[1] + ret[2] + ret[3] + ret[4] + ret[5] + ret[6] + ret[7];
                tme.borrow_mut().stop();
                ret_all
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `acm_unr_16_var_16` statements.
pub fn emit_bens_acm_unr_16_var_16() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Acm, Lbl::Unr(16), Lbl::Var(16)]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 16];
                let mut n: usize = 0;
                tme.borrow_mut().start();
                while n < #lit_len {
                    ret[0] += vals[n];
                    ret[1] += vals[n + 1];
                    ret[2] += vals[n + 2];
                    ret[3] += vals[n + 3];
                    ret[4] += vals[n + 4];
                    ret[5] += vals[n + 5];
                    ret[6] += vals[n + 6];
                    ret[7] += vals[n + 7];
                    ret[8] += vals[n + 8];
                    ret[9] += vals[n + 9];
                    ret[10] += vals[n + 10];
                    ret[11] += vals[n + 11];
                    ret[12] += vals[n + 12];
                    ret[13] += vals[n + 13];
                    ret[14] += vals[n + 14];
                    ret[15] += vals[n + 15];
                    n += 16;
                }
                let mut ret_all = ret[0] + ret[1] + ret[2] + ret[3] + ret[4] + ret[5] + ret[6] + ret[7]; 
                ret_all += ret[8] + ret[9] + ret[10] + ret[11] + ret[12] + ret[13] + ret[14] + ret[15];
                tme.borrow_mut().stop();
                ret_all
            })?;
        });
    }

    // sec: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}
