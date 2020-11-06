mod v4l;

use clap::{value_t, App, Arg, SubCommand};
use crate::v4l::frame_writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Dog Cam")
        .subcommand(
            SubCommand::with_name("capture")
                .arg(Arg::with_name("name"))
                .arg(Arg::with_name("frames")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("capture") {
        let name = matches.value_of("name").unwrap_or("frames/last");
        let num_frames = value_t!(matches.value_of("frames"), i32).unwrap_or(30);
        return frame_writer(name, num_frames);
    }

    Ok(())
}
