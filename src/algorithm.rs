use std::io::{
	Read,
	Write,
	Result as IoResult,
	copy as io_copy,
};

use clap::ValueEnum;
use digest::{Digest, DynDigest};
use itertools::Itertools;

#[derive(
	Clone, Copy, Debug, Hash,
	PartialEq, Eq, PartialOrd, Ord,
	// clap traits
	ValueEnum
)]
pub enum Algorithm {
	#[cfg(feature = "algo-crc32")]
	Crc32,
	
	#[cfg(feature = "algo-md5")]
	Md5,
	
	#[cfg(feature = "algo-sha1")]
	Sha1,
	
	#[cfg(feature = "algo-sha2")]
	Sha256,
	#[cfg(feature = "algo-sha2")]
	Sha512,
	
	#[cfg(feature = "algo-sha3")]
	Sha3_224,
	#[cfg(feature = "algo-sha3")]
	Sha3_256,
	#[cfg(feature = "algo-sha3")]
	Sha3_384,
	#[cfg(feature = "algo-sha3")]
	Sha3_512,
}

impl Algorithm {
	pub fn get_checksum_string<R: Read>(
		&self,
		data: &mut R
	) -> IoResult<String> {
		match self {
			#[cfg(feature = "algo-crc32")]
			Self::Crc32 => self.checksum_crc32(data),
			
			#[cfg(feature = "algo-md5")]
			Self::Md5 => self.checksum_digest(data),
			
			#[cfg(feature = "algo-sha1")]
			Self::Sha1 => self.checksum_digest(data),
			
			#[cfg(feature = "algo-sha2")]
			Self::Sha256 | Self::Sha512 => self.checksum_digest(data),
			
			#[cfg(feature = "algo-sha3")]
			Self::Sha3_224 | Self::Sha3_256
			| Self::Sha3_384 | Self::Sha3_512 => self.checksum_digest(data),
		}
	}
	
	// -------- DIGEST ALGORITHMS -------- //
	fn checksum_digest<R: Read>(
		&self,
		data: &mut R
	) -> IoResult<String> {
		let hash = {
			let mut digester = self.get_as_digest_trait();
			io_copy(data, &mut digester)?;
			digester.finalize()
		};
		
		Ok(format!("{:02x}", hash.iter().format("")))
	}
	
	// TODO: Maybe it would be easier to write a wrapper over `T: Default +
	// Write`? That would cover all the `digest` crate impls, plus `crc32fast`,
	// simplifying our code in the process. However, this needs investigation
	// because I'm not sure how it would affect algorithms that have
	// user-defined output sizes (i.e. `blake2`)
	fn get_as_digest_trait(&self) -> Box<dyn DynDigestAndWrite> {
		match self {
			#[cfg(feature = "algo-md5")]
			Self::Md5 => Box::new(md5::Md5::new()),
			
			#[cfg(feature = "algo-sha1")]
			Self::Sha1 => Box::new(sha1::Sha1::new()),
			
			#[cfg(feature = "algo-sha2")]
			Self::Sha256 => Box::new(sha2::Sha256::new()),
			#[cfg(feature = "algo-sha2")]
			Self::Sha512 => Box::new(sha2::Sha512::new()),
			
			#[cfg(feature = "algo-sha3")]
			Self::Sha3_224 => Box::new(sha3::Sha3_224::new()),
			#[cfg(feature = "algo-sha3")]
			Self::Sha3_256 => Box::new(sha3::Sha3_256::new()),
			#[cfg(feature = "algo-sha3")]
			Self::Sha3_384 => Box::new(sha3::Sha3_384::new()),
			#[cfg(feature = "algo-sha3")]
			Self::Sha3_512 => Box::new(sha3::Sha3_512::new()),
			
			_ => unreachable!(),
		}
	}
	
	// -------- NON-DIGEST ALGORITHMS -------- //
	fn checksum_crc32<R: Read>(
		&self,
		data: &mut R
	) -> IoResult<String> {
		// ----- Write wrapper to help with using CRC32 ----- //
		// Aside: This wrapper is actually inspired by how the `digest` crate
		// implements the `Write` trait for use with `std::io::copy()`. In fact,
		// the `Write` impl is nearly identical when read side-by-side. So thank
		// you, `digest` crate maintainers, and I hope you don't mind me ripping
		// off your code. :P
		//
		// `digest`'s implementation, for comparison:
		// <https://docs.rs/digest/0.10.7/src/digest/core_api/wrapper.rs.html#245-261>
		#[derive(Default)]
		struct Crc32Writer(crc32fast::Hasher);
		
		impl Crc32Writer {
			fn finalize(self) -> u32 { self.0.finalize() }
		}
		
		impl Write for Crc32Writer {
			fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
				self.0.update(buf);
				Ok(buf.len())
			}
			
			fn flush(&mut self) -> IoResult<()> {
				Ok(())
			}
		}
		
		// ----- ACTUAL FUNCTION STARTS HERE ----- //
		let hash = {
			let mut digester = Crc32Writer::default();
			io_copy(data, &mut digester)?;
			digester.finalize()
		};
		
		Ok(format!("{hash:08x}"))
	}
}

// Trait impl for anything that implements DynDigest and Write, since we can't
// write `Box<dyn DynDigest + Write>` above
trait DynDigestAndWrite: DynDigest + Write {}
impl<T> DynDigestAndWrite for T
where
	T: DynDigest + Write {}
