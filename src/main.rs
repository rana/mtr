#![allow(dead_code)]
#![feature(core_intrinsics)]
use ben::*;
mod bens;
use bens::*;
use comfy_table::{Table, presets::UTF8_FULL};
use clap::{Parser, Subcommand};
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
pub fn arr_vec() {
    let stdy = Stdy::<Lbl>::new(&[Lbl::Raw, Lbl::Cyc]);
    sec_arr_alc(&stdy);
    sec_vec_alc(&stdy);
    let qry = stdy.qry(&[Lbl::Alc, Lbl::Arr, Lbl::Prm(0)]).unwrap();
    qry.srt(Lbl::Len(0));
    let mut apnd = qry.lbls(Lbl::Len(0)).unwrap();
    apnd.extend([Lbl::Ser, Lbl::Asc]);
    let dat_lbls = qry.ins_dat_lbls(Lbl::Len(0), &apnd).unwrap();
    let mut tbl = Table::new();
    tbl.load_preset(UTF8_FULL);
    tbl.set_header(dat_lbls.vals_row_prpnd(&["len"]));
    apnd.push(Lbl::Mdn);
    let mut mdn_dats: Vec<Dat<Lbl>> = Vec::new();
    let prm: u32 = 0;
    if let Some(qry) = stdy.qry_srt(&[Lbl::Arr, Lbl::Alc, Lbl::Prm(prm)], Lbl::Len(0)) {
        let dat = qry.ins_dat_mdns(&apnd).unwrap();
        tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
        mdn_dats.push(dat.clone());
    }
    if let Some(qry)
        = stdy.qry_srt(&[Lbl::Vec, Lbl::Alc, Lbl::Prm(prm), Lbl::Rsz], Lbl::Len(0))
    {
        let dat = qry.ins_dat_mdns(&apnd).unwrap();
        tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
        mdn_dats.push(dat.clone());
    }
    if let Some(qry)
        = stdy.qry_srt(&[Lbl::Vec, Lbl::Alc, Lbl::Prm(prm), Lbl::Mcr], Lbl::Len(0))
    {
        let dat = qry.ins_dat_mdns(&apnd).unwrap();
        tbl.add_row(dat.vals_row_dif_lbls(&dat_lbls.lbls));
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
        let mut lbls_cmp: Vec<Lbl> = Vec::new();
        lbls_cmp.extend(apnd);
        let dat_cmp = Dat::new(&lbls_cmp, vals);
        tbl.add_row(dat_cmp.vals_row_prpnd(&["times"]));
    }
    println!("{tbl}");
}
pub fn mat_arr() {
    let stdy = Stdy::<Lbl>::new(&[Lbl::Raw, Lbl::Cyc]);
    sec_mat(&stdy);
    let qry = stdy.qry_srt(&[Lbl::Mat], Lbl::Len(0)).unwrap();
    let mut apnd = qry.lbls(Lbl::Len(0)).unwrap();
    apnd.extend([Lbl::Ser, Lbl::Asc]);
    let dat_lbls = qry.ins_dat_lbls(Lbl::Len(0), &apnd).unwrap();
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
        let mut lbls_cmp: Vec<Lbl> = Vec::new();
        lbls_cmp.extend(apnd);
        let dat_cmp = Dat::new(&lbls_cmp, vals);
        tbl.add_row(dat_cmp.vals_row_prpnd(&["times"]));
    }
    println!("{tbl}");
}
