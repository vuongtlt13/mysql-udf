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
use xxhash_rust::xxh3::{xxh3_64, Xxh3};
use xxhash_rust::xxh32::{xxh32, Xxh32};
use xxhash_rust::xxh64::{xxh64, Xxh64};

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
