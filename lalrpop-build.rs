extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .process_dir("src/")
        // .process_file("src/grammar.lalrpop")
        .expect("LALRPOP processing failed");
    println!("cargo:rerun-if-changed=src/shell/grammar.lalrpop");
}
