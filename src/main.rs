/*
 * slicer: Better slicing for multi-color extrusion.
 * Copyright (C) 2017  Terrence Cole
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
extern crate libstl;
extern crate clap;
#[macro_use]
extern crate error_chain;

use clap::{Arg, App};
use libstl::Mesh;
use std::fs::File;

mod errors {
    error_chain! {}
}
use ::errors::{Result};

quick_main!(run);
fn run() -> Result<()> {
    let matches = App::new("particolor")
        .version("0.1")
        .author("Terrence Cole")
        .about("Split a multi-color STL into separate STL files for printing")
        .arg(Arg::with_name("INPUT")
            .help("The file to split.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let verbose = matches.occurrences_of("v") > 0;
    let input = matches.value_of("INPUT").expect("no input");

    if verbose {
        println!("Reading: {}", input);
    }
    let mut fp = File::open(input).expect("no such file");
    let m = Mesh::from_file(&mut fp).expect("not an stl file");


    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(4, 2 + 2);
    }
}
