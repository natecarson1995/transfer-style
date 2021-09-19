use clap::{Arg, App};
use glob::glob;
use texture_synthesis as ts;
use std::path::Path;

// Transfer styles from style_examples onto the file at input_path
fn apply_style_to_image(input_path: String, output_path : String, style_examples:&[String], width: u32, height: u32) {
    let texsynth = ts::Session::builder()
        .add_examples(style_examples)
        .resize_input(ts::Dims {
            width: width,
            height: height,
        })
        .load_target_guide(&input_path)
        .guide_alpha(0.8)
        .build()
        .expect("Coudln't create style transfer session");

    let generated = texsynth.run(None);
    generated.save(output_path).expect("Could not write processed image to file");
}

// Convert a glob string into a vector of matching file names
fn glob_to_path_vec(glob_string: String) -> Vec<String> {
    let glob = glob(&glob_string).expect("Couldn't parse file/s glob");
    let mut output_vec:Vec<String> = Vec::new();

     for entry in glob {
        match entry {
            Ok(path) => {
                let meta = path.metadata().expect("Couldn't get file metadata");
                if meta.is_file() {
                    output_vec.push(path.display().to_string());
                }
            },
            Err(err) => println!("{:?}", err),
        }
    }

    return output_vec;
}

fn main() {
    // Set up the command line arguments
    let matches = App::new("Style transfer utility")
    .author("Nathaniel Carson. <nate@natecarson.co>")
    .about("Takes all of the images in the STYLES argument, and applies style transfer onto the files in the INPUT argument")
    .arg(Arg::with_name("STYLES")
        .short("s")
        .long("styles")
        .value_name("GLOB")
        .help("Sets the glob of image file/s to use as a style")
        .takes_value(true)
        .required(true)
    )
    .arg(Arg::with_name("WIDTH")
        .short("w")
        .long("width")
        .value_name("px")
        .help("Sets the width to resize input images to")
        .takes_value(true)
    )
    .arg(Arg::with_name("HEIGHT")
        .short("h")
        .long("height")
        .value_name("px")
        .help("Sets the height to resize input images to")
        .takes_value(true)
    )
    .arg(Arg::with_name("INPUT")
        .help("Sets the glob of image file/s to transfer the style onto")
        .required(true)
        .index(1)
    )
    .get_matches();

    // Parse width and height parameters
    let mut width:u32= 100;
    let mut height:u32 = 100;
    if let Some(w) = matches.value_of("WIDTH") {
        width = w.parse().expect("Couldn't convert width to integer");
    }
    if let Some(h) = matches.value_of("HEIGHT") {
        height = h.parse().expect("Couldn't convert height to integer");
    }

    // Parse file globs
    let styles = matches.value_of("STYLES").expect("Could not parse style command line argument");
    let styles_vec = glob_to_path_vec(styles.to_string());
    let images = matches.value_of("INPUT").expect("Could not parse input command line argument");
    let images_vec = glob_to_path_vec(images.to_string());

    println!("List of styles being applied: {:?}", styles_vec);
    // Apply styles to each input file, changing its extension to .out.png
    for image_path in images_vec{
        let output_path = Path::new(&image_path).with_extension("out.png");
        println!("Processing {} -> {}", image_path, output_path.display());
        apply_style_to_image(image_path, output_path.display().to_string(), styles_vec.as_slice(), width, height);
    }
}
