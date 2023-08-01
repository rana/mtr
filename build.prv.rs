use convert_case::{self, Case, Casing};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rand::Rng;
use std::fs;
use std::ops::Range;
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
    let tok_fns = [
        emit_main_imports,
        emit_main_fn,
        emit_main_cli_types,
        emit_main_stdy_arrvec,
        // emit_main_stdy_matarr,
    ];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for main file `imports`.
pub fn emit_main_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(dead_code)]
        #![feature(core_intrinsics)]
        mod ben;
        mod bens;
        use crate::ben::*;
        use bens::*;
        use clap::Parser;
        use comfy_table::{Table, presets::UTF8_FULL};
    });

    stm
}

/// Emits a token stream for main file `cli types`.
pub fn emit_main_cli_types() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #[derive(Parser)]
        struct Cli {
            /// Run benchmarks from one or more labels
            #[arg(short, long, value_names=["lbl", "lbl-lbl"], num_args=1.., value_delimiter=',', required=true)]
            frm: Vec<String>,
            /// Select a statisitcal function to apply to raw benchmark values
            #[arg(short = 'x', long, value_name = "lbl[stat]")]
            sel: Option<String>,
            /// Group benchmarks into one or more labels. Each label is a group
            #[arg(short, long, value_names=["lbl", "lbl-lbl"], num_args=1.., value_delimiter=',')]
            grp: Option<Vec<String>>,
            /// Sort benchmarks by label. Useful for struct labels like Len(u32)
            #[arg(short, long, value_name = "lbl[struct]")]
            srt: Option<String>,
            /// Compare pairs of benchmarks as a ratio of max/min
            #[arg(short = 'c', long, value_name = "lbl")]
            cmp: bool,
            /// Number of iterations to run a benchmark function
            #[arg(short = 'i', long, value_name = "u32", default_value_t = 16)]
            itr: u32,
        }
    });

    stm
}

/// Emits a token stream for the `main` function.
pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn main() {
            let cli = Cli::parse();
            println!("frm: {:?}", cli.frm);
            println!("sel: {:?}", cli.sel);
            println!("grp: {:?}", cli.grp);
            println!("srt: {:?}", cli.srt);
            println!("cmp: {:?}", cli.cmp);
            println!("itr: {:?}", cli.itr);
            arr_vec();
        }
    });

    stm
}

/// Emits a token stream for the `emit_main_stdy_arrvec` function.
pub fn emit_main_stdy_arrvec() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn arr_vec() {
            let stdy = Stdy::<Lbl>::new(&[Lbl::Raw]);
            ben_alc_arr(&stdy);
            ben_alc_vec_rsz(&stdy);
            // ben_alc_vec_mcr(&stdy);

            let qry = stdy.qry_srt(&[Lbl::Alc, Lbl::Arr], Lbl::Len(0)).unwrap();

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

            if let Some(qry) = stdy.qry_srt(&[Lbl::Alc, Lbl::Arr], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            if let Some(qry) = stdy.qry_srt(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz], Lbl::Len(0)) {
                let dat = qry.ins_dat_mdns(&apnd).unwrap();
                tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
                mdn_dats.push(dat.clone());
            }
            // if let Some(qry) = stdy.qry_srt(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr], Lbl::Len(0)) {
            //     let dat = qry.ins_dat_mdns(&apnd).unwrap();
            //     tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
            //     // mdn_dats.push(dat.clone());
            // }
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
                tbl.add_row(dat_cmp.vals_row_prpnd(&["ratio (max / min)"]));
                // stdy.ins(dat_cmp);
            }
            println!("{tbl}");
        }
    });

    stm
}

/// Emits a token stream for the `emit_main_stdy_matarr` function.
pub fn emit_main_stdy_matarr() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        pub fn mat_arr() {
            let stdy = Stdy::<Lbl>::new(&[Lbl::Raw]);
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
                tbl.add_row(dat_cmp.vals_row_prpnd(&["times (max / min)"]));
                // stdy.ins(dat_cmp);
            }
            println!("{tbl}");
        }
    });

    stm
}

/// Emits a token stream for the `bens` module.
pub fn emit_bens_fle() -> TokenStream {
    let tok_fns = [
        emit_bens_imports,
        emit_ben_alc_arr,
        emit_ben_alc_vec_rsz,
        // emit_ben_alc_vec_mcr,
        // sec_mat,
        emit_bens_lbl_enum,
        emit_bens_lbl_impl_display,
        emit_bens_lbl_impl_enumstructval,
        emit_bens_lbl_impl_fromstr,
        emit_bens_lbl_name_consts,
    ];
    tok_fns.iter().fold(TokenStream::new(), |mut stm, tok_fn| {
        stm.extend(tok_fn());
        stm
    })
}

/// Emits a token stream for the `main` imports.
pub fn emit_bens_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(clippy::slow_vector_initialization)]
        use crate::ben::*;
        use core::fmt;
        use core::hash::Hash;
        use core::str;
        // use rand::distributions::{Uniform, Distribution};
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

pub const LBL_DFLT: &str = "raw";

/// Emits a token stream for `LbL` name constants.
pub fn emit_bens_lbl_name_consts() -> TokenStream {
    let mut stm = TokenStream::new();

    for lbl_str in lbl_strs_all() {
        let idn = Ident::new(lbl_str.to_uppercase().as_str(), Span::call_site());
        let str = Literal::string(lbl_str);
        stm.extend(quote! {
            pub const #idn: &str = #str;
        });
    }

    stm
}

pub const LBL_STR: &str = "Lbl";

/// Emits a token stream for the `Lbl` enum.
pub fn emit_bens_lbl_enum() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_STR, Span::call_site());

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
        if lbl_str == LBL_DFLT {
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

/// Emits a token stream for `Lbl` implementing `EnumStructVal`.
pub fn emit_bens_lbl_impl_enumstructval() -> TokenStream {
    let mut stm_0 = TokenStream::new();
    let mut stm_1 = TokenStream::new();
    let mut stm_2 = TokenStream::new();
    let mut stm_3 = TokenStream::new();

    let idn_lbl = Ident::new(LBL_STR, Span::call_site());

    stm_0.extend(quote! { impl EnumStructVal for #idn_lbl });
    stm_1.extend(quote! { fn val(&self) -> Option<u64> });
    stm_2.extend(quote! { match *self });
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        stm_3.extend(quote! {
            #idn_lbl::#idn(x) => Some(x as u64),
        });
    }
    stm_3.extend(quote! { _ => None, });
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

/// Emits a token stream for `Lbl` implementing `Display`.
pub fn emit_bens_lbl_impl_display() -> TokenStream {
    let mut stm_0 = TokenStream::new();
    let mut stm_1 = TokenStream::new();
    let mut stm_2 = TokenStream::new();
    let mut stm_3 = TokenStream::new();

    let idn_lbl = Ident::new(LBL_STR, Span::call_site());

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
        stm_3.extend(quote! {
            #idn_lbl::#idn(x) => write!(f, #lit, x),
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

    let idn_lbl = Ident::new(LBL_STR, Span::call_site());

    stm_0.extend(quote! { impl str::FromStr for #idn_lbl });
    stm_1.extend(quote! { fn from_str(s: &str) -> Result<Self> });
    stm_2.extend(quote! { match s });
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
        _ => Err(format!("invalid variant: {s}")),
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

pub static ALC_RNG: Range<u32> = 4..18;

/// Emits a token stream for the `emit_ben_alc_arr` function.
pub fn emit_ben_alc_arr() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        pub fn ben_alc_arr(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = stdy.sec(&[Lbl::Alc, Lbl::Arr]);
    });
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ben(|| [0u32; #lit_len], &[Lbl::Len(#lit_len)]);
        });
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `emit_ben_alc_vec_rsz` function.
pub fn emit_ben_alc_vec_rsz() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        pub fn ben_alc_vec_rsz(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = stdy.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Rsz]);
    });
    for len in ALC_RNG.clone().clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ben(|| {
                let mut ret = Vec::<u32>::with_capacity(#lit_len);
                ret.resize(#lit_len, 0);
                ret
            }, &[Lbl::Len(#lit_len)]);
        });
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `emit_ben_alc_vec_mcr` function.
pub fn emit_ben_alc_vec_mcr() -> TokenStream {
    let mut stm = TokenStream::new();

    // fn: start
    stm.extend(quote! {
        pub fn ben_alc_vec_mcr(stdy: &Stdy<Lbl>)
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let idn_sec = Ident::new("sec", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec = stdy.sec(&[Lbl::Alc, Lbl::Vec, Lbl::Mcr]);
    });
    for len in ALC_RNG.clone().clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            #idn_sec.ben(|| {
                vec![0u32; #lit_len];
            }, &[Lbl::Len(#lit_len)]);
        });
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
        stm_inr.extend(emit_prvben_sec_mat(&idn_sec_mat, len));
    }

    let idn_sec_arr = Ident::new("sec_arr", Span::call_site());
    stm_inr.extend(quote! {
        let #idn_sec_arr = stdy.sec(&[Lbl::Arr]);
    });
    for len in seq.clone().map(|x| 2u32.pow(x)) {
        stm_inr.extend(emit_prvben_sec_arr(&idn_sec_arr, len));
    }

    // fn: end
    stm.extend(quote! {
        {
            #stm_inr
        }
    });

    stm
}

/// Emits a token stream for the `emit_ben_sec_mat` function call.
pub fn emit_prvben_sec_mat(idn_sec: &Ident, len: u32) -> TokenStream {
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
pub fn emit_prvben_sec_arr(idn_sec: &Ident, len: u32) -> TokenStream {
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
