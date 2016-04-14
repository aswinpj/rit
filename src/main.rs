extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut initialise = false;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("rit - git written in Rust!");
        ap.refer(&mut initialise)
            .add_option(&["-i", "--initialise"], StoreTrue,
            "initialise  repository");

        ap.parse_args_or_exit();
    }

    if initialise {
        println!("Initialising empty repository");
    }

}
