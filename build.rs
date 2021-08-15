use std::{
    env,
    fs,
    path::{Path,PathBuf}
};

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    Path::new(&manifest_dir_string).join("target").join(build_type)
}

fn copy_to_output_dir(path: &PathBuf, filename:String) {
    let dst = get_output_path();
    let dst_path = dst.join(filename.to_owned());
    if dst_path.to_owned().exists(){
        println!("cargo:{:?} already exists, skipping copy", dst_path);
    } else if dst.exists() {
        match fs::copy(path.to_path_buf(), dst_path.to_owned()){
            Ok(_ok) => {
                println!("cargo:Copied {:?} to {:?}", path, dst_path.to_owned());
            },
            Err(e) => {
                panic!("cargo:Failed to copy {:?} to {:?}, got error:{:?}", path, dst_path.to_owned(), e);
            }
        }
    } else {
        panic!("cargo:Output directory does not exist: {:?}", dst);
    }
}

fn main() {
    let reqs = [
        "inpoutx64.dll",
        "inpout32.dll",
        "inpoutx64.lib",
        "inpoutx64.sys",
        "WinRing0x64.dll",
        "WinRing0x64.lib"
        ];
    let source_dir:String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&source_dir).join("libs");
    for req in reqs {
        copy_to_output_dir(&lib_dir.join(Path::new(&req.to_owned())),req.to_owned());
    }
}