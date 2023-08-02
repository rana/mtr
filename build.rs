use convert_case::{self, Case, Casing};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
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
        mod ben;
        mod bens;
        use crate::ben::*;
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
        use core::fmt;
        use core::hash::Hash;
        use core::str;
        use crate::ben::*;
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
        "alc", "arr", "asc", "cap", "dsc", "lop", "mat", "mcr", "mdn", "raw", "rd", "rsz", "ser",
        "unr", "vec", "wrt",
    ]
}
/// Returns label strings which map to struct u32 cases of an enum.
pub fn lbl_strs_struct_u32() -> Vec<&'static str> {
    vec!["len", "prm"]
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

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm_0.extend(quote! { impl str::FromStr for #idn_lbl });
    stm_1.extend(quote! { fn from_str(s: &str) -> Result<Self> });
    stm_2.extend(quote! { match s.trim().to_lowercase().as_str() });
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
    }
    stm_3.extend(quote! {
        _ => Err(format!("invalid Lbl: {s}")),
    });
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
    stm_1.extend(quote! { fn val(&self) -> Result<u64> });
    stm_2.extend(quote! { match *self });
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        stm_3.extend(quote! {
            #idn_lbl::#idn(x) => Ok(x as u64),
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
