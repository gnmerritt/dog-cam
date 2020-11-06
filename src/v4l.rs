#[cfg(feature = "v4l-libs")]
fn frame_writer(name: &str, num_frames: i32) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;
    use v4l::prelude::*;
    use v4l::FourCC;

    let mut dev = CaptureDevice::new(0)?;

    // Let's say we want to explicitly request another format
    let mut fmt = dev.format()?;
    fmt.width = 1920;
    fmt.height = 1080;
    fmt.fourcc = FourCC::new(b"YUYV");
    dev.set_format(&fmt)?;

    // The actual format chosen by the device driver may differ from what we
    // requested! Print it out to get an idea of what is actually used now.
    println!("Format in use:\n{}", fmt);

    println!("Capturing {} frames to {}\n", num_frames, name);

    // Now we'd like to capture some frames!
    // First, we need to create a stream to read buffers from. We choose a
    // mapped buffer stream, which uses mmap to directly access the device
    // frame buffer. No buffers are copied nor allocated, so this is actually
    // a zero-copy operation.

    // To achieve the best possible performance, you may want to use a
    // UserBufferStream instance, but this is not supported on all devices,
    // so we stick to the mapped case for this example.
    // Please refer to the rustdoc docs for a more detailed explanation about
    // buffer transfers.

    // Create the stream, which will internally 'allocate' (as in map) the
    // number of requested buffers for us.
    let mut stream = MmapStream::with_buffers(&mut dev, 4)?;

    // At this point, the stream is ready and all buffers are setup.
    // We can now read frames (represented as buffers) by iterating through
    // the stream. Once an error condition occurs, the iterator will return
    // None.
    for i in 0..num_frames {
        let frame = stream.next().unwrap();
        let filename = format!(
            "{n}-{i}.{w}-{h}.yuyv",
            n = name,
            i = i,
            w = fmt.width,
            h = fmt.height
        );
        println!(
            "Buffer size: {}, seq: {}, timestamp: {}  --> {}",
            frame.len(),
            frame.meta().sequence,
            frame.meta().timestamp,
            filename,
        );
        let mut file: File = File::create(filename)?;
        file.write_all(frame.data())?;

        // To process the captured data, you can pass it somewhere else.
        // If you want to modify the data or extend its lifetime, you have to
        // copy it. This is a best-effort tradeoff solution that allows for
        // zero-copy readers while enforcing a full clone of the data for
        // writers.
    }

    Ok(())
}

#[cfg(not(feature = "v4l-libs"))]
pub fn frame_writer(_name: &str, _num_frames: i32) -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!("V4L not enabled, won't capture any frames")
}
