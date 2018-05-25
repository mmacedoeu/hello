extern crate vergen;
extern crate rustc_version;

use vergen::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
	vergen(OutputFns::all()).unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("rustc_version.rs");
	let mut f = File::create(&dest_path).unwrap();
	f.write_all(format!("
		/// Returns compiler version.
		pub fn rustc_version() -> &'static str {{
			\"{}\"
		}}
	", rustc_version::version()).as_bytes()).unwrap();
}
