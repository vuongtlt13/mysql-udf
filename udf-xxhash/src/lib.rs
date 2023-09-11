//! A function to generate lipsum of a given word count
//!
//! # Usage
//!
//! ```sql
//! CREATE FUNCTION xxhash3 RETURNS integer SONAME 'libudf_xxhash.so';
//! CREATE FUNCTION xxhash32 RETURNS integer SONAME 'libudf_xxhash.so';
//! CREATE FUNCTION xxhash64 RETURNS integer SONAME 'libudf_xxhash.so';
//! CREATE FUNCTION xxhash RETURNS integer SONAME 'libudf_xxhash.so';
//!
//! SELECT xxhash3('Hello world!');
//! SELECT xxhash32('Hello world!');
//! SELECT xxhash64('Hello world!');
//! ```

use udf::prelude::*;
use xxhash_rust::xxh3::Xxh3;
use xxhash_rust::xxh32::Xxh32;
use xxhash_rust::xxh64::Xxh64;

/// We will reuse our state between runs to save a few cycles
struct XxHash3(Xxh3);
struct XxHash32(Xxh32);
struct XxHash64(Xxh64);

#[register(name = "xxhash3")]
impl BasicUdf for XxHash3 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self(Xxh3::new()))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let hasher = &mut self.0;

        hash_args(args, |buf| hasher.update(buf));

        let result = hasher.digest();
        hasher.reset();

        Ok(result as i64)
    }
}

#[register(name = "xxhash32")]
impl BasicUdf for XxHash32 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self(Xxh32::new(0)))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let hasher = &mut self.0;

        hash_args(args, |buf| hasher.update(buf));

        let result = hasher.digest();
        hasher.reset(0);

        Ok(result.into())
    }
}

#[register(name = "xxhash64", alias = "xxhash")]
impl BasicUdf for XxHash64 {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, _args: &ArgList<Init>) -> Result<Self, String> {
        Ok(Self(Xxh64::new(0)))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let hasher = &mut self.0;

        hash_args(args, |buf| hasher.update(buf));

        let result = hasher.digest();
        hasher.reset(0);

        Ok(result as i64)
    }
}

/// Hash all arguments to a hasher function
fn hash_args(args: &ArgList<Process>, mut update_fn: impl FnMut(&[u8])) {
    for arg in args {
        // Any non-null value will update the hash, null values do nothing.
        match arg.value() {
            SqlResult::String(Some(buf)) => update_fn(buf),
            SqlResult::Real(Some(f)) => update_fn(&f.to_le_bytes()),
            SqlResult::Int(Some(i)) => update_fn(&i.to_le_bytes()),
            SqlResult::Decimal(Some(d)) => update_fn(d.as_bytes()),
            _ => update_fn([].as_slice()),
        }
    }
}
