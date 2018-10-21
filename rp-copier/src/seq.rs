use super::Copier;
use rp_error::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Seq;

impl Copier for Seq {
    fn file_to_file(src: &Path, target: &Path) -> Result<u64> {
        Self::copy(src, target)
    }

    fn dir_to_dir(src: &Path, target: &Path) -> Result<u64> {
        let base_dir: PathBuf = if target.is_dir() {
            target.join(src.file_name().unwrap())
        } else {
            target.to_path_buf()
        };

        if !base_dir.is_dir() {
            fs::create_dir(&base_dir)?;
        }

        for entry in src.read_dir()? {
            let entry_path = entry?.path();

            if entry_path.is_file() {
                let file_name = entry_path.file_name();
                let file_path = base_dir.join(file_name.unwrap().to_str().unwrap());

                Self::copy(entry_path.as_path(), file_path.as_path())?;
            } else {
                Self::dir_to_dir(entry_path.as_path(), base_dir.as_path())?;
            }
        }

        Ok(0)
    }
}
