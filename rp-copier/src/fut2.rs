use super::Copier;
use futures::{self, future, *};
use futures_fs::{FsPool, FsReadStream, FsWriteSink};
use num_cpus;
use rp_error::Result;
use std::fs;
use std::path::{Path, PathBuf};

type CopyFuture = Box<Future<Item = (), Error = ()>>;

pub struct Fut2;

impl Fut2 {
    fn copy_fut(src: &Path, target: &Path) -> CopyFuture {
        let f = future::ok::<(PathBuf, PathBuf), ()>((src.to_path_buf(), target.to_path_buf()))
            .map(|(src, target)| {
                Self::copy(src.as_path(), target.as_path());
                ()
            })
            .map_err(|_| ());

        Box::new(f)
     }

    fn dir_to_dir_inner(src: &Path, target: &Path) -> Result<Vec<CopyFuture>> {
        let base_dir: PathBuf = if target.is_dir() {
            target.join(src.file_name().unwrap())
        } else {
            target.to_path_buf()
        };

        if !base_dir.is_dir() {
            fs::create_dir(&base_dir)?;
        }

        let mut v: Vec<CopyFuture> = vec![];

        for entry in src.read_dir()? {
            let entry_path = entry?.path();

            if entry_path.is_file() {
                let file_name = entry_path.file_name();
                let file_path = base_dir.join(file_name.unwrap().to_str().unwrap());

                let f = Self::copy_fut(entry_path.as_path(), file_path.as_path());
                v.push(f);
            } else {
                let mut f = Self::dir_to_dir_inner(entry_path.as_path(), base_dir.as_path())?;
                v.append(&mut f);
            }
        }

        Ok(v)
    }
}

impl Copier for Fut2 {
    fn file_to_file(src: &Path, target: &Path) -> Result<u64> {
        Self::copy(src, target)
    }

    fn dir_to_dir(src: &Path, target: &Path) -> Result<u64> {
        let f = Self::dir_to_dir_inner(src, target);

        future::join_all(f?).wait().unwrap();

        Ok(0)
    }
}
