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
    let tok_fns = [emit_main_imports, emit_main_fn, emit_main_cli_types];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` imports.
pub fn emit_main_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        mod ben;
        mod bens;
        mod itr;
        use anyhow::{bail, Result};
        use bens::*;
        use clap::{arg, Parser};
        use crate::ben::*;
    });

    stm
}

/// Emits a token stream for the `main` function.
pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn main() -> Result<()> {
            let cli = Cli::parse();
            if let Err(e) = DBG.set(cli.dbg) {
                bail!(e);
            }
            run_mtr_qrys()?;
            Ok(())
        }
    });

    stm
}

/// Emits a token stream for the `cli` type.
pub fn emit_main_cli_types() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        /// Benchmark, query, and analyze functions
        #[derive(Parser, Debug)]
        pub struct Cli
        {
            /// Print debug information
            #[arg(short = 'd', long)]
            dbg: bool,
        }
    });

    stm
}

pub fn emit_bens_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(clippy::into_iter_on_ref)]
        #![allow(clippy::needless_range_loop)]
        #![allow(clippy::slow_vector_initialization)]
        #![allow(dead_code)]
        use anyhow::{bail, Result};
        use crate::ben::*;
        use crate::itr::*;
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use std::borrow::Borrow;
        use std::fmt;
        use std::hash::Hash;
        use std::sync::Arc;
        use std::thread::{self, JoinHandle};
        use std::sync::mpsc::channel;
        use threadpool::ThreadPool;
    });

    stm
}

pub fn emit_bens_fle() -> TokenStream {
    let tok_fns = [
        emit_bens_imports,
        emit_bens_lbl_enum,
        emit_bens_lbl_impl_display,
        emit_bens_lbl_impl_enumstructval,
        emit_bens_lbl_impl_label,
        emit_bens_run_mtr_qrys,
        emit_bens_new_mtr_set,
    ];
    let ret = tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    });

    ret
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
        "acm", "add", "alc", "arr", "chk", "cnt", "cst", "idx", "into_itr", "itr", "join", "lop", "mat",
        "mcr", "mpsc", "none", "one", "ptr", "raw", "rnd", "rd", "rsz", "seq", "slc", "u8", "unchk", "usize",
        "val",  "vec",
    ]
}
/// Returns label strings which map to struct u32 cases of an enum.
pub fn lbl_strs_struct_u32() -> Vec<&'static str> {
    vec!["len", "pll", "unr", "var"]
}
pub const LBL_NAM: &str = "Lbl";
pub const LBL_VAL_DFLT: &str = "raw";

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
    stm_3.extend(quote! { _ => bail!("label '{}' isn't a struct enum", self), });
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

pub fn emit_bens_lbl_impl_label() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm.extend(quote! {
        impl Label for #idn_lbl { }
    });

    stm
}

pub fn emit_bens_run_mtr_qrys() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        /// Run mtr queries.
        pub fn run_mtr_qrys() -> Result<()> {
            let set = new_mtr_set()?;
            let itr: u32 = 64;
            // // Allocation: array vs vector macro
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Alc, Lbl::Arr], vec![Lbl::Alc, Lbl::Vec, Lbl::Mcr]],
            //     grp: Some(vec![vec![Lbl::Alc, Lbl::Arr], vec![Lbl::Alc, Lbl::Vec, Lbl::Mcr]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Allocation: array vs vector capacity and resize
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Alc, Lbl::Arr], vec![Lbl::Alc, Lbl::Vec, Lbl::Rsz]],
            //     grp: Some(vec![vec![Lbl::Alc, Lbl::Arr], vec![Lbl::Alc, Lbl::Vec, Lbl::Rsz]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Allocation: vector macro vs vector capacity and resize
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Alc, Lbl::Vec, Lbl::Mcr], vec![Lbl::Alc, Lbl::Vec, Lbl::Rsz]],
            //     grp: Some(vec![vec![Lbl::Alc, Lbl::Vec, Lbl::Mcr], vec![Lbl::Alc, Lbl::Vec, Lbl::Rsz]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Lookup: Sequential: array vs match
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Rd, Lbl::Seq, Lbl::Arr], vec![Lbl::Rd, Lbl::Seq, Lbl::Mat]],
            //     grp: Some(vec![vec![Lbl::Rd, Lbl::Seq, Lbl::Arr], vec![Lbl::Rd, Lbl::Seq, Lbl::Mat]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Lookup: Random: array vs match
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Rd, Lbl::Rnd, Lbl::Arr], vec![Lbl::Rd, Lbl::Rnd, Lbl::Mat]],
            //     grp: Some(vec![vec![Lbl::Rd, Lbl::Rnd, Lbl::Arr], vec![Lbl::Rd, Lbl::Rnd, Lbl::Mat]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Iteration: range index (bounds checked) vs iterator
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Idx, Lbl::Chk], vec![Lbl::Lop, Lbl::Itr, Lbl::Vec]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Idx, Lbl::Chk], vec![Lbl::Lop, Lbl::Itr, Lbl::Vec]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Iteration: range index bounds checked vs range index unchecked
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Idx, Lbl::Chk], vec![Lbl::Lop, Lbl::Idx, Lbl::Unchk]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Idx, Lbl::Chk], vec![Lbl::Lop, Lbl::Idx, Lbl::Unchk]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Iteration: Vector: iterator vs into iterator
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Vec, Lbl::Itr], vec![Lbl::Lop, Lbl::Vec, Lbl::IntoItr]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Vec, Lbl::Itr], vec![Lbl::Lop, Lbl::Vec, Lbl::IntoItr]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Iteration: Slice: iterator vs into iterator
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Slc, Lbl::Itr], vec![Lbl::Lop, Lbl::Slc, Lbl::IntoItr]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Slc, Lbl::Itr], vec![Lbl::Lop, Lbl::Slc, Lbl::IntoItr]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Cast: u8 vs usize
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Cst, Lbl::U8], vec![Lbl::Cst, Lbl::Usize]],
            //     grp: Some(vec![vec![Lbl::Cst, Lbl::U8], vec![Lbl::Cst, Lbl::Usize]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: read pointer vs read de-referenced value
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Acm, Lbl::Rd, Lbl::Ptr], vec![Lbl::Acm, Lbl::Rd, Lbl::Val]],
            //     grp: Some(vec![vec![Lbl::Acm, Lbl::Rd, Lbl::Ptr], vec![Lbl::Acm, Lbl::Rd, Lbl::Val]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: total count vs multiple add one
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Acm, Lbl::Add, Lbl::Cnt], vec![Lbl::Acm, Lbl::Add, Lbl::One]],
            //     grp: Some(vec![vec![Lbl::Acm, Lbl::Add, Lbl::Cnt], vec![Lbl::Acm, Lbl::Add, Lbl::One]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: Unroll: Single accumulator: no unrolling vs unroll 8
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(1)]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(1)]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: Unroll 8: 1 accumulator vs 8 accumulators
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(1)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(1)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: Unroll: unroll 8, 8 accumulators vs unroll 16, 16 accumulators
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(16), Lbl::Var(16)]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(16), Lbl::Var(16)]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: Unroll: no unrolling vs unroll 8 with 8 accumulators
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(8), Lbl::Var(8)]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            // // Accumulate: Unroll: no unrolling vs unroll 16 with 16 accumulators
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(16), Lbl::Var(16)]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(16), Lbl::Var(16)]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: single thread, single accumulator vs 2 threads, 2 accumulators, join
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Join]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Join]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: single thread, single accumulator  vs 2 threads, 2 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: single thread, 2 accumulators vs 2 threads, 2 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(2), Lbl::Var(2)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(2), Lbl::Var(2)], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: 2 threads, 2 accumulators, join vs 2 threads, 2 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Join], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Join], vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: 2 threads, 2 accumulators, mspc vs 4 threads, 4 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // // Accumulate: Parallel: 1 thread, 1 accumulator vs 4 threads, 4 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Lop, Lbl::Acm, Lbl::Unr(0)], vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;
            
            // // Accumulate: Parallel: 4 threads, 4 accumulators, mspc vs 8 threads, 8 accumulators, mpsc
            // set.qry(Qry{
            //     frm: vec![vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(8), Lbl::Var(8), Lbl::Mpsc]],
            //     grp: Some(vec![vec![Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(8), Lbl::Var(8), Lbl::Mpsc]]),
            //     srt: Some(Lbl::Len(0)),
            //     sta: Some(Sta::Mdn),
            //     trn: Some(Lbl::Len(0)),
            //     cmp: true,
            //     itr,
            // })?;

            // Accumulate: Parallel: 8 threads, 8 accumulators, mspc vs 16 threads, 16 accumulators, mpsc
            set.qry(Qry{
                frm: vec![vec![Lbl::Acm, Lbl::Pll(8), Lbl::Var(8), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(16), Lbl::Var(16), Lbl::Mpsc]],
                grp: Some(vec![vec![Lbl::Acm, Lbl::Pll(8), Lbl::Var(8), Lbl::Mpsc], vec![Lbl::Acm, Lbl::Pll(16), Lbl::Var(16), Lbl::Mpsc]]),
                srt: Some(Lbl::Len(0)),
                sta: Some(Sta::Mdn),
                trn: Some(Lbl::Len(0)),
                cmp: true,
                itr,
            })?;

            Ok(())
        }
    });

    stm
}

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
        emit_bens_lop_vec_itr,
        emit_bens_lop_vec_into_itr,
        emit_bens_lop_slc_itr,
        emit_bens_lop_slc_into_itr,
        emit_bens_cst_u8,
        emit_bens_cst_usize,
        emit_bens_acm_rd_ptr,
        emit_bens_acm_rd_val,
        emit_bens_acm_add_cnt,
        emit_bens_acm_add_one,
        emit_bens_acm_unr_0,
        emit_bens_acm_unr_2_var_2,
        emit_bens_acm_unr_8_var_1,
        emit_bens_acm_unr_8_var_8,
        emit_bens_acm_unr_16_var_16,
        emit_bens_acm_pll_2_var_2_join,
        emit_bens_acm_pll_2_var_2_mpsc,
        emit_bens_acm_pll_4_var_4_mpsc,
        emit_bens_acm_pll_8_var_8_mpsc,
        emit_bens_acm_pll_16_var_16_mpsc,
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

pub fn emit_bens_lop_vec_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Itr, Lbl::Vec]);
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

pub fn emit_bens_lop_vec_into_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::IntoItr, Lbl::Vec]);
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

pub fn emit_bens_lop_slc_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Itr, Lbl::Slc]);
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
                for val in vals.as_slice().iter() {
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

pub fn emit_bens_lop_slc_into_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::IntoItr, Lbl::Slc]);
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
                for val in vals.as_slice().into_iter() {
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


pub static CST_RNG: Range<u32> = 4..18;

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

pub fn emit_bens_acm_unr_2_var_2() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Lop, Lbl::Acm, Lbl::Unr(2), Lbl::Var(2)]);
    });
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rng = thread_rng();
                vals.shuffle(&mut rng);
                let mut ret = [0u32; 2];
                let mut n: usize = 0;
                tme.borrow_mut().start();
                while n < #lit_len {
                    ret[0] += vals[n];
                    ret[1] += vals[n + 1];
                    n += 2;
                }
                tme.borrow_mut().stop();
                ret[0] + ret[1]
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

pub static PLL_RNG: Range<u32> = 4..18;

// pub fn emit_bens_acm_pll_2_var_2() -> TokenStream {
//     let mut stm = TokenStream::new();

//     // sec: inner
//     let mut stm_inr = TokenStream::new();
//     let idn_sec = Ident::new("sec", Span::call_site());
//     stm_inr.extend(quote! {
//         let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(22), Lbl::Var(2), Lbl::Join]);
//     });
//     for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
//         let lit_len = Literal::u32_unsuffixed(len);
//         stm_inr.extend(quote! {
//             #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
//                 // Create a list of random u32s.
//                 let mut vals: Vec<u32> = (0u32..#lit_len).collect();
//                 let mut rnd_rng = thread_rng();
//                 vals.shuffle(&mut rnd_rng);
//                 let vals: Arc<Vec<u32>> = Arc::new(vals);

//                 // Sum the list of values in parallel.
//                 // Use a separate accumulator in each thread.
//                 let thd_cnt: usize = 2;
//                 let mut hndls: Vec<JoinHandle<(u32, Tme)>> = Vec::with_capacity(thd_cnt);
//                 for rng in rngs(thd_cnt, vals.len()) {
//                     let vals_clone = vals.clone();
//                     let hndl = thread::spawn(move || {
//                         let mut thd_tme = Tme(0);
//                         thd_tme.start();
//                         let mut acm: u32 = 0;
//                         let vals_read: &Vec<u32> = vals_clone.borrow();
//                         for idx in rng {
//                             acm += vals_read[idx];
//                         }
//                         thd_tme.stop();
//                         (acm, thd_tme)
//                     });
//                     hndls.push(hndl);
//                 }

//                 // Combine the separate accumulators into a single value.
//                 // Combine time to run each thread, and time to calculate sum.
//                 let mut tme_cmb = Tme(0);
//                 tme_cmb.start();
//                 let mut sum_val: u32 = 0;
//                 let mut sum_thd_tme: u64 = 0;
//                 for hndl in hndls {
//                     let thd_ret = hndl.join().unwrap();
//                     sum_val += thd_ret.0;
//                     sum_thd_tme += thd_ret.1.0;
//                 }
//                 tme_cmb.stop();
//                 tme.borrow_mut().0 += tme_cmb.0 + sum_thd_tme;

//                 sum_val
//             })?;
//         });
//     }

//     // sec: end
//     stm.extend(quote! {
//         {
//             #stm_inr
//         }
//     });

//     stm
// }

pub fn emit_bens_acm_pll_2_var_2_join() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Join]);
    });
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                // Create a list of random u32s.
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rnd_rng = thread_rng();
                vals.shuffle(&mut rnd_rng);
                let vals: Arc<Vec<u32>> = Arc::new(vals);

                // Sum the list of values in parallel.
                // Use a separate accumulator in each thread.
                let thd_cnt: usize = 2;
                let mut hndls: Vec<JoinHandle<u32>> = Vec::with_capacity(thd_cnt);
                tme.borrow_mut().start();
                for rng in rngs(thd_cnt, vals.len()) {
                    let vals_clone = vals.clone();
                    let hndl = thread::spawn(move || {
                        let mut acm: u32 = 0;
                        let vals_read: &Vec<u32> = vals_clone.borrow();
                        for idx in rng {
                            acm += vals_read[idx];
                        }
                        acm
                    });
                    hndls.push(hndl);
                }

                // Combine the separate accumulators into a single value.
                let mut sum: u32 = 0;
                for hndl in hndls {
                    sum += hndl.join().unwrap();
                }
                tme.borrow_mut().stop();
                sum
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

pub fn emit_bens_acm_pll_2_var_2_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(2), Lbl::Var(2), Lbl::Mpsc]);
    });
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                // Create a list of random u32s.
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rnd_rng = thread_rng();
                vals.shuffle(&mut rnd_rng);
                let vals: Arc<Vec<u32>> = Arc::new(vals);

                // Sum the list of values in parallel.
                // Use a separate accumulator in each thread.
                let thd_cnt: usize = 2;
                let pool = ThreadPool::new(thd_cnt);
                let (tx, rx) = channel();
                tme.borrow_mut().start();
                for rng in rngs(thd_cnt, vals.len()) {
                    let vals = vals.clone();
                    let tx = tx.clone();
                    pool.execute(move || {
                        let mut acm: u32 = 0;
                        let vals: &Vec<u32> = vals.borrow();
                        for idx in rng {
                            acm += vals[idx];
                        }
                        tx.send(acm).unwrap();
                    });
                }

                // Combine the separate accumulators into a single value.
                let sum = rx.iter().take(thd_cnt).sum::<u32>();
                tme.borrow_mut().stop();
                sum
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

pub fn emit_bens_acm_pll_4_var_4_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(4), Lbl::Var(4), Lbl::Mpsc]);
    });
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                // Create a list of random u32s.
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rnd_rng = thread_rng();
                vals.shuffle(&mut rnd_rng);
                let vals: Arc<Vec<u32>> = Arc::new(vals);

                // Sum the list of values in parallel.
                // Use a separate accumulator in each thread.
                let thd_cnt: usize = 4;
                let pool = ThreadPool::new(thd_cnt);
                let (tx, rx) = channel();
                tme.borrow_mut().start();
                for rng in rngs(thd_cnt, vals.len()) {
                    let vals = vals.clone();
                    let tx = tx.clone();
                    pool.execute(move || {
                        let mut acm: u32 = 0;
                        let vals: &Vec<u32> = vals.borrow();
                        for idx in rng {
                            acm += vals[idx];
                        }
                        tx.send(acm).unwrap();
                    });
                }

                // Combine the separate accumulators into a single value.
                let sum = rx.iter().take(thd_cnt).sum::<u32>();
                tme.borrow_mut().stop();
                sum
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

pub fn emit_bens_acm_pll_8_var_8_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(8), Lbl::Var(8), Lbl::Mpsc]);
    });
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                // Create a list of random u32s.
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rnd_rng = thread_rng();
                vals.shuffle(&mut rnd_rng);
                let vals: Arc<Vec<u32>> = Arc::new(vals);

                // Sum the list of values in parallel.
                // Use a separate accumulator in each thread.
                let thd_cnt: usize = 8;
                let pool = ThreadPool::new(thd_cnt);
                let (tx, rx) = channel();
                tme.borrow_mut().start();
                for rng in rngs(thd_cnt, vals.len()) {
                    let vals = vals.clone();
                    let tx = tx.clone();
                    pool.execute(move || {
                        let mut acm: u32 = 0;
                        let vals: &Vec<u32> = vals.borrow();
                        for idx in rng {
                            acm += vals[idx];
                        }
                        tx.send(acm).unwrap();
                    });
                }

                // Combine the separate accumulators into a single value.
                let sum = rx.iter().take(thd_cnt).sum::<u32>();
                tme.borrow_mut().stop();
                sum
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

pub fn emit_bens_acm_pll_16_var_16_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = ret.sec(&[Lbl::Acm, Lbl::Pll(16), Lbl::Var(16), Lbl::Mpsc]);
    });
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ins_prm(&[Lbl::Len(#lit_len)], |tme| {
                // Create a list of random u32s.
                let mut vals: Vec<u32> = (0u32..#lit_len).collect();
                let mut rnd_rng = thread_rng();
                vals.shuffle(&mut rnd_rng);
                let vals: Arc<Vec<u32>> = Arc::new(vals);

                // Sum the list of values in parallel.
                // Use a separate accumulator in each thread.
                let thd_cnt: usize = 16;
                let pool = ThreadPool::new(thd_cnt);
                let (tx, rx) = channel();
                tme.borrow_mut().start();
                for rng in rngs(thd_cnt, vals.len()) {
                    let vals = vals.clone();
                    let tx = tx.clone();
                    pool.execute(move || {
                        let mut acm: u32 = 0;
                        let vals: &Vec<u32> = vals.borrow();
                        for idx in rng {
                            acm += vals[idx];
                        }
                        tx.send(acm).unwrap();
                    });
                }

                // Combine the separate accumulators into a single value.
                let sum = rx.iter().take(thd_cnt).sum::<u32>();
                tme.borrow_mut().stop();
                sum
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
