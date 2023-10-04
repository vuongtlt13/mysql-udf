//! Blake2 and Blake3 hash algorithms
//!
//! # Usage
//!
//! ```sql
//! CREATE FUNCTION blake2b512 RETURNS string SONAME 'libudf_blake.so';
//! CREATE FUNCTION blake2s256 RETURNS string SONAME 'libudf_blake.so';
//! CREATE FUNCTION blake3 RETURNS string SONAME 'libudf_blake.so';
//!
//! SELECT blake2b512('Hello world!');
//! SELECT blake2s256('Hello world!');
//! SELECT blake3('Hello world!');
//! ```

use blake2::Digest;
use udf::prelude::*;

// We store our hashers to save a small bit of time
struct Blake2b512 {
    hasher: blake2::Blake2b512,
    ret: [u8; 64],
}

struct Blake2s256 {
    hasher: blake2::Blake2s256,
    ret: [u8; 32],
}

struct Blake3 {
    hasher: blake3::Hasher,
    ret: [u8; 32],
}

#[register(name = "blake2b512")]
impl BasicUdf for Blake2b512 {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake2::Blake2b512::new(),
            ret: [0u8; 64],
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

#[register(name = "blake2s256")]
impl BasicUdf for Blake2s256 {
    type Returns<'a> = &'a [u8];

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        let ret = Self {
            hasher: blake2::Blake2s256::new(),
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
        args.iter()
            .for_each(|arg| hash_arg(arg, |buf| self.hasher.update(buf)));
        self.hasher.finalize_into_reset((&mut self.ret).into());
        Ok(&self.ret)
    }
}

#[register(name = "blake3")]
impl BasicUdf for Blake3 {
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
