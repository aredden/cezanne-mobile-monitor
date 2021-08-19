#![feature(path_try_exists)]
use fs_extra::dir::{copy, CopyOptions};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type)
}

fn copy_to_output_dir(path: &PathBuf, filename: String) {
    let dst = get_output_path();
    let dst_path = dst.join(filename.to_owned());
    if dst_path.to_owned().exists() {
        println!("cargo:warning={:?} already exists, skipping copy", dst_path);
    } else if dst.exists() {
        match fs::copy(path.to_path_buf(), dst_path.to_owned()) {
            Ok(_ok) => {
                println!(
                    "cargo:success=Copied {:?} to {:?}",
                    path,
                    dst_path.to_owned()
                );
            }
            Err(e) => {
                panic!(
                    "cargo:warning=Failed to copy {:?} to {:?}, got error:{:?}",
                    path,
                    dst_path.to_owned(),
                    e
                );
            }
        }
    } else {
        panic!("cargo:warning=Output directory does not exist: {:?}", dst);
    }
}

fn main() {
    let reqs = [
        "inpout32.dll",
        "inpoutx64.dll",
        "inpoutx64.lib",
        "inpoutx64.sys",
        "WinRing0x64.dll",
        "WinRing0x64.lib",
        "WinRing0x64.sys",
        "WinRing0x64.exp",
    ];
    let out_path = get_output_path();
    let source_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&source_dir).join("libs");
    let offset_maps_dir = Path::new(&source_dir).join("offset_maps");
    let mut options = CopyOptions::new();
    options.skip_exist = true;
    options.overwrite = true;
    match copy(offset_maps_dir.to_owned(), &out_path, &options) {
        Ok(_ok) => {
            println!(
                "cargo:success=Copied {:?} to {:?}",
                &offset_maps_dir, &out_path
            );
        }
        Err(e) => {
            panic!(
                "cargo:warning=Failed to copy {:?} to {:?}, got error:{:?}",
                &offset_maps_dir, &out_path, e
            );
        }
    }
    for req in reqs {
        copy_to_output_dir(&lib_dir.to_owned().join(req.to_owned()), req.to_owned());
    }
}
