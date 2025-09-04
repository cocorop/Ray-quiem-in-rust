use std::fs::File;
use std::io::{BufWriter, Write};

mod vec3;

//progress bar part
use std::{cmp::min, fmt::Write as WriteProgress};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Create output file
    let file = File::create("testraytracing.ppm")?;
    let mut writer = BufWriter::new(file);

    //Progress bar setup
    let total_size = image_height;
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos_lines}/{total_lines} ({eta})",
        )
        .unwrap()
        .with_key("pos_lines", |state: &ProgressState, w: &mut dyn WriteProgress| {
            write!(w, "{} L", state.pos()).unwrap()
        })
        .with_key("total_lines", |state: &ProgressState, w: &mut dyn WriteProgress| {
            if let Some(len) = state.len() {
                write!(w, "{} L", len).unwrap()
            }
        })
        .with_key("eta", |state: &ProgressState, w: &mut dyn WriteProgress| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    // Render
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {

        //progress
        let new = min(j, total_size);
        pb.set_position(new);
        thread::sleep(Duration::from_millis(120));


        for i in 0..image_width {
            let r=i as f64 / (image_width - 1) as f64;
            let g=j as f64 / (image_height - 1) as f64;
            let b=0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(writer, "{} {} {}", ir, ig, ib)?;
        }
    }
    pb.finish_with_message("finished");

    Ok(())
}
