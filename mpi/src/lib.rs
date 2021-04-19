//use mpi::traits::*;
use rayon::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;
use rug::ops::Pow;
use rug::{Assign, Float};
use serde::{Serialize, Serializer};
use serde_json::{Result, Value};

/*fn main() {
    let universe = mpi::initialize().expect("Couldn't unwrap universe");
    let world = universe.world();
    let size = world.size;

    println!("mpi size is {}", size);
}*/

#[no_mangle]
pub unsafe extern "C"
fn rs_make_vec() -> *mut Vec<rug::Float> {
    let v: Vec<rug::Float> = Vec::new();
    let b: Box<Vec<rug::Float>> = Box::new(v);
    //b.leak().as_mut_ptr()
    let leaked = Box::leak(b);
    //leaked.as_mut_ptr()
    leaked
}

#[no_mangle]
pub unsafe extern "C"
fn rs_sum_results(v: *mut Vec<rug::Float>, precision: u32) {
    let r = v.as_mut().expect("Couldn't convert vec ptr to ref");

    let mut running = rug::Float::with_val(precision, 0);

    for res in r.iter() {
        running = running + res;
    }

    println!("Result of computation: {}", running.to_string());
}

/// uses *const u8 because I can't be bothered to make this technically UB free
/// on arbitrary systems
///
/// assumes a byte is 8 bits and all bytes are closely packed in a string
#[no_mangle]
pub unsafe extern "C"
fn rs_add_result(v: *mut Vec<rug::Float>, result: *const i8) {
    let r = v.as_mut().expect("Couldn't convert vec ptr to ref");
    let cstr = std::ffi::CStr::from_ptr(result);
    let rstr = cstr.to_str().expect("String wasn't valid UTF for deserializing");
    //println!("Added partial result {}", rstr);
    println!("Add partial result with len {}", rstr.len());

    let parsed: rug::Float = serde_json::from_str(rstr).expect("Couldn't deserialize float from given str");
    //println!("Parsed partial result, it is {}\n", parsed);

    r.push(parsed);

    println!("Added partial result");
}

#[no_mangle]
pub unsafe extern "C"
fn rs_calc_to_cstr(precision: u32, start: usize, end: usize) -> *const u8 {
    let s = par_calc_as_str(precision, start, end);

    //let cs = std::ffi::CString::from(s.as_str());
    //println!("s is {}", s);
    let cs = std::ffi::CString::new(s).expect("Couldn't convert to cstring");
    let cs = Box::new(cs);
    let cs = Box::leak(cs);

    let bytes = cs.as_bytes_with_nul();

    bytes.as_ptr()
}

fn par_calc_as_str(precision: u32, start: usize, end: usize) -> String {
    let r = par_calc(precision, start, end);
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
