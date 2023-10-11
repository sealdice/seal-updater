use crate::lib::progress::ProgressBar;
use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Components, Path, PathBuf};
use std::{fs, io};
use zip::result::ZipError;
use zip::ZipArchive;

#[cfg(target_family = "windows")]
static UPD_NAME: &str = "seal-updater.exe";
#[cfg(target_family = "unix")]
static UPD_NAME: &str = "seal-updater";

struct ResettableArchive(File);

impl ResettableArchive {
    fn new(file: File) -> Self {
        Self(file)
    }

    fn count(&mut self) -> io::Result<usize> {
        let decoder = GzDecoder::new(&mut self.0);
        let mut archive = tar::Archive::new(decoder);
        let count = archive.entries()?.count();
        self.0.seek(SeekFrom::Start(0))?;
        Ok(count)
    }

    fn archive(self) -> tar::Archive<GzDecoder<File>> {
        let decoder = GzDecoder::new(self.0);
        tar::Archive::new(decoder)
    }
}

pub fn decompress(path: impl AsRef<Path>, target: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    if path.as_ref().as_os_str().is_empty() {
        Err("指向更新文件路径为空")?;
    }
    let file = File::open(path.as_ref())?;
    if let Err(err) = unzip(file, target.as_ref()) {
        if !matches!(err.downcast_ref(), Some(ZipError::InvalidArchive(_))) {
            Err(err)?;
        }
        let file = File::open(path.as_ref())?;
        let mut archive = ResettableArchive::new(file);
        // Get count
        let count = archive.count()?;

        untar(archive.archive(), target.as_ref(), count)?;
    }

    Ok(())
}

fn unzip(file: File, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = ZipArchive::new(file)?;
    let arc_len = archive.len();

    let mut progress_bar = ProgressBar::new(arc_len);

    for i in 0..arc_len {
        let mut zip_file = archive.by_index(i)?;
        progress_bar.blackout();
        println!("  \x1b[33mdecompressing:\x1b[0m {}", zip_file.name());
        _ = io::stdout().flush();
        // TODO: Trust and skip name check?
        let name = zip_file
            .enclosed_name()
            .ok_or("文件名不安全，可能导致 zip slip")?;
        let dest = if name.to_string_lossy() != UPD_NAME {
            target.join(name)
        } else {
            target.join("new_updater").join(name)
        };
        progress_bar.progress();
        make_file(&mut zip_file, &dest)?;
    }

    Ok(())
}

fn untar<T: Read>(
    mut archive: tar::Archive<T>,
    target: &Path,
    count: usize,
) -> Result<(), Box<dyn Error>> {
    let is_path_safe = |com: Components| {
        let normals: Vec<Component> = com
            .into_iter()
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();
        !normals.is_empty()
    };

    let mut progress_bar = ProgressBar::new(count);

    for entry in archive.entries()? {
        let mut tar_file = entry?;
        let name = tar_file.path()?;
        progress_bar.blackout();
        println!("  \x1b[33mdecompressing:\x1b[0m {}", name.to_string_lossy());
        _ = io::stdout().flush();
        if !is_path_safe(name.components()) {
            // TODO: Trust and skip name check?
            Err("文件名不安全，可能导致 slip")?;
        }
        let dest = if name.to_string_lossy() != UPD_NAME {
            target.join(name)
        } else {
            target.join("new_updater").join(name)
        };
        progress_bar.progress();
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
    if dest.is_dir() || dest.to_string_lossy().ends_with('/') {
        fs::create_dir_all(dest)?;
    } else {
        let mut out = File::create(dest)?;
        io::copy(&mut source, &mut out)?;
    }
    Ok(())
}
