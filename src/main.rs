use std::collections::HashMap;
use std::fs::File;
use std::io::copy;

fn foo(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(&url)?;
    //let bytes = resp.bytes()?;
    // https://cdnlive.shooowit.net/rtvalive/smil:channel7.smil/chunklist_b250000.m3u8
    // https://cdnlive.shooowit.net/rtvalive/smil:channel7.smil/chunklist_b250000.m3u8
    // https://cdnlive.shooowit.net/rtvalive/smil:channel7.smil/media_b250000_295802.aac
    // https://cdnlive.shooowit.net/rtvalive/smil:channel7.smil/media_b250000_299944.aac
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

//  EditCount 48
