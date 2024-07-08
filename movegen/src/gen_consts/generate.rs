use std::{env, fs::File, path::Path};

use super::{king, knights, magic, pawns};

pub fn generate_all() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let magic_path = Path::new(&out_dir).join("magic_gen.rs");
    let mut f = File::create(&magic_path).unwrap();

    pawns::write(&mut f);
    king::write(&mut f);
    knights::write(&mut f);
    magic::write(&mut f);
}
