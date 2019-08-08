extern crate image;
extern crate img_hash;
extern crate hex;

use std::path::Path;
use img_hash::{ImageHash, HashType};
use std::env;

fn main() {
  env::args().skip(1).for_each(|file| {
    let img = image::open(&Path::new(&file)).unwrap();
    println!("{} {}", hex::encode(ImageHash::hash(&img, 8, HashType::Gradient).bitv.to_bytes()), file)
  })
}
