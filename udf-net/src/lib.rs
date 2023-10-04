//! Blake2 and Blake3 hash algorithms
//!
//! # Usage
//!
//! ```sql
//! CREATE FUNCTION ip_validate RETURNS string SONAME 'libudf_net.so';
//! CREATE FUNCTION ip_to_canonical RETURNS string SONAME 'libudf_net.so';
//! CREATE FUNCTION ip_to_ipv6_mapped RETURNS string SONAME 'libudf_net.so';
//! ```

use std::fmt::Write;
use std::net::IpAddr;

use udf::prelude::*;

struct IpValidate;
struct IpToCanonical(String);
struct IpToIpv6Mapped(String);

#[register(name = "ip_validate")]
impl BasicUdf for IpValidate {
    type Returns<'a> = Option<&'a str>;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        verify_one_string_arg(args)?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let value = args.get(0).unwrap().value();
        let res = match value.as_string().map(|v| v.parse::<IpAddr>()) {
            Some(Ok(IpAddr::V4(_))) => Some("ipv4"),
            Some(Ok(IpAddr::V6(_))) => Some("ipv6"),
            Some(Err(_)) | None => None,
        };
        Ok(res)
    }
}

#[register(name = "ip_to_canonical")]
impl BasicUdf for IpToCanonical {
    type Returns<'a> = Option<&'a str>;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        verify_one_string_arg(args)?;
        Ok(Self(String::new()))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        self.0.clear();
        let value = args.get(0).unwrap().value();
        let Some(s) = value.as_string() else {
            return Ok(None);
        };
        match s.parse::<IpAddr>() {
            Ok(IpAddr::V4(ip)) => write!(self.0, "{ip}"),
            Ok(IpAddr::V6(ip)) => {
                if let Some(mapped) = ip.to_ipv4_mapped() {
                    write!(self.0, "{mapped}")
                } else {
                    write!(self.0, "{ip}")
                }
            }
            Err(_) => return Ok(None),
        }
        .unwrap();
        Ok(Some(&self.0))
    }
}

#[register(name = "ip_to_ipv6_mapped")]
impl BasicUdf for IpToIpv6Mapped {
    type Returns<'a> = Option<&'a str>;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        verify_one_string_arg(args)?;
        Ok(Self(String::new()))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        self.0.clear();
        let value = args.get(0).unwrap().value();
        let Some(s) = value.as_string() else {
            return Ok(None);
        };
        let res = match s.parse::<IpAddr>() {
            Ok(IpAddr::V4(ip)) => ip.to_ipv6_mapped(),
            Ok(IpAddr::V6(ip)) => ip,
            Err(_) => return Ok(None),
        };
        write!(self.0, "{res}").unwrap();
        Ok(Some(&self.0))
    }
}

/// Helper to make sure we have one argument, then set it to type string
fn verify_one_string_arg(args: &ArgList<Init>) -> Result<(), String> {
    if args.len() != 1 {
        return Err(format!("expected one argument but got {}", args.len()));
    }
    args.get(0).unwrap().set_type_coercion(SqlType::String);
    Ok(())
}
