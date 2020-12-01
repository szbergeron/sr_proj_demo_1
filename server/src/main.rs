extern crate hyper;
use hyper::Client;
use std::io::Read;

use rug::ops::Pow;
use rug::{Assign, Float};
use serde::{Serialize};
use std::str;
use rayon::prelude::*;

//#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //let mut stream = TcpStream::
    println!("Usage: ./req <precision> <iterations>");
    let client = Client::new();

    let precision: u32 = 100000;
    let range: u32 = 1000000;

    let start_machine = 1;
    let end_machine = 6;

    let mut resps = Vec::new();

    let mut machines: Vec<(u32, String)> = Vec::new();
    for i in start_machine..=end_machine {
        machines.push((i - 1, format!("hpc{}", i)));
    }

    let block_size = (range / machines.len() as u32) + 1;

    resps = machines.par_iter().map(|(index, hostname)| {
        //
        let start = (index * block_size).min(range);
        let end = ((index + 1) * block_size).min(range);
        let uri = format!("http://{}.cs.unh.edu:8000/pi/{}/{}/{}", hostname, precision, start, end);
        do_req(uri).expect("do_req returned failure")
    }).collect();

    let mut running = rug::Float::with_val(precision, 0);

    for res in resps {
        running = running + res;
    }

    // do awaits

    println!("Result of computation: {}", running.to_string());

    return Ok(())
}

fn do_req(uri: String) -> Result<rug::Float, Box<dyn std::error::Error>> {
    loop {
        let timeout = std::time::Duration::from_secs(1000);
        let client = reqwest::blocking::Client::builder().timeout(timeout).build()?;
        let resp = client.get(&uri[..]).send()?;
        if resp.status().is_success() {
            let text = resp.text()?;
            let parsed: rug::Float = serde_json::from_str(&text[..]).unwrap();
            return Ok(parsed);
        } else {
            // try again?
            println!("Failed to request from {}, trying again...", uri);
            continue
        }
    }
}
