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
    let range: u32 = 100000;

    let start_machine = 1;
    let end_machine = 2;

    let mut resps = Vec::new();

    let mut machines: Vec<(u32, String)> = Vec::new();
    for i in start_machine..=end_machine {
        machines.push((i - 1, format!("hpc{}", i)));
    }

    let block_size = (range / machines.len() as u32) + 1;

    for (index, hostname) in machines.iter() {
        //let start = index * ;
        let start = (index * block_size).min(range);
        let end = ((index + 1) * block_size).min(range);
        let uri: hyper::Uri = format!("http://{}.cs.unh.edu:8000/pi/{}/{}/{}", hostname, precision, start, end).parse()?;

        println!("going to request: {}", uri.to_string());
        let resp = client.get(uri);
        resps.push(resp);
    }

    resps = machines.par_iter().map(|(index, hostname)| {
        //
        let start = (index * block_size).min(range);
        let end = ((index + 1) * block_size).min(range);
        let uri: hyper::Uri = format!("http://{}.cs.unh.edu:8000/pi/{}/{}/{}", hostname, precision, start, end).parse().unwrap();

        println!("going to request: {}", uri.to_string());
        let resp = client.get(uri);
        resp
    }).collect();

    let mut running = rug::Float::with_val(precision, 0);

    // do awaits
    let bodies = resps.par_iter_mut().map(|resp| {
        let r = resp.await.unwrap();
        let body = r.into_body();
        body
    }).collect::<Vec<hyper::body::Body>>();

    for el in resps.iter_mut() {
        let r = el.await?;
        let body = r.into_body();
        //body.to_bytes();
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        let utf8 = str::from_utf8(&bytes).unwrap();
        let next: rug::Float = serde_json::from_str(utf8).unwrap();
        //println!("int res of {}", next.to_string());
        running = running + next;
        //let next: rug::Float = serde_json::from_str(r.into_body().).unwrap();
    }

    println!("Result of computation: {}", running.to_string());

    return Ok(())
}

fn do_req(uri: String) -> Result<rug::Float, Box<dyn std::error::Error>> {
    loop {
        let mut resp = reqwest::blocking::get(&uri[..])?;
        if resp.status().is_success() {
            let text = resp.text()?;
            let parsed: rug::Float = serde_json::from_str(&text[..]).unwrap();
        } else {
            // try again?
        }
    }
}
