extern crate hyper;
use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //let mut stream = TcpStream::
    let client = Client::new();

    let precision = 100;
    let range = 100;

    let mut resps = Vec::new();

    for (index, hostname) in [(1, "hpc1"), (2, "hpc2"), (3, "hpc3"), (4, "hpc4"), (5, "hpc5"), (6, "hpc6")].iter() {
        let start = 0;
        let end = 1;
        let uri = format!("http://{}.cs.unh.edu/pi/{}/{}/{}", hostname, precision, start, end).parse()?;

        let resp = client.get(uri);
        resps.push(resp);
    }

    return Ok(())
}
