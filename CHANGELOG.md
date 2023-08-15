# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- MD5 checksum generation, via the `md-5` crate.
- SHA1 checksum generation, via the `sha1` crate.
- SHA2 checksum generation, via the `sha2` crate.
- SHA3 checksum generation, via the `sha3` crate. Only includes the algorithms
  listed as `Sha3_*` in that crate.
- CRC32 checksum generation, via the `crc32fast` crate.
