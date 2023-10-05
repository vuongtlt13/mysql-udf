//! Various hash algorithms that return binary and hex
//!
//! # Usage
//!
//! ```sql
//! CREATE FUNCTION blake2b512 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION blake2b512_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION blake2s256 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION blake2s256_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION blake3 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha1_u RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha1_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION md5_u RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION md5_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha224 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha224_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha256 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha256_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha384 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha384_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha512 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha512_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION keccak224 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION keccak224_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION keccak256 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION keccak256_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_224 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_224_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_256 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_256_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_384 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_384_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_512 RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION sha3_512_hex RETURNS string SONAME 'libudf_hash.so';
//! CREATE FUNCTION xxhash RETURNS integer SONAME 'libudf_hash.so';
//! CREATE FUNCTION xxhash3 RETURNS integer SONAME 'libudf_hash.so';
//! CREATE FUNCTION xxhash32 RETURNS integer SONAME 'libudf_hash.so';
//! CREATE FUNCTION xxhash64 RETURNS integer SONAME 'libudf_hash.so';
//! ```

use digest::Digest;
use udf::prelude::*;
use xxhash_rust::xxh3::{xxh3_64, Xxh3};
use xxhash_rust::xxh32::{xxh32, Xxh32};
use xxhash_rust::xxh64::{xxh64, Xxh64};

// Make a simple UDF for anything that uses the `Digest` interface
macro_rules! digest_udf {
    ($hash_ty:ty, $fn_name:ident, $bin_fn_name:ident, $hash_len:expr) => {
        // Provide an implementation that returns a hexified string
        #[allow(non_camel_case_types)]
        struct $fn_name {
            /// The hasher is stored so we can create it once then reset it on each call.
            hasher: $hash_ty,
            /// Store our hashed value
            hashed: [u8; $hash_len],
            /// Output for our hexified values
            hex: [u8; $hash_len * 2],
        }

        #[register]
        impl BasicUdf for $fn_name {
            type Returns<'a> = &'a [u8];

            fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
                let ret = Self {
                    hasher: <$hash_ty as digest::Digest>::new(),
                    hashed: [0u8; $hash_len],
                    hex: [0u8; $hash_len * 2],
                };
                Ok(ret)
            }

            fn process<'a>(
                &'a mut self,
                _cfg: &UdfCfg<Process>,
                args: &ArgList<Process>,
                _error: Option<NonZeroU8>,
            ) -> Result<Self::Returns<'a>, ProcessError> {
                args.iter()
                    .for_each(|arg| hash_arg(arg, |buf| self.hasher.update(buf)));
                self.hasher.finalize_into_reset((&mut self.hashed).into());
                // Encode our data to hex
                data_encoding::HEXUPPER.encode_mut(&self.hashed, &mut self.hex);
                Ok(&self.hex)
            }
        }

        // Provide an implementation that returns a binary string
        #[allow(non_camel_case_types)]
        struct $bin_fn_name {
            /// The hasher is stored so we can create it once then reset it on each call.
            /// This seems like it is
            hasher: $hash_ty,
            /// Store our return value, we can return a reference to it without allocating
            ret: [u8; $hash_len],
        }

        #[register]
        impl BasicUdf for $bin_fn_name {
            type Returns<'a> = &'a [u8];

            fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
                let ret = Self {
                    hasher: <$hash_ty as digest::Digest>::new(),
                    ret: [0u8; $hash_len],
                };
                Ok(ret)
            }

            fn process<'a>(
                &'a mut self,
                _cfg: &UdfCfg<Process>,
                args: &ArgList<Process>,
                _error: Option<NonZeroU8>,
            ) -> Result<Self::Returns<'a>, ProcessError> {
                args.iter()
                    .for_each(|arg| hash_arg(arg, |buf| self.hasher.update(buf)));
                self.hasher.finalize_into_reset((&mut self.ret).into());
                Ok(&self.ret)
            }
        }
    };
}

digest_udf!(blake2::Blake2b512, blake2b512, blake2b512_bin, 64);
digest_udf!(blake2::Blake2s256, blake2s256, blake2s256_bin, 32);
digest_udf!(sha1::Sha1, sha1_u, sha1_u_bin, 20);
digest_udf!(md5::Md5, md5_u, md5_u_bin, 16);
digest_udf!(sha2::Sha224, sha224, sha224_bin, 28);
digest_udf!(sha2::Sha256, sha256, sha256_bin, 32);
digest_udf!(sha2::Sha384, sha384, sha384_bin, 48);
digest_udf!(sha2::Sha512, sha512, sha512_bin, 64);
digest_udf!(sha3::Keccak224, keccak224, keccak224_bin, 28);
digest_udf!(sha3::Keccak256, keccak256, keccak256_bin, 32);
digest_udf!(sha3::Sha3_224, sha3_224, sha3_224_bin, 28);
digest_udf!(sha3::Sha3_256, sha3_256, sha3_256_bin, 32);
digest_udf!(sha3::Sha3_384, sha3_384, sha3_384_bin, 48);
digest_udf!(sha3::Sha3_512, sha3_512, sha3_512_bin, 64);

// Blake3 is special and doesn't implement `Digest` :). We also provide a threaded
// implementation.
struct Blake3 {
    hasher: blake3::Hasher,
    hashed: [u8; 32],
    hex: [u8; 64],
}

#[register(name = "blake3")]
impl BasicUdf for Blake3 {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake3::Hasher::new(),
            hashed: [0u8; 32],
            hex: [0u8; 64],
        };
        Ok(ret)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        args.iter().for_each(|arg| {
            hash_arg(arg, |buf| {
                self.hasher.update(buf);
            })
        });
        let hash = self.hasher.finalize();
        self.hashed = hash.into();
        self.hasher.reset();
        data_encoding::HEXUPPER.encode_mut(&self.hashed, &mut self.hex);
        Ok(&self.hex)
    }
}

struct Blake3Bin {
    hasher: blake3::Hasher,
    ret: [u8; 32],
}

#[register(name = "blake3_bin")]
impl BasicUdf for Blake3Bin {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake3::Hasher::new(),
            ret: [0u8; 32],
        };
        Ok(ret)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        args.iter().for_each(|arg| {
            hash_arg(arg, |buf| {
                self.hasher.update(buf);
            })
        });
        let hash = self.hasher.finalize();
        self.ret = hash.into();
        self.hasher.reset();
        Ok(&self.ret)
    }
}

struct Blake3Thd {
    hasher: blake3::Hasher,
    hashed: [u8; 32],
    hex: [u8; 64],
}

#[register(name = "blake3_thd")]
impl BasicUdf for Blake3Thd {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake3::Hasher::new(),
            hashed: [0u8; 32],
            hex: [0u8; 64],
        };
        Ok(ret)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        args.iter().for_each(|arg| {
            hash_arg(arg, |buf| {
                self.hasher.update_rayon(buf);
            })
        });
        let hash = self.hasher.finalize();
        self.hashed = hash.into();
        self.hasher.reset();
        data_encoding::HEXUPPER.encode_mut(&self.hashed, &mut self.hex);
        Ok(&self.hex)
    }
}

struct Blake3ThdBin {
    hasher: blake3::Hasher,
    ret: [u8; 32],
}

#[register(name = "blake3_thd_bin")]
impl BasicUdf for Blake3ThdBin {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake3::Hasher::new(),
            ret: [0u8; 32],
        };
        Ok(ret)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        args.iter().for_each(|arg| {
            hash_arg(arg, |buf| {
                self.hasher.update_rayon(buf);
            })
        });
        let hash = self.hasher.finalize();
        self.ret = hash.into();
        self.hasher.reset();
        Ok(&self.ret)
    }
}

struct XxHash3;
struct XxHash32;
struct XxHash64;

#[register(name = "xxhash3")]
impl BasicUdf for XxHash3 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        if args.len() == 1 {
            Ok(hash_arg(args.get(0).unwrap(), xxh3_64) as i64)
        } else {
            let mut hasher = Xxh3::new();
            args.iter()
                .for_each(|arg| hash_arg(arg, |buf| hasher.update(buf)));
            Ok(hasher.digest() as i64)
        }
    }
}

#[register(name = "xxhash32")]
impl BasicUdf for XxHash32 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        if args.len() == 1 {
            Ok(hash_arg(args.get(0).unwrap(), |buf| xxh32(buf, 0)).into())
        } else {
            let mut hasher = Xxh32::new(0);
            args.iter()
                .for_each(|arg| hash_arg(arg, |buf| hasher.update(buf)));
            Ok(hasher.digest() as i64)
        }
    }
}

#[register(name = "xxhash64", alias = "xxhash")]
impl BasicUdf for XxHash64 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        if args.len() == 1 {
            Ok(hash_arg(args.get(0).unwrap(), |buf| xxh64(buf, 0)) as i64)
        } else {
            let mut hasher = Xxh64::new(0);
            args.iter()
                .for_each(|arg| hash_arg(arg, |buf| hasher.update(buf)));
            Ok(hasher.digest() as i64)
        }
    }
}

/// Turn a SQL argument into a hashable buffer and pass it the given function
fn hash_arg<T>(arg: SqlArg<Process>, mut hash_fn: impl FnMut(&[u8]) -> T) -> T {
    // Any non-null value will update the hash, null values do nothing.
    match arg.value() {
        SqlResult::String(Some(buf)) => hash_fn(buf),
        SqlResult::Real(Some(f)) => hash_fn(&f.to_le_bytes()),
        SqlResult::Int(Some(i)) => hash_fn(&i.to_le_bytes()),
        SqlResult::Decimal(Some(d)) => hash_fn(d.as_bytes()),
        _ => hash_fn([].as_slice()),
    }
}
