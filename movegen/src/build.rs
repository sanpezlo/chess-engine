mod gen_consts;

fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");

    gen_consts::generate_all();
}
