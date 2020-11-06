use walkdir::WalkDir;
use crate::frame::Frame;

fn get_frame_files(name: &str) -> Vec<String> {
    let mut files: Vec<String> = WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let f = e.path().to_string_lossy();
            if f.contains(name) && f.contains(".yuyv") {
                Some(f.into())
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
    let frames: Vec<Frame> = get_frame_files(name)
        .iter()
        .filter_map(|f| Frame::from_file(f).ok())
        .collect();
    println!("got {} frames", frames.len());
    Ok(())
}
