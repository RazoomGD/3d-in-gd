use r3dgd::run;

fn main() {
    match run() {
        Ok(_) => {println!("SUCCESS!")},
        Err(e) => {println!("ERROR: {e}")}
    }

}