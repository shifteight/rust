use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);

    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();

    let mut new_filename = file_stem.to_os_string().into_string().unwrap();
    new_filename.push_str("_rotated.");
    new_filename.push_str(extension.to_str().unwrap());
    let out_path = parent.join(PathBuf::from(&new_filename));
    
    let img = image::open(path).unwrap();
    let rotated = img.rotate90().rotate270();

    rotated.save(out_path).unwrap();
}
