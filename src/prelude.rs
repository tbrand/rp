use rp_core::copier::Copier;
use rp_core::error::Error;
use std::ffi::OsStr;
use std::path::Path;

pub fn pattern(src: &Vec<&Path>, target: &Path) -> Result<(), Error> {
    if src.len() == 0 {
        return Err("No src files specified".into());
    }

    if target.is_dir() {
        Ok(())
    } else {
        if src.len() == 1 {
            let src = src.first().unwrap();

            if src.is_file() || src.is_dir() {
                Ok(())
            } else {
                Err(format!("The source file {:?} does not exist", src).into())
            }
        } else {
            Err(format!("{:?} is file", target).into())
        }
    }
}

pub fn cp_all<T>(src: &Vec<&Path>, target: &Path, f: bool, r: bool) -> Result<u64, Error>
where
    T: Copier,
{
    let mut bytes = 0;

    for file_or_dir in src.iter() {
        bytes += cp::<T>(&file_or_dir, &target, f, r)?;
    }

    Ok(bytes)
}

fn cp<T>(src: &Path, target: &Path, f: bool, r: bool) -> Result<u64, Error>
where
    T: Copier,
{
    if src.is_file() && target.is_file() {
        if !f {
            return Err(format!("{:?} already exists. Put '-f' to overwrite it", target).into());
        }

        return T::file_to_file(src, target);
    } else if src.is_file() && target.is_dir() {
        let file: Result<&OsStr, Error> = src.file_name().ok_or("hoge".into());
        let file = file?;
        let target_path = target.join(file);

        if target_path.is_file() && !f {
            return Err(format!("{:?} already exists. Put '-f' to overwrite it", target).into());
        }

        return T::file_to_file(src, target_path.as_path());
    } else if src.is_file() {
        return T::file_to_file(src, target);
    } else {
        if !r {
            return Err(format!(
                "{:?} is a directory. Put '-r' to copy files recursively",
                src
            ).into());
        }

        if target.is_file() {
            return Err(format!("{:?} is a file", target).into());
        } else {
            return T::dir_to_dir(src, target);
        }
    }
}
