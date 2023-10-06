use crate::lib::progress::ProgressBar;
use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Component, Components, Path, PathBuf};
use std::{fs, io};
use zip::ZipArchive;

#[cfg(target_family = "windows")]
static UPD_NAME: &str = "seal-updater.exe";
#[cfg(target_family = "unix")]
static UPD_NAME: &str = "seal-updater";

struct ResettableArchive {
    file: File,
    data: Vec<u8>,
}

impl ResettableArchive {
    fn new(mut file: File) -> io::Result<Self> {
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        file.seek(SeekFrom::Start(0))?;
        Ok(Self { file, data })
    }

    fn count(&self) -> io::Result<usize> {
        let cursor = Cursor::new(self.data.clone());
        let decoder = GzDecoder::new(cursor);
        let mut archive = tar::Archive::new(decoder);
        Ok(archive.entries()?.count())
    }

    fn archive(self) -> tar::Archive<GzDecoder<File>> {
        let decoder = GzDecoder::new(self.file);
        tar::Archive::new(decoder)
    }
}

pub fn decompress(path: impl AsRef<Path>, target: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    if path.as_ref().as_os_str().is_empty() {
        Err("指向更新文件路径为空")?;
    }
    let file = File::open(path.as_ref())?;
    if unzip(file, target.as_ref()).is_err() {
        let file = File::open(path.as_ref())?;
        let archive = ResettableArchive::new(file)?;
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
    if dest.is_dir() || dest.to_string_lossy().ends_with("/") {
        fs::create_dir_all(dest)?;
    } else {
        let mut out = File::create(dest)?;
        io::copy(&mut source, &mut out)?;
    }
    Ok(())
}
