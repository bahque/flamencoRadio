use std::collections::HashMap;
use std::fs::File;
use std::io::copy;

fn foo(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(&url)?;
    //let bytes = resp.bytes()?;
    let mut dest = File::create("target/dest")?;
    //resp.copy_to(&mut std::io::stdout())?;
    resp.copy_to(&mut dest)?;
    //copy(&mut bytes, &mut dest)?;
    //println!("bytes: {:?}", bytes);
    Ok(())
}

fn main() {
    foo("https://lib.rs/crates/reqwest".to_string()); // https://httpbin.org/ip
    println!("Hello, world!");
}

//  EditCount 42
