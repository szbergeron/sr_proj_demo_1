extern crate hyper;
use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //let mut stream = TcpStream::
    let client = Client::new();

    let precision: u32 = 100;
    let range: u32 = 100;

    let mut resps = Vec::new();

    let mut machines: Vec<(u32, String)> = Vec::new();
    for i in 1..=100 {
        machines.push((i - 1, format!("hpc1")));
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

    for el in resps.iter_mut() {
        el.await?;
    }

    return Ok(())
}
