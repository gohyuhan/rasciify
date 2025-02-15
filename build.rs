use sha2::{Digest, Sha256};
use std::fs::{self, create_dir_all, remove_dir_all, remove_file, File};
use std::io::{copy, Cursor, Read};
use std::path::{Path, PathBuf};
use std::{env, error::Error};
use zip::ZipArchive;

// Verify SHA256 checksum
fn verify_checksum(file_path: &Path, expected: &str) -> Result<bool, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    let result = format!("{:x}", hasher.finalize());
    Ok(result == expected)
}

// Extract ZIP archive
fn extract_zip(bytes: &[u8], output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.name());

        // Skip __MACOSX directory
        if file.name().starts_with("__MACOSX") {
            continue;
        }

        if file.is_dir() {
            create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn all_fonts_file_available() -> bool {
    let files_array:[&str; 7] = [
        "dejavu/DejaVuSansMono-Bold.ttf",
        "monotty/monotty-dev2.ttf",
        "azarmehr/AzarMehrMonospacedSansBold.ttf",
        "mitra/mitra.ttf",
        "simsun/SimSun-Bold.ttf",
        "arial-unicode/Arial-Unicode-Bold.ttf",
        "roboto/RobotoMono-Bold.ttf"
    ];
    let font_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("assets/fonts");
    for item in files_array {
        let file_path = font_dir.join(item);
        if !file_path.exists() {
            return false;
        }
    }

    return true
}

fn main() -> Result<(), Box<dyn Error>> {
    let zip_url = "https://github.com/gohyuhan/rasciify/releases/download/v0.1.0/fonts.zip";
    let expected_sha256 = "814a4f066d8b2c2cabcd7dcabb5ca4084c5b5bf8143e6353e56f2b839673bbf6";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("assets");
    let fonts_dir = out_dir.join("fonts");
    let zip_path = out_dir.join("fonts.zip");
    if !out_dir.exists(){
        create_dir_all(out_dir.clone())?;
    }

    // Download if not cached
    if !all_fonts_file_available() {
        println!("Downloading fonts...");
        let response = reqwest::blocking::get(zip_url)?;
        let mut out_file = File::create(&zip_path)?;
        copy(&mut response.take(100_000_000), &mut out_file)?;

        // Verify SHA256 checksum
        if verify_checksum(&zip_path, expected_sha256)? {
            println!("✅ SHA256 verified.");
        } else {
            panic!("❌ SHA256 checksum verification failed!");
        }

        if fonts_dir.exists(){
            let _ = remove_dir_all(fonts_dir);
        }

        // read and extract fonts file 
        println!("Extracting fonts...");
        let mut zip_bytes = Vec::new();
        File::open(&zip_path)?.read_to_end(&mut zip_bytes)?;
        extract_zip(&zip_bytes, &out_dir)?;
        println!("✅ Fonts extracted.");

        // remove zip file
        let _ = remove_file(&zip_path);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=RASCIIFY_ASSET_PATH={}", out_dir.display());
    Ok(())
}
