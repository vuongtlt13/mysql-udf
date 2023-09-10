//! UUID <-> binary conversion
//!
//! We store our results in our structs to avoid some allocations

use udf::prelude::*;
use uuid::{Bytes as UuidBytes, Uuid};

use crate::{HYPHENATED_UUID_LEN, HYPHENATED_UUID_LEN_U64, UUID_BYTES_LEN, UUID_BYTES_LEN_U64};

#[derive(Debug, Default)]
struct UuidToBin(UuidBytes);

#[register(name = "uuid_to_bin")]
impl BasicUdf for UuidToBin {
    type Returns<'a> = Option<&'a [u8]>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        if !(args.len() == 1 || args.len() == 2) {
            return Err(format!(
                "uuid_to_bin takes one or two arguments but got {}",
                args.len()
            ));
        }

        if let Some(mut v) = args.get(0) {
            v.set_type_coercion(SqlType::String);
        }
        if let Some(mut v) = args.get(1) {
            v.set_type_coercion(SqlType::Int);
        }

        let ret = Self::default();
        cfg.set_max_len(UUID_BYTES_LEN_U64);

        Ok(ret)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let input = args.get(0).unwrap().value();
        let in_str = input.as_bytes().unwrap();
        let Ok(uuid) = Uuid::try_parse_ascii(in_str) else {
            return Ok(None);
        };

        let bytes = uuid.as_bytes();
        self.0.copy_from_slice(bytes);

        let should_swap = args
            .get(1)
            .map(|v| v.value().as_int().unwrap() != 0)
            .unwrap_or(false);

        if should_swap {
            swap_v1_time(&mut self.0);
        }

        Ok(Some(&self.0))
    }
}

#[derive(Debug)]
struct UuidFromBin([u8; HYPHENATED_UUID_LEN]);

#[register(name = "uuid_from_bin", alias = "bin_to_uuid")]
impl BasicUdf for UuidFromBin {
    type Returns<'a> = Option<&'a str>;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        if !(args.len() == 1 || args.len() == 2) {
            return Err(format!(
                "bin_to_uuid takes one or two arguments but got {}",
                args.len()
            ));
        }

        if let Some(mut v) = args.get(0) {
            v.set_type_coercion(SqlType::String);
        }
        if let Some(mut v) = args.get(1) {
            v.set_type_coercion(SqlType::Int);
        }
        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);

        Ok(Self([0u8; HYPHENATED_UUID_LEN]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let input = args.get(0).unwrap().value();
        let in_bytes = input.as_bytes().unwrap();
        let should_swap = args
            .get(1)
            .map(|v| v.value().as_int().unwrap() != 0)
            .unwrap_or(false);

        if in_bytes.len() != UUID_BYTES_LEN {
            return Ok(None);
        }

        let mut bytes = in_bytes;
        if should_swap {
            let tmp = &mut self.0[..UUID_BYTES_LEN];
            let tmp_buf: &mut [u8; 16] = tmp.try_into().unwrap();
            tmp_buf.copy_from_slice(in_bytes);
            unswap_v1_time(tmp_buf);
            bytes = tmp_buf;
        }

        let uuid = Uuid::from_slice(bytes).unwrap();
        let ret = uuid.hyphenated().encode_lower(&mut self.0);

        Ok(Some(ret))
    }
}

// Positions of components within a v1 UUID
const NODE: usize = 8;
const LOW_TIME: usize = 0;
const MID_TIME: usize = 4;
const HIGH_TIME: usize = 6;

const NEW_HIGH_TIME: usize = 0;
const NEW_MID_TIME: usize = 2;
const NEW_LOW_TIME: usize = 4;

// Swap the first group of 4 bits with the second group
fn swap_v1_time(uuid: &mut UuidBytes) {
    let mut tmp_low_time = [0u8; 4];

    tmp_low_time.copy_from_slice(&uuid[LOW_TIME..MID_TIME]);
    uuid.copy_within(MID_TIME..HIGH_TIME, NEW_MID_TIME);
    uuid.copy_within(HIGH_TIME..NODE, NEW_HIGH_TIME);
    uuid[NEW_LOW_TIME..NODE].copy_from_slice(&tmp_low_time);
}

// Swap the first group of 4 bits with the second group
fn unswap_v1_time(uuid: &mut UuidBytes) {
    let mut tmp_low_time = [0u8; 4];

    tmp_low_time.copy_from_slice(&uuid[NEW_LOW_TIME..NODE]);
    uuid.copy_within(NEW_HIGH_TIME..NEW_MID_TIME, HIGH_TIME);
    uuid.copy_within(NEW_MID_TIME..NEW_LOW_TIME, MID_TIME);
    uuid[LOW_TIME..MID_TIME].copy_from_slice(&tmp_low_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_swap() {
        const NORMAL: UuidBytes = hex_literal::hex!("6CCD780CBABA102695645B8C656024DB");
        const SWAPPED: UuidBytes = hex_literal::hex!("1026BABA6CCD780C95645B8C656024DB");

        let input = uuid::uuid!("6ccd780c-baba-1026-9564-5b8c656024db");
        let mut bytes = *input.as_bytes();

        assert_eq!(bytes, NORMAL);
        swap_v1_time(&mut bytes);
        assert_eq!(bytes, SWAPPED);
        unswap_v1_time(&mut bytes);
        assert_eq!(bytes, NORMAL);
    }
}
