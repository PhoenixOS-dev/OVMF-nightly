use std::error::Error;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

const LATEST_OVMF_URL: &str = "
    https://retrage.github.io/edk2-nightly/bin/
";

const RELEASE_OVMF_FILES_X64: [&str; 3] = [
    "RELEASEX64_OVMF.fd",
   "RELEASEX64_OVMF_CODE.fd",
    "RELEASEX64_OVMF_VARS.fd",
];
const DEBUG_OVMF_FILES_X64: [&str; 3] = [
    "DEBUGX64_OVMF.fd",
   "DEBUGX64_OVMF_CODE.fd",
    "DEBUGX64_OVMF_VARS.fd",
];
const RELEASE_OVMF_FILES_AARCH64: [&str; 2] = [
    "RELEASEAARCH64_QEMU_EFI.fd",
    "RELEASEAARCH64_QEMU_VARS.fd",
];
const DEBUG_OVMF_FILES_AARCH64: [&str; 2] = [
    "DEBUGAARCH64_QEMU_EFI.fd",
    "DEBUGAARCH64_QEMU_VARS.fd",
];


#[tokio::main]
async fn main() {
    latest_release_ovmf_x64().await.unwrap();
    latest_debug_ovmf_x64().await.unwrap();
    latest_release_ovmf_aarch64().await.unwrap();
    latest_debug_ovmf_aarch64().await.unwrap();

}

async fn latest_release_ovmf_x64()-> Result<(), Box<dyn Error>>{
    let ovmf_out_dir = Path::new("ci-x64-release");

    fs::create_dir_all(&ovmf_out_dir)?;

    for ovmf_file in RELEASE_OVMF_FILES_X64.iter() {
        println!("INFO: Downloading {}", ovmf_file);

        let response = reqwest::get(format!("{}{}", LATEST_OVMF_URL, ovmf_file)).await?;
        let bytes = response.bytes().await?;

        let mut output = File::create(ovmf_out_dir.join(ovmf_file))?;
        output.write_all(bytes.as_ref())?;
    }

    Ok(())
}
async fn latest_debug_ovmf_x64()-> Result<(), Box<dyn Error>>{
    let ovmf_out_dir = Path::new("ci-x64-debug");

    fs::create_dir_all(&ovmf_out_dir)?;

    for ovmf_file in DEBUG_OVMF_FILES_X64.iter() {
        println!("INFO: Downloading {}", ovmf_file);

        let response = reqwest::get(format!("{}{}", LATEST_OVMF_URL, ovmf_file)).await?;
        let bytes = response.bytes().await?;

        let mut output = File::create(ovmf_out_dir.join(ovmf_file))?;
        output.write_all(bytes.as_ref())?;
    }

    Ok(())
}
async fn latest_release_ovmf_aarch64()-> Result<(), Box<dyn Error>>{
    let ovmf_out_dir = Path::new("ci-aarch64-release");

    fs::create_dir_all(&ovmf_out_dir)?;

    for ovmf_file in RELEASE_OVMF_FILES_AARCH64.iter() {
        println!("INFO: Downloading {}", ovmf_file);

        let response = reqwest::get(format!("{}{}", LATEST_OVMF_URL, ovmf_file)).await?;
        let bytes = response.bytes().await?;

        let mut output = File::create(ovmf_out_dir.join(ovmf_file))?;
        output.write_all(bytes.as_ref())?;
    }

    Ok(())
}
async fn latest_debug_ovmf_aarch64()-> Result<(), Box<dyn Error>>{
    let ovmf_out_dir = Path::new("ci-aarch64-debug");

    fs::create_dir_all(&ovmf_out_dir)?;

    for ovmf_file in DEBUG_OVMF_FILES_AARCH64.iter() {
        println!("INFO: Downloading {}", ovmf_file);

        let response = reqwest::get(format!("{}{}", LATEST_OVMF_URL, ovmf_file)).await?;
        let bytes = response.bytes().await?;

        let mut output = File::create(ovmf_out_dir.join(ovmf_file))?;
        output.write_all(bytes.as_ref())?;
    }

    Ok(())
}
