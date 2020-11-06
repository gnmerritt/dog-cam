mod v4l;
mod replay;
mod frame;

use clap::{value_t, App, Arg, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Dog Cam")
        .subcommand(
            SubCommand::with_name("capture")
                .arg(Arg::with_name("name"))
                .arg(Arg::with_name("frames")),
        )
        .subcommand(
            SubCommand::with_name("process")
                .arg(Arg::with_name("name"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("capture") {
        let name = matches.value_of("name").unwrap_or("frames/last");
        let num_frames = value_t!(matches.value_of("frames"), i32).unwrap_or(30);
        return v4l::frame_writer(name, num_frames);
    }
    if let Some(matches) = matches.subcommand_matches("process") {
        let name = matches.value_of("name").unwrap_or("frames/last");
        return replay::process_from_disk(name);
    }

    Ok(())
}
