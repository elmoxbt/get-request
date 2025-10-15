use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n\n{}", body);
    Ok(())
}
