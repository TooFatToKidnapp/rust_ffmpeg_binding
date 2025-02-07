use ffmpeg_next as ffmpeg;
use std::env;
use std::path::Path;

fn convert_mp4_to_mkv(input_path: &Path) -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;

    // Open input file
    let mut ictx = ffmpeg::format::input(input_path)?;

    // Create output path with .mkv extension
    let output_path = input_path.with_extension("mkv");
    let mut octx = ffmpeg::format::output(&output_path)?;

    // Map input streams to output streams
    for istream in ictx.streams() {
        let codec_params = istream.parameters();
        let encoder = ffmpeg::encoder::find(codec_params.id())
            .ok_or(ffmpeg::Error::EncoderNotFound)?;

        // Add output stream
        let mut ostream = octx.add_stream(encoder)?;

        let mut modified_params = codec_params.clone();
        unsafe {
            (*modified_params.as_mut_ptr()).codec_tag = 0;
        }

        // Set parameters
        ostream.set_parameters(modified_params);
    }

    // Write output header
    octx.write_header()?;

    // Process packets and write to output
    for (ist, packet) in ictx.packets() {
        let mut opacket = packet.clone();
        let ost = octx.stream(ist.index())
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        opacket.rescale_ts(ist.time_base(), ost.time_base());
        opacket.write_interleaved(&mut octx)?;
    }

    // Finalize output
    octx.write_trailer()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.mp4>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    match convert_mp4_to_mkv(input_path) {
        Ok(_) => println!("Converted to: {}", input_path.with_extension("mkv").display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
