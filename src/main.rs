use std::{
	fs::File,
	io::Result as IoResult,
	path::PathBuf,
};

use clap::Parser;

use self::algorithm::Algorithm;

mod algorithm;

#[derive(
	Clone, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
	// clap traits
	Parser,
)]
#[command(
	version,
	about, long_about = None,
)]
struct CliArgs {
	/// The algorithm to run over the specified files.
	#[arg(ignore_case = true)]
	algorithm: Algorithm,
	
	/// The files to generate checksums for.
	#[arg(required = true)]
	file_paths: Vec<PathBuf>,
}

#[cfg(not(any(
	feature = "algo-crc32",
	feature = "algo-md5",
	feature = "algo-sha1",
	feature = "algo-sha2",
	feature = "algo-sha3",
)))]
compile_error!("At least one algorithm feature must be enabled.");

fn main() -> IoResult<()> {
	let args = CliArgs::parse();
	
	//dbg!(&args);
	
	for file_path in args.file_paths {
		// Errors that could occur:
		// * File not found
		let mut file = File::open(&file_path)?;
		
		let hex_hash = args.algorithm.get_checksum_string(&mut file)?;
		
		println!(
			"{hex_hash} *{}",
			file_path.display()
		);
	}
	
	Ok(())
}
