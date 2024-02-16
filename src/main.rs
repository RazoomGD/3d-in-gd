use r3dgd::run;
use text_io::read;

fn main() {
    match run() {
        Ok(_) => {println!("SUCCESS!")},
        Err(e) => {println!("ERROR: {e}")}
    }
    println!("Click \"Enter\" to finish");
    let _: String = read!("{}\n");
}