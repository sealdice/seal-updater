use flate2::read::GzDecoder;
use log::{error, info, warn};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Components, Path, PathBuf};
use zip::result::ZipError;
use zip::ZipArchive;

use crate::colorize::Colorize;
use crate::global::{CMD_OPT, UPD_NAME};

use super::progress;

/// A wrapper that helps get the count of entries in a GZ archive.
struct ResettableTarArchive(File);

impl ResettableTarArchive {
    fn new(file: File) -> Self {
        Self(file)
    }

    /// Gets the count of entries in this file. Does not consume self.
    fn count(&mut self) -> io::Result<usize> {
        // `decoder` and `archive` hold a reference to the underlying file.
        // since they are local, this is fine.
        let decoder = GzDecoder::new(&mut self.0);
        let mut archive = tar::Archive::new(decoder);

        let count = archive.entries()?.count();
        self.0.seek(SeekFrom::Start(0))?;

        Ok(count)
    }

    /// Consumes self and returns the inner archive.
    fn archive(self) -> tar::Archive<GzDecoder<File>> {
        let decoder = GzDecoder::new(self.0);
        tar::Archive::new(decoder)
    }
}

/// Extract the content from the archive at `src` to `dst`. The source can be a ZIP or
/// a TAR file; the ZIP format is tried first, then TAR as fallback.
pub(crate) fn decompress(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    if src.as_ref().as_os_str().is_empty() {
        error!("更新文件路径为空: {:?}", src.as_ref());
        Err("指向更新文件路径为空")?;
    }

    let file = File::open(src.as_ref()).map_err(|e| {
        error!("未能打开更新压缩包: {}", e);
        e
    })?;

    if let Err(e) = unzip(file, dst.as_ref()) {
        if !matches!(e.downcast_ref(), Some(ZipError::InvalidArchive(_))) {
            error!("解压缩失败: {}", e);
            Err(e)?;
        }

        info!("尝试 ZIP 解压缩失败，开始 GZ 解压缩");

        let file = File::open(src.as_ref()).map_err(|e| {
            error!("未能打开更新压缩包: {}", e);
            e
        })?;
        let mut archive = ResettableTarArchive::new(file);

        let count = archive
            .count()
            .map_err(|e| {
                warn!("无法获取 GZ 压缩包的文件数量: {}", e);
                e
            })
            .unwrap_or(0);

        untar(archive.archive(), dst.as_ref(), count)?;
    }

    Ok(())
}

fn unzip(file: File, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = ZipArchive::new(file)?;
    let arc_len = archive.len();

    let mut progress_bar = progress::ProgressBar::new(arc_len);

    for i in 0..arc_len {
        let mut zip_file = archive.by_index(i).map_err(|e| {
            error!("获取 ZIP 文件 entry 时出现错误: {}", e);
            e
        })?;

        if CMD_OPT.verbose {
            progress::ProgressBar::blackout();
            println!("  {} {}", "decompressing:".yellow(), zip_file.name());
            _ = io::stdout().flush();
        }

        let name = zip_file
            .enclosed_name()
            .ok_or("文件名不安全，可能导致 zip slip")
            .map_err(|e| {
                error!("发现不安全的文件名，解压缩终止: {}", e);
                e
            })?;

        let dest = if name.to_string_lossy() != UPD_NAME {
            target.join(name)
        } else {
            target.join("new_updater").join(name)
        };
        make_file(&mut zip_file, &dest)?;

        progress_bar.progress();
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
            .filter(|c| matches!(c, Component::Normal(_)) || matches!(c, Component::CurDir))
            .collect();
        !normals.is_empty()
    };

    let mut progress_bar = progress::ProgressBar::new(count);

    let entries = archive.entries().map_err(|e| {
        error!("获取 GZ 文件 entry 时出现错误: {}", e);
        e
    })?;

    for entry in entries {
        let mut tar_file = entry.map_err(|e| {
            error!("获取 GZ 文件 entry 时出现错误: {}", e);
            e
        })?;

        let name = tar_file.path().map_err(|e| {
            error!("获取文件名时出现错误: {}", e);
            e
        })?;

        if CMD_OPT.verbose {
            progress::ProgressBar::blackout();
            println!("  {} {}", "reading:".yellow(), name.to_string_lossy());
            _ = io::stdout().flush();
        }

        if !is_path_safe(name.components()) {
            error!("发现不安全的文件名，解压缩终止: {:?}", name);
            Err("文件名不安全，可能导致 slip")?;
        }

        let dest = if name.to_string_lossy() != UPD_NAME {
            target.join(name)
        } else {
            target.join("new_updater").join(name)
        };
        make_file(&mut tar_file, &dest)?;

        progress_bar.progress();
    }
    Ok(())
}

fn make_file(mut source: &mut impl Read, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                error!("未能创建文件夹: {}", e);
                e
            })?;
        }
    }

    if dest.is_dir() || dest.to_string_lossy().ends_with('/') {
        fs::create_dir_all(dest).map_err(|e| {
            error!("未能创建文件夹: {}", e);
            e
        })?;
    } else {
        let mut out = File::create(dest).map_err(|e| {
            error!("未能创建文件: {}", e);
            e
        })?;

        io::copy(&mut source, &mut out).map_err(|e| {
            error!("未能复制文件: {}", e);
            e
        })?;
    }
    Ok(())
}
