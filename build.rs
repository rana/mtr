#![feature(type_name_of_val)]
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rand::Rng;
use std::any::type_name_of_val;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Runs the build script.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    write_all_files("./src/")
}

/// Writes all files to a directory.
pub fn write_all_files(dir: &str) -> std::io::Result<()> {
    let pth = Path::new(dir);
    fs::create_dir_all(pth)?;

    write_one_fle(emit_fle_main(), &pth.join("main.rs"))?;
    write_one_fle(emit_fle_mod_bens(), &pth.join("bens.rs"))?;

    Ok(())
}

/// Writes a token stream to a file.
pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(fle_pth, fmt)
}

/// Emits a token stream for the `main` file.
pub fn emit_fle_main() -> TokenStream {
    let tok_fns = [emit_fn_main, emit_fn_main_arrvec, emit_fn_main_matarr];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` function.
pub fn emit_fn_main() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(dead_code)]
        #![feature(core_intrinsics)]
        // mod ben;
        use ben::*;
        mod bens;
        use bens::*;
        use comfy_table::{Table, presets::UTF8_FULL};
        use clap::{Parser, Subcommand};
    });
    stm.extend(quote! {
        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Option<Commands>,
        }
        #[derive(Subcommand)]
        enum Commands {
            /// Array-vector comparison
            ArrVec,
            /// Match-Array comparison
            MatArr,
        }
    });
    stm.extend(quote! {
        pub fn main() {
            let cli = Cli::parse();
            match &cli.command {
                Some(Commands::ArrVec) => {
                    arr_vec();
                }
                Some(Commands::MatArr) => {
                    mat_arr();
                }
                None => {}
            }
        }
    });

    stm
}

/// Emits a token stream for the `emit_fn_main_arrvec` function.
pub fn emit_fn_main_arrvec() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn arr_vec() {
            let stdy = Stdy::<Lbl>::new(&[Lbl::Raw, Lbl::Cyc]);
            sec_arr_alc(&stdy);
            sec_vec_alc(&stdy);
            let qry = stdy.qry(&[Lbl::Alc, Lbl::Arr, Lbl::Prm(0)]).unwrap();
            qry.srt(Lbl::Len(0));

            // Create lbls series
            let mut apnd = qry.lbls(Lbl::Len(0)).unwrap();
            apnd.extend([Lbl::Ser, Lbl::Asc]);
            let dat_lbls = qry.ins_dat_lbls(Lbl::Len(0), &apnd).unwrap();

            // Create display table
            let mut tbl = Table::new();
            tbl.load_preset(UTF8_FULL);
            tbl.set_header(dat_lbls.vals_row_prpnd(&["len"]));

            apnd.push(Lbl::Mdn);
            let mut mdn_dats: Vec<Dat<Lbl>> = Vec::new();
            // TODO: USE SINGLE PRMS LIST?
            let prm: u32 = 0;
            if let Some(qry) = stdy.qry_srt(&[Lbl::Arr, Lbl::Alc, Lbl::Prm(prm)], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            if let Some(qry) = stdy.qry_srt(&[Lbl::Vec, Lbl::Alc, Lbl::Prm(prm), Lbl::Rsz], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            if let Some(qry) = stdy.qry_srt(&[Lbl::Vec, Lbl::Alc, Lbl::Prm(prm), Lbl::Mcr], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                // mdn_dats.push(dat.clone());
            }
            if mdn_dats.len() >= 2 {
                let len = mdn_dats[0].vals.len().min(mdn_dats[1].vals.len());
                let mut vals: Vec<u64> = Vec::with_capacity(len);
                for n in 0..len {
                    let mut min: u64;
                    let max: u64;
                    if mdn_dats[0].vals[n] < mdn_dats[1].vals[n] {
                        min = mdn_dats[0].vals[n];
                        max = mdn_dats[1].vals[n];
                    } else {
                        min = mdn_dats[1].vals[n];
                        max = mdn_dats[0].vals[n];
                    }
                    min = min.max(1);
                    vals.push(max.saturating_div(min));
                }
                // TODO: LBLS INSTERSECT?
                let mut lbls_cmp: Vec<Lbl> = Vec::new();
                lbls_cmp.extend(apnd);
                let dat_cmp = Dat::new(&lbls_cmp, vals);
                tbl.add_row(dat_cmp.vals_row_prpnd(&["times"]));
                // stdy.ins(dat_cmp);
            }
            println!("{tbl}");
        }
    });

    stm
}

/// Emits a token stream for the `emit_fn_main_matarr` function.
pub fn emit_fn_main_matarr() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn mat_arr() {
            let stdy = Stdy::<Lbl>::new(&[Lbl::Raw, Lbl::Cyc]);
            sec_mat(&stdy);
            let qry = stdy.qry_srt(&[Lbl::Mat], Lbl::Len(0)).unwrap();
            // Create lbls series
            let mut apnd = qry.lbls(Lbl::Len(0)).unwrap();
            apnd.extend([Lbl::Ser, Lbl::Asc]);
            let dat_lbls = qry.ins_dat_lbls(Lbl::Len(0), &apnd).unwrap();

            // Create display table
            let mut tbl = Table::new();
            tbl.load_preset(UTF8_FULL);
            tbl.set_header(dat_lbls.vals_row_prpnd(&["len"]));

            apnd.push(Lbl::Mdn);
            let mut mdn_dats: Vec<Dat<Lbl>> = Vec::new();
            if let Some(qry) = stdy.qry_srt(&[Lbl::Mat], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            if let Some(qry) = stdy.qry_srt(&[Lbl::Arr], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            if mdn_dats.len() >= 2 {
                let len = mdn_dats[0].vals.len().min(mdn_dats[1].vals.len());
                let mut vals: Vec<u64> = Vec::with_capacity(len);
                for n in 0..len {
                    let mut min: u64;
                    let max: u64;
                    if mdn_dats[0].vals[n] < mdn_dats[1].vals[n] {
                        min = mdn_dats[0].vals[n];
                        max = mdn_dats[1].vals[n];
                    } else {
                        min = mdn_dats[1].vals[n];
                        max = mdn_dats[0].vals[n];
                    }
                    min = min.max(1);
                    vals.push(max.saturating_div(min));
                }
                // TODO: LBLS INSTERSECT?
                let mut lbls_cmp: Vec<Lbl> = Vec::new();
                lbls_cmp.extend(apnd);
                let dat_cmp = Dat::new(&lbls_cmp, vals);
                tbl.add_row(dat_cmp.vals_row_prpnd(&["times"]));
                // stdy.ins(dat_cmp);
            }
            println!("{tbl}");
        }
    });

    stm
}

/// Emits a token stream for the `bens` module.
pub fn emit_fle_mod_bens() -> TokenStream {
    let tok_fns = [
        emit_import_bens,
        emit_fn_ben_fns,
        sec_arr_alc,
        sec_vec_alc,
        sec_mat,
        emit_enum_lbl,
    ];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` imports.
pub fn emit_import_bens() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(clippy::slow_vector_initialization)]
        use ben::*;
        use core::fmt;
        use core::hash::Hash;
        use num_format::{Locale, ToFormattedString};
        use rand::distributions::{Uniform, Distribution};
    });

    stm
}

/// Emits a token stream for the `Lbl` enum.
pub fn emit_enum_lbl() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        /// `Lbl` is a label associated with a benchmark.
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
        pub enum Lbl {
            /// `Arr` is an array.
            Arr,
            /// `Vec` is an vector.
            Vec,
            /// `Alc` is an allocation operation.
            Alc,
            /// `Prm` is a parameter.
            Prm(u32),
            /// `Len` is a length.
            Len(u32),
            /// `Mdn` is a median.
            Mdn,
            /// `Rd` is a read operation.
            Rd,
            /// `Wrt` is a write operation.
            Wrt,
            /// `Mat` is a match operation.
            Mat,
            /// `Mcr` is a macro operation.
            Mcr,
            /// `Cap` is a capacity operation.
            Cap,
            /// `Rsz` is a resize operation.
            Rsz,
            /// `Lop` is a loop operation.
            Lop,
            /// `Unrl` is an unroll operation.
            Unrl,
            /// `Cyc` is a CPU cycle measurement.
            #[default]
            Cyc,
            /// `Raw` is a raw benchmark.
            Raw,
            /// `Ser` is a series.
            Ser,
            /// `Asc` is sort ascending.
            Asc,
            /// `Dsc` is sort descending.
            Dsc,
        }
        impl EnumStructVal for Lbl {
            fn val(&self) -> Option<u64> {
                match *self {
                    Lbl::Prm(x) => Some(x as u64),
                    Lbl::Len(x) => Some(x as u64),
                    _ => None,
                }
            }
        }
        impl fmt::Display for Lbl {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Lbl::Arr => write!(f, "arr"),
                    Lbl::Vec => write!(f, "vec"),
                    Lbl::Alc => write!(f, "alc"),
                    Lbl::Rd => write!(f, "rd"),
                    Lbl::Wrt => write!(f, "wrt"),
                    Lbl::Mat => write!(f, "mat"),
                    Lbl::Mcr => write!(f, "mcr"),
                    Lbl::Cap => write!(f, "cap"),
                    Lbl::Rsz => write!(f, "rsz"),
                    Lbl::Lop => write!(f, "lop"),
                    Lbl::Unrl => write!(f, "unrl"),
                    Lbl::Cyc => write!(f, "cyc"),
                    Lbl::Mdn => write!(f, "mdn"),
                    Lbl::Raw => write!(f, "raw"),
                    Lbl::Ser => write!(f, "ser"),
                    Lbl::Asc => write!(f, "asc"),
                    Lbl::Dsc => write!(f, "dsc"),
                    Lbl::Prm(x) => write!(f, "prm({})", x.to_formatted_string(&Locale::en)),
                    Lbl::Len(x) => write!(f, "len({})", x.to_formatted_string(&Locale::en)),
                }
            }
        }
    });

    stm
}

pub fn fn_name(long_name: &'static str) -> &'static str {
    // Example function name: "build_script_build::emit_fn_arr2"
    match long_name.rfind(':') {
        None => long_name,
        Some(idx) => &long_name[idx + 1..],
    }
}

pub fn fn_names() -> Vec<&'static str> {
    vec![
        // fn_name(type_name_of_val(&sec_arr_alc)),
        // fn_name(type_name_of_val(&sec_vec_alc)),
        fn_name(type_name_of_val(&sec_mat)),
    ]
}

/// Emits a token stream for the `fns` function.
pub fn emit_fn_ben_fns() -> TokenStream {
    let mut stm = TokenStream::new();

    // let tok_fns = [sec_arr_alc, sec_vec_alc];
    let fn_names = fn_names();

    // fn: start
    let lit_len = Literal::usize_unsuffixed(fn_names.len());
    stm.extend(quote! {
        pub fn ben_fns() -> [fn(stdy: &Stdy<Lbl>); #lit_len]
    });

    // fn: inner
    let inr = fn_names.iter().fold(TokenStream::new(), |mut stm, name| {
        // Example function name: "build_script_build::emit_fn_arr2"
        let idn_fn = Ident::new(name, Span::call_site());
        stm.extend(quote! {#idn_fn,});
        stm
    });

    // fn: end
    stm.extend(quote! {
        {
            [#inr]
        }
    });

    stm
}

/// Emits a token stream for the `sec_arr_alc` function.
pub fn sec_arr_alc() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    // let idn_fn = Ident::new("sec_arr_alc", Span::call_site());
    stm.extend(quote! {
        pub fn sec_arr_alc(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = stdy.sec(&[Lbl::Alc, Lbl::Arr]);
    });
    for len in (4..18).map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_ben_arr_alc(&idn_sec, len, 0));
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `sec_vec_alc` function.
pub fn sec_vec_alc() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        pub fn sec_vec_alc(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let seq = 4..18;
    let idn_sec_a = Ident::new("sec_a", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec_a = stdy.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz]);
    });
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_ben_vec_alc_rsz(&idn_sec_a, len, 0));
    }
    let idn_sec_b = Ident::new("sec_b", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec_b = stdy.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr]);
    });
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_ben_vec_alc_mcr(&idn_sec_b, len, 0));
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `sec_mat` function call.
pub fn sec_mat() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        pub fn sec_mat(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    stm_inr.extend(quote! {
        let mut val = [0u32];
    });
    let seq = 2..14;
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        let tok_arr = TokenStream::from_str(&format!("arr_of_idx_{}", len)).unwrap();
        stm_inr.extend(quote! {
            let mut #tok_arr = [0usize; 256];
            let range = Uniform::from(0..#lit_len);
            let mut rng = rand::thread_rng();
            for itm in &mut #tok_arr {
                *itm = range.sample(&mut rng);
            }
        });
    }

    let idn_sec_mat = Ident::new("sec_mat", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec_mat = stdy.sec(&[Lbl::Mat]);
    });
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_ben_sec_mat(&idn_sec_mat, len));
    }

    let idn_sec_arr = Ident::new("sec_arr", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec_arr = stdy.sec(&[Lbl::Arr]);
    });
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_ben_sec_arr(&idn_sec_arr, len));
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `emit_ben_arr_alc` function call.
pub fn emit_ben_arr_alc(idn_sec: &Ident, len: u32, prm: u32) -> TokenStream {
    let mut stm = TokenStream::new();

    let lit_len = Literal::u32_unsuffixed(len);
    let lit_prm = Literal::u32_suffixed(prm);
    stm.extend(quote! {
        #idn_sec.ben(|| [#lit_prm; #lit_len], &[Lbl::Len(#lit_len), Lbl::Prm(#lit_prm)]);
    });

    stm
}

/// Emits a token stream for the `emit_ben_vec_alc_rsz` function call.
pub fn emit_ben_vec_alc_rsz(idn_sec: &Ident, len: u32, prm: u32) -> TokenStream {
    let mut stm = TokenStream::new();

    let lit_len = Literal::u32_unsuffixed(len);
    let lit_prm = Literal::u32_suffixed(prm);
    stm.extend(quote! {
        #idn_sec.ben(|| {
            let mut ret = Vec::<u32>::with_capacity(#lit_len);
            ret.resize(#lit_len, #lit_prm);
            ret
        }, &[Lbl::Len(#lit_len), Lbl::Prm(#lit_prm)]);
    });

    stm
}

/// Emits a token stream for the `emit_ben_vec_alc_mcr` function call.
pub fn emit_ben_vec_alc_mcr(idn_sec: &Ident, len: u32, prm: u32) -> TokenStream {
    let mut stm = TokenStream::new();

    let lit_len = Literal::u32_unsuffixed(len);
    let lit_prm = Literal::u32_suffixed(prm);
    stm.extend(quote! {
        #idn_sec.ben(|| {
            vec![#lit_prm; #lit_len]
        }, &[Lbl::Len(#lit_len), Lbl::Prm(#lit_prm)]);
    });

    stm
}

/// Emits a token stream for the `emit_ben_sec_mat` function call.
pub fn emit_ben_sec_mat(idn_sec: &Ident, len: u32) -> TokenStream {
    let mut stm = TokenStream::new();

    let lit_len = Literal::u32_unsuffixed(len);

    // ben: inner
    let mut stm_inr = TokenStream::new();
    let mut rng = rand::thread_rng();
    for idx in 0..len {
        let lit_idx = Literal::u32_unsuffixed(idx);
        let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
        stm_inr.extend(quote! {
            #lit_idx => #lit_ret_n,
        });
    }

    // ben: end
    let tok_arr = TokenStream::from_str(&format!("arr_of_idx_{}", len)).unwrap();
    stm.extend(quote! {
        #idn_sec.ben(|| {
            for idx in #tok_arr {
                val[0] = match idx {
                    #stm_inr
                    _ => panic!("uh oh, no no: beyond match limit"),
                }
            }
            val[0]
        }, &[Lbl::Len(#lit_len)]);
    });

    stm
}

/// Emits a token stream for the `emit_ben_sec_arr` function call.
pub fn emit_ben_sec_arr(idn_sec: &Ident, len: u32) -> TokenStream {
    let mut stm = TokenStream::new();

    let lit_len = Literal::u32_unsuffixed(len);

    // ben: inner
    let mut stm_inr = TokenStream::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        // let lit_idx = Literal::u32_unsuffixed(idx);
        let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
        stm_inr.extend(quote! {
            #lit_ret_n,
        });
    }

    // ben: end
    let tok_arr = TokenStream::from_str(&format!("arr_of_idx_{}", len)).unwrap();
    stm.extend(quote! {
        #idn_sec.ben(|| {
            let arr = [#stm_inr];
            for idx in #tok_arr {
                val[0] = arr[idx];
            }
            val[0]
        }, &[Lbl::Len(#lit_len)]);
    });

    stm
}
