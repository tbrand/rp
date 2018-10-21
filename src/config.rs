use clap::{ArgMatches, Values};
use mode::Mode;
use rp_core::error::Error;
use std::path::Path;

pub struct Config<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Config<'a> {
    pub fn new(matches: ArgMatches<'a>) -> Self {
        Config { matches: matches }
    }

    pub fn is_r(&self) -> bool {
        self.matches.is_present("recursive")
    }

    pub fn is_f(&self) -> bool {
        self.matches.is_present("force")
    }

    pub fn files(&self) -> Result<Values, Error> {
        self.matches
            .values_of("files")
            .ok_or_else(|| "please specify files or directory to be copied".into())
    }

    pub fn src(&self) -> Result<Vec<&Path>, Error> {
        let mut files = self.files()?.map(|f| Path::new(f)).collect::<Vec<&Path>>();

        let last = files.len() - 1;
        let _ = files.split_off(last);
        Ok(files)
    }

    pub fn target(&self) -> Result<&Path, Error> {
        let target: Result<&str, Error> = self
            .files()?
            .last()
            .ok_or("please specify target file or directory".into());

        Ok(Path::new(target?))
    }

    pub fn mode(&self) -> Result<Mode, Error> {
        match self.matches.value_of("mode").unwrap_or_default() {
            "seq" => Ok(Mode::Seq),
            "fut" => Ok(Mode::Fut),
            "fut2" => Ok(Mode::Fut2),
            i => Err(format!("invalid mode {:?}", i).into()),
        }
    }
}
