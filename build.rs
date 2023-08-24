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

    Ok(())
}

/// Writes a token stream to a file.
pub fn write_one_fle(fle_stm: TokenStream, fle_pth: &PathBuf) -> std::io::Result<()> {
    let fle = syn::parse_file(fle_stm.to_string().as_str()).unwrap();
    let fmt = prettyplease::unparse(&fle);
    fs::write(fle_pth, fmt)
}

pub fn emit_imports() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        #![allow(clippy::into_iter_on_ref)]
        #![allow(clippy::needless_range_loop)]
        #![allow(clippy::slow_vector_initialization)]
        #![allow(dead_code)]
        #![allow(unused_imports)]
        use anyhow::{bail, Result};
        use ben::*;
        use ben::Sta::*;
        use itr::*;
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use std::borrow::Borrow;
        use std::fmt;
        use std::hash::Hash;
        use std::sync::Arc;
        use std::sync::mpsc::channel;
        use std::thread::{self, JoinHandle};
        use threadpool::ThreadPool;
        use Lbl::*;
    });

    stm
}

pub fn emit_main_fle() -> TokenStream {
    let tok_fns = [
        emit_imports,
        emit_main_fn,
        emit_new_stdy,
        emit_lbl_enum,
        emit_lbl_impl_display,
        emit_lbl_impl_enumstructval,
        emit_lbl_impl_label,
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
        "add", "alc", "arr", "chk", "cnt", "cst", "idx", "into_itr", "itr", "join", "lop", "mat",
        "mcr", "mpsc", "none", "one", "ptr", "rnd", "rd", "rsz", "seq", "slc", "u8", "unchk", "usize",
        "val",  "vct",
    ]
}
/// Returns label strings which map to struct u32 cases of an enum.
pub fn lbl_strs_struct_u32() -> Vec<&'static str> {
    vec!["len", "acm", "unr", "thd"]
}
pub const LBL_NAM: &str = "Lbl";

pub fn emit_lbl_enum() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    // enum: start
    stm.extend(quote! {
        /// Benchmark labels.
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum #idn_lbl
    });

    // enum: inner
    let mut stm_inr = TokenStream::new();

    for lbl_str in lbl_strs_plain() {
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

pub fn emit_lbl_impl_display() -> TokenStream {
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
            #idn => write!(f, #lit),
        });
    }
    for lbl_str in lbl_strs_struct_u32() {
        let idn = Ident::new(lbl_str.to_case(Case::Pascal).as_str(), Span::call_site());
        let mut tmp = String::from(lbl_str);
        tmp.push_str("({})");
        let lit = Literal::string(tmp.as_str());
        let lit_alt = Literal::string(lbl_str);
        stm_3.extend(quote! {
            #idn(x) => {
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

pub fn emit_lbl_impl_enumstructval() -> TokenStream {
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
            #idn(x) => Ok(x),
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

pub fn emit_lbl_impl_label() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    stm.extend(quote! {
        impl Label for #idn_lbl { }
    });

    stm
}

pub fn emit_main_fn() -> TokenStream {
    let mut stm = TokenStream::new();

    stm.extend(quote! {
        /// Runs a benchmark function analysis.
        pub fn main() -> Result<()> {
            let mut stdy = new_stdy()?;
            let itr: u16 = 64;
            let mut qry = QryBld::new();

            // Allocation: array vs vector macro
            let alc_arr_id = qry.sel(&[Alc, Arr]);
            let alc_vct_mcr_id = qry.sel(&[Alc, Vct, Mcr]);
            qry.cmp(alc_arr_id, alc_vct_mcr_id);
            // Allocation: array vs vector capacity and resize
            let alc_vct_rsz_id = qry.sel(&[Alc, Vct, Rsz]);
            qry.cmp(alc_arr_id, alc_vct_rsz_id);
            // Allocation: vector macro vs vector capacity and resize
            qry.cmp(alc_vct_mcr_id, alc_vct_rsz_id);
            // Lookup: Sequential: array vs match
            let rd_seq_arr_id = qry.sel(&[Rd, Seq, Arr]);
            let rd_seq_mat_id = qry.sel(&[Rd, Seq, Mat]);
            qry.cmp(rd_seq_arr_id, rd_seq_mat_id);
            // Lookup: Random: array vs match
            let rd_rnd_arr_id = qry.sel(&[Rd, Rnd, Arr]);
            let rd_rnd_mat_id = qry.sel(&[Rd, Rnd, Mat]);
            qry.cmp(rd_rnd_arr_id, rd_rnd_mat_id);
            // Iteration: range index bounds checked vs range index unchecked
            let lop_idx_chk_id = qry.sel(&[Lop, Idx, Chk]);
            let lop_idx_unchk_id = qry.sel(&[Lop, Idx, Unchk]);
            qry.cmp(lop_idx_chk_id, lop_idx_unchk_id);
            // Iteration: range index (bounds checked) vs iterator
            let lop_vct_itr_id = qry.sel(&[Lop, Vct, Itr]);
            qry.cmp(lop_idx_chk_id, lop_vct_itr_id);
            // Iteration: Vector: iterator vs into iterator
            let lop_vct_intoitr_id = qry.sel(&[Lop, Vct, IntoItr]);
            qry.cmp(lop_vct_itr_id, lop_vct_intoitr_id);
            // Iteration: Slice: iterator vs into iterator
            let lop_slc_itr_id = qry.sel(&[Lop, Slc, Itr]);
            let lop_slc_intoitr_id = qry.sel(&[Lop, Slc, IntoItr]);
            qry.cmp(lop_slc_itr_id, lop_slc_intoitr_id);
            // Cast: u8 vs usize
            let cst_u8_id = qry.sel(&[Cst, U8]);
            let cst_usize_id = qry.sel(&[Cst, Usize]);
            qry.cmp(cst_u8_id, cst_usize_id);
            // Accumulate: read pointer vs read de-referenced value
            let acm_rd_ptr_id = qry.sel(&[Acm(1), Rd, Ptr]);
            let acm_rd_val_id = qry.sel(&[Acm(1), Rd, Val]);
            qry.cmp(acm_rd_ptr_id, acm_rd_val_id);
            // Accumulate: total count vs multiple add one
            let acm_add_cnt_id = qry.sel(&[Acm(1), Add, Cnt]);
            let acm_add_one_id = qry.sel(&[Acm(1), Add, One]);
            qry.cmp(acm_add_cnt_id, acm_add_one_id);
            
            // Accumulate: acm 1, unr 1, thd 1 vs acm 2, unr 2, thd 1
            let acm1_unr1_thd1 = qry.sel(&[Acm(1), Unr(1), Thd(1)]);
            let acm2_unr2_thd1 = qry.sel(&[Acm(2), Unr(2), Thd(1)]);
            qry.cmp(acm1_unr1_thd1, acm2_unr2_thd1);

            // Accumulate: acm 1, unr 1, thd 1 vs acm 1, unr 8, thd 1
            let acm1_unr8_thd1 = qry.sel(&[Acm(1), Unr(8), Thd(1)]);
            qry.cmp(acm1_unr1_thd1, acm1_unr8_thd1);

            // Accumulate: unr 1, var 1 vs unr 8, var 8
            let acm8_unr8_thd1 = qry.sel(&[Acm(8), Unr(8), Thd(1)]);
            qry.cmp(acm1_unr1_thd1, acm8_unr8_thd1);

            // Accumulate: unr 8, var 8 vs unr 16, var 16
            let acm16_unr16_thd1 = qry.sel(&[Acm(16), Unr(16), Thd(1)]);
            qry.cmp(acm8_unr8_thd1, acm16_unr16_thd1);

            // Accumulate: unr 1, var 1 vs unr 16, var 16
            qry.cmp(acm1_unr1_thd1, acm16_unr16_thd1);

            // Accumulate: acm 1, unr 1, thd 2, join vs acm 1, unr 1, thd 2, mpsc
            let acm1_unr1_thd2_join = qry.sel(&[Acm(1), Unr(1), Thd(2), Join]);
            let acm1_unr1_thd2_mpsc = qry.sel(&[Acm(1), Unr(1), Thd(2), Mpsc]);
            qry.cmp(acm1_unr1_thd2_join, acm1_unr1_thd2_mpsc);

            // Accumulate: acm 1, unr 1, thd 2, mpsc vs acm 1, unr 1, thd 4, mpsc
            let acm1_unr1_thd4_mpsc = qry.sel(&[Acm(1), Unr(1), Thd(4), Mpsc]);
            qry.cmp(acm1_unr1_thd2_mpsc, acm1_unr1_thd4_mpsc);

            // Accumulate: acm 1, unr 1, thd 4, mpsc vs acm 1, unr 1, thd 8, mpsc
            let acm1_unr1_thd8_mpsc = qry.sel(&[Acm(1), Unr(1), Thd(8), Mpsc]);
            qry.cmp(acm1_unr1_thd4_mpsc, acm1_unr1_thd8_mpsc);

            // Accumulate: acm 1, unr 1, thd 8, mpsc vs acm 1, unr 1, thd 16, mpsc
            let acm1_unr1_thd16_mpsc = qry.sel(&[Acm(1), Unr(1), Thd(16), Mpsc]);
            qry.cmp(acm1_unr1_thd8_mpsc, acm1_unr1_thd16_mpsc);

            qry.cmp(acm1_unr1_thd1, acm1_unr1_thd2_mpsc);
            qry.cmp(acm1_unr1_thd1, acm1_unr1_thd4_mpsc);
            qry.cmp(acm1_unr1_thd1, acm1_unr1_thd8_mpsc);
            qry.cmp(acm1_unr1_thd1, acm1_unr1_thd16_mpsc);
            
            stdy.run(qry, itr)?;
            Ok(())
        }
    });

    stm
}

pub fn emit_new_stdy() -> TokenStream {
    let mut stm = TokenStream::new();

    let idn_lbl = Ident::new(LBL_NAM, Span::call_site());

    // fn: start
    stm.extend(quote! {
        /// Returns a study with registered benchmark functions.
        pub fn new_stdy() -> Result<Stdy<#idn_lbl>>
    });

    // fn: inner
    let mut stm_inr = TokenStream::new();
    let tok_bens = [
        emit_alc_arr,
        emit_alc_vct_mcr,
        emit_alc_vct_rsz,
        emit_rd_seq_arr,
        emit_rd_seq_mat,
        emit_rd_rnd_arr,
        emit_rd_rnd_mat,
        emit_lop_idx_chk,
        emit_lop_idx_unchk,
        emit_lop_vec_itr,
        emit_lop_vec_into_itr,
        emit_lop_slc_itr,
        emit_lop_slc_into_itr,
        emit_cst_u8,
        emit_cst_usize,
        emit_acm_rd_ptr,
        emit_acm_rd_val,
        emit_acm_add_cnt,
        emit_acm_add_one,
        emit_acm1_unr1_thd1,
        emit_acm2_unr2_thd1,
        emit_acm1_unr8_thd1,
        emit_acm8_unr8_thd1,
        emit_acm16_unr16_thd1,
        emit_acm1_unr1_thd2_join,
        emit_acm1_unr1_thd2_mpsc,
        emit_acm1_unr1_thd4_mpsc,
        emit_acm1_unr1_thd8_mpsc,
        emit_acm1_unr1_thd16_mpsc,
    ];
    tok_bens
        .iter()
        .for_each(|tok_ben| stm_inr.extend(tok_ben()));

    // fn: end
    stm.extend(quote! {
        {
            let mut ret = Stdy::new();
            #stm_inr
            Ok(ret)
        }
    });

    stm
}

pub static ALC_RNG: Range<u32> = 4..18;

pub fn emit_alc_arr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins(Len(#lit_len), || [0u32; #lit_len]);
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Alc, Arr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_alc_vct_mcr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins(Len(#lit_len), || vec![0u32; #lit_len]);
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Alc, Vct, Mcr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_alc_vct_rsz() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ALC_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins(Len(#lit_len), || {
                let mut ret = Vec::<u32>::with_capacity(#lit_len);
                ret.resize(#lit_len, 0);
                ret
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Alc, Vct, Rsz], |x| {
            #stm_inr
        });
    });

    stm
}

pub static RD_RNG: Range<u32> = 4..12;

pub fn emit_rd_seq_arr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in RD_RNG.clone().map(|x| 2u32.pow(x)) {
        // Create an array with random elements.
        let mut stm_arr = TokenStream::new();
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            let lit_ret_n = Literal::u32_unsuffixed(rng.gen_range(0..u32::MAX));
            stm_arr.extend(quote! { #lit_ret_n, });
        }

        // Read each element from an array in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins(Len(#lit_len), || {
                let arr = [#stm_arr];
                let mut ret = [0u32; 1];
                for idx in 0..#lit_len {
                    ret[0] = arr[idx];
                }
                ret[0]
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Rd, Seq, Arr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_rd_seq_mat() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
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
            x.ins(Len(#lit_len), || {
                let mut ret = [0u32; 1];
                for idx in 0..#lit_len {
                    ret[0] = match idx {
                        #stm_arm
                        _ => panic!("uh oh, no no: beyond the match limit"),
                    }
                }
                ret[0]
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Rd, Seq, Mat], |x| {
            #stm_inr
        });
    });

    stm

}

pub fn emit_rd_rnd_arr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
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
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Rd, Rnd, Arr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_rd_rnd_mat() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
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

        // Read each element from an array in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Rd, Rnd, Mat], |x| {
            #stm_inr
        });
    });

    stm

}

pub static LOP_RNG: Range<u32> = 4..18;

pub fn emit_lop_idx_chk() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Read each element from an array in sequence.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, Idx, Chk], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_lop_idx_unchk() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, Idx, Unchk], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_lop_vec_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, Itr, Vct], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_lop_vec_into_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, IntoItr, Vct], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_lop_slc_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, Slc, Itr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_lop_slc_into_itr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in LOP_RNG.clone().map(|x| 2u32.pow(x)) {
        // Iterate a for loop with range syntax 0..len.
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Lop, Slc, IntoItr], |x| {
            #stm_inr
        });
    });

    stm
}


pub static CST_RNG: Range<u32> = 4..18;

pub fn emit_cst_u8() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in CST_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Cst, U8], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_cst_usize() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in CST_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Cst, Usize], |x| {
            #stm_inr
        });
    });

    stm
}

pub static ACM_RNG: Range<u32> = 4..18;

pub fn emit_acm_rd_ptr() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Rd, Ptr], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm_rd_val() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Rd, Val], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm_add_cnt() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Add, Cnt], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm_add_one() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Add, One], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr1_thd1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(1)], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm2_unr2_thd1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(2), Unr(2), Thd(1)], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr8_thd1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(8), Thd(1)], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm8_unr8_thd1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(8), Unr(8), Thd(1)], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm16_unr16_thd1() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in ACM_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(16), Unr(16), Thd(1)], |x| {
            #stm_inr
        });
    });

    stm
}

pub static PLL_RNG: Range<u32> = 4..18;

pub fn emit_acm1_unr1_thd2_join() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(2), Join], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr1_thd2_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(2), Mpsc], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr1_thd4_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(4), Mpsc], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr1_thd8_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(8), Mpsc], |x| {
            #stm_inr
        });
    });

    stm
}

pub fn emit_acm1_unr1_thd16_mpsc() -> TokenStream {
    let mut stm = TokenStream::new();

    // sec: inner
    let mut stm_inr = TokenStream::new();
    for len in PLL_RNG.clone().map(|x| 2u32.pow(x)) {
        let lit_len = Literal::u32_unsuffixed(len);
        stm_inr.extend(quote! {
            x.ins_prm(Len(#lit_len), |tme| {
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
            });
        });
    }

    // sec: end
    stm.extend(quote! {
        ret.reg_bld(&[Acm(1), Unr(1), Thd(16), Mpsc], |x| {
            #stm_inr
        });
    });

    stm
}