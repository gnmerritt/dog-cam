use std::fs::{self, File};
use std::io::prelude::*;
use walkdir::WalkDir;

fn get_frame_files(name: &str) -> Vec<String> {
    let mut files: Vec<String> = WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let f = e.file_name().to_string_lossy();
            if f.contains(name) && f.contains(".yuyv") {
                Some(String::from(name))
            } else {
                None
            }
        })
        .collect();
    files.sort();
    files
}

pub fn process_from_disk(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing frames with prefix {}", name);
    let frames: Vec<Vec<u8>> = get_frame_files(name)
        .into_iter()
        .filter_map(|f| fs::read(f).ok())
        .collect();
    println!("got frames: {:?}", frames);
    Ok(())
}
