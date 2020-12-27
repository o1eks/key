use clap::{App, load_yaml};

fn main() {
    let options = load_yaml!("options.yaml");
    let matches = App::from(yaml).get_matches();

    println!("Hello, world!");
}
