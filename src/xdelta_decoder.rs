extern crate xdelta;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    xdelta_encode.exe input delta output
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    xdelta_encode input delta output
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect(&USAGE);
    let delta = args.get(2).expect(&USAGE);
    let output = args.get(3).expect(&USAGE);
    
    println!("Patching delta target file: {}, from file {} using the file {}", &output, &input, &delta);
    xdelta::decode_file(Some(&input), &delta, &output);
}