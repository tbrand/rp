extern crate futures;
extern crate futures_fs;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rp_error;
extern crate actix;
#[macro_use]
extern crate lazy_static;

use rp_error::Result;
use std::fs;
use std::path::Path;

pub mod fut;
pub mod fut2;
pub mod seq;

pub trait Copier {
    fn file_to_file(src: &Path, target: &Path) -> Result<u64>;
    fn dir_to_dir(src: &Path, target: &Path) -> Result<u64>;
    fn copy(src: &Path, target: &Path) -> Result<u64> {
        fs::copy(src, target).map_err(Into::into)
    }
}

pub mod prelude {
    pub use fut::Fut;
    pub use fut2::Fut2;
    pub use seq::Seq;
}
