#![feature(decl_macro)]

use itertools::Itertools;
use rayon::prelude::*;
use rug::ops::Pow;
use rug::{Assign, Float};
use serde::{Serialize, Serializer};
use serde_json::{Result, Value};


#[macro_use] extern crate rocket;

fn main() {
    println!("Usage: ./exe <precision> <range_start> <range_end> <output_file>");
    //let res = par_calc(100, 40, 30000);
    //println!("result of calc: {}", res.to_string());

    rocket::ignite().mount("/", routes![handler]).launch();
}

#[get("/pi/<precision>/<start>/<end>")]
fn handler(precision: u32, start: usize, end: usize) -> String {
    //println!("called with: {}, {}, {}", precision, start, end);
    let r = par_calc(precision, start, end);
    //println!("returns {}", r);
    let serialized = serde_json::to_string(&r).unwrap();
    serialized
}

fn par_calc(precision: u32, start: usize, end: usize) -> rug::Float {
    let num_cpus = num_cpus::get();
    let twelve = rug::Float::with_val(precision, 12);
    let sqrt = twelve.sqrt();
    //calculate(precision, start, end) * sqrt
    //let vec_iter = start..end;
    /*let vec_chunks = vec_iter
        .chunks((start - end) / num_cpus)
        .into_iter()
        .collect_vec();*/
    /*().map(|numrange| {
    });*/

    let rvec = (start..end).collect_vec();
    //println!("rvec is {:?}", rvec);
    let rvec: Vec<rug::Float> = rvec
        .into_par_iter()
        .chunks(((end - start) / num_cpus).max(1))
        .map(|numrange| calculate(precision, *numrange.first().unwrap(), *numrange.last().unwrap()))
        .collect();

    //&vec_chunks[..].par_iter();
    //println!("rvec after map is {:?}", rvec);

    let fref = rvec.into_par_iter().reduce( || rug::Float::with_val(precision, 0), |a, b| { a + b });

    fref * sqrt
}

fn calculate(precision: u32, start: usize, end: usize) -> rug::Float {
    // start and end signify the nth term, non-inclusive
    // if start > end then result is zero
    //println!("calculate called with {}, {}", start, end);
    if start > end {
        //println!("Return 0\n");
        rug::Float::with_val(precision, 0)
    } else {
        let mut sum = rug::Float::with_val(precision, 0);
        for iter in start..=end {
            let sign = rug::Float::with_val(precision, if iter % 2 == 0 { 1 } else { -1 });
            let numerator = sign;
            let denominator_1 = rug::Float::with_val(precision, 1 + (2 * iter));
            let denominator_2 = rug::Float::with_val(precision, 3).pow(iter as u64);

            let denominator = denominator_1 * denominator_2;
            //println!("term num, den are {}, {} for term {}", numerator.to_string(), denominator.to_string(), iter);

            let term = numerator / denominator;

            sum = sum + term;
        }

        sum
    }
}
