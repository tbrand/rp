extern crate clap;

use clap::Arg;

pub fn r<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("recursive")
        .help("Copy all files recursively")
        .short("r")
}

pub fn f<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("force")
        .help("Overwrite copied file")
        .short("f")
}

pub fn mode<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("mode")
        .help("Execution mode (seq, con, par)")
        .short("m")
        .long("mode")
        .takes_value(true)
        .default_value("fut2")
}

pub fn files<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("files")
        .help("Source files or directory to be copied.")
        .multiple(true)
}
