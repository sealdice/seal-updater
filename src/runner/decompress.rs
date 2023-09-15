use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::Read;
use std::path::{Component, Components, Path, PathBuf};
use flate2::read;
use zip::ZipArchive;
use crate::lib::progress;

pub fn decompress(path: impl AsRef<Path>, target: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    if path.as_ref().as_os_str().is_empty() {
        Err("指向更新文件路径为空")?;
    }
    let file = File::open(path.as_ref())?;
    let lower = path.as_ref()
        .extension()
        .ok_or("无法取得压缩文件扩展名")?
        .to_ascii_lowercase();
    let ext = lower.to_str()
        .ok_or("无法将文件扩展名转换为 UTF-8 编码")?;
    print!("正在解压…  ");
    match ext {
        "zip" => unzip(file, target.as_ref()),
        "gz" => {
            let decoder = read::GzDecoder::new(file);
            let (total, show) = match get_tar_count(decoder) {
                Ok(t) => (t, true),
                Err(_) => (0, false)
            };
            let file = File::open(path.as_ref())?;
            let decoder = read::GzDecoder::new(file);
            untar(decoder, target.as_ref(), show, total)
        },
        _ => Err(format!("压缩文件具有未知扩展名 {}", ext))?
    }
}

fn unzip(file: File, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = ZipArchive::new(file)?;
    let arc_len = archive.len();
    for i in 0..arc_len {
        let mut zip_file = archive.by_index(i)?;
        // TODO: Trust and skip name check?
        let name = zip_file.enclosed_name()
            .ok_or("文件名不安全，可能导致 zip slip")?;
        let dest = target.join(name);
        progress::print_progress(i+1, arc_len);
        make_file(&mut zip_file, &dest)?;
    }

    Ok(())
}

fn get_tar_count(reader: impl Read) -> Result<usize, Box<dyn Error>> {
    let mut archive = tar::Archive::new(reader);
    Ok(archive.entries()?.count())
}

fn untar(reader: impl Read, target: &Path, show_prog: bool, total: usize) -> Result<(), Box<dyn Error>> {
    let is_path_safe = |com: Components| {
        let normals: Vec<Component> = com
            .into_iter()
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();
        !normals.is_empty()
    };

    let mut archive = tar::Archive::new(reader);
    for (i, entry) in archive.entries()?.enumerate() {
        let mut tar_file = entry?;
        let name = tar_file.path()?;
        if !is_path_safe(name.components()) {
            // TODO: Trust and skip name check?
            Err("文件名不安全，可能导致 slip")?;
        }
        let dest = target.join(name);
        if show_prog {
            progress::print_progress(i+1, total);
        }
        make_file(&mut tar_file, &dest)?;
    }
    Ok(())
}

fn make_file(mut source: &mut impl Read, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    if dest.is_dir() || dest.to_string_lossy().ends_with("/") {
        fs::create_dir_all(dest)?;
    } else {
        let mut out = File::create(dest)?;
        io::copy(&mut source, &mut out)?;
    }
    Ok(())
}