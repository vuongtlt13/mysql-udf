#![cfg(feature = "backend")]

mod backend;

use backend::get_db_connection;
use mysql::prelude::*;

const SETUP: &[&str] = &[
    "create or replace function blake2b512 returns string
        soname 'libudf_hash.so'",
    "create or replace function blake2b512_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function blake2s256 returns string
        soname 'libudf_hash.so'",
    "create or replace function blake2s256_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function blake3 returns string
        soname 'libudf_hash.so'",
    "create or replace function blake3_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function blake3_thd returns string
        soname 'libudf_hash.so'",
    "create or replace function blake3_thd_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function md5_u returns string
        soname 'libudf_hash.so'",
    "create or replace function md5_u_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha1_u returns string
        soname 'libudf_hash.so'",
    "create or replace function sha1_u_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha224 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha224_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha256 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha256_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha384 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha384_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha512 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha512_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function keccak224 returns string
        soname 'libudf_hash.so'",
    "create or replace function keccak224_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function keccak256 returns string
        soname 'libudf_hash.so'",
    "create or replace function keccak256_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_224 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_224_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_256 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_256_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_384 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_384_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_512 returns string
        soname 'libudf_hash.so'",
    "create or replace function sha3_512_bin returns string
        soname 'libudf_hash.so'",
    "create or replace function xxhash returns integer
        soname 'libudf_hash.so'",
    "create or replace function xxhash3 returns integer
        soname 'libudf_hash.so'",
    "create or replace function xxhash32 returns integer
        soname 'libudf_hash.so'",
    "create or replace function xxhash64 returns integer
        soname 'libudf_hash.so'",
];

const TEST: &str = "Hello, world!";
const PARTS: (&str, &str, &str) = ("Hello, ", "world", "!");

const RESULT_BLAKE2B512: &str = "A2764D133A16816B5847A737A786F2ECE4C148095C5FAA73E24B4CC5D666C3E4\
                                 5EC271504E14DC6127DDFCE4E144FB23B91A6F7B04B53D695502290722953B0F";
const RESULT_BLAKE2S256: &str = "30D8777F0E178582EC8CD2FCDC18AF57C828EE2F89E978DF52C8E7AF078BD5CF";
const RESULT_BLAKE3: &str = "EDE5C0B10F2EC4979C69B52F61E42FF5B413519CE09BE0F14D098DCFE5F6F98D";
const RESULT_MD5: &str = "6CD3556DEB0DA54BCA060B4C39479839";
const RESULT_SHA1: &str = "943A702D06F34599AEE1F8DA8EF9F7296031D699";
const RESULT_SHA224: &str = "8552D8B7A7DC5476CB9E25DEE69A8091290764B7F2A64FE6E78E9568";
const RESULT_SHA256: &str = "315F5BDB76D078C43B8AC0064E4A0164612B1FCE77C869345BFC94C75894EDD3";
const RESULT_SHA384: &str = "55BC556B0D2FE0FCE582BA5FE07BAAFFF035653638C7AC0D\
                             5494C2A64C0BEA1CC57331C7C12A45CDBCA7F4C34A089EEB";
const RESULT_SHA512: &str = "C1527CD893C124773D811911970C8FE6E857D6DF5DC9226BD8A160614C0CD963\
                             A4DDEA2B94BB7D36021EF9D865D5CEA294A82DD49A0BB269F51F6E7A57F79421";
const RESULT_KECCAK224: &str = "F89E15347FC711F25FC629F4BA60E3326643DC1DAF5AE9C04E86961D";
const RESULT_KECCAK256: &str = "B6E16D27AC5AB427A7F68900AC5559CE272DC6C37C82B3E052246C82244C50E4";
const RESULT_SHA3_224: &str = "6A33E22F20F16642697E8BD549FF7B759252AD56C05A1B0ACC31DC69";
const RESULT_SHA3_256: &str = "F345A219DA005EBE9C1A1EAAD97BBF38A10C8473E41D0AF7FB617CAA0C6AA722";
const RESULT_SHA3_384: &str = "6BA9EA268965916F5937228DDE678C202F9FE756A87D8B1B7\
                               362869583A45901FD1A27289D72FC0E3FF48B1B78827D3A";
const RESULT_SHA3_512: &str = "8E47F1185FFD014D238FABD02A1A32DEFE698CBF38C037A90E3C0A0A32370FB5\
                               2CBD641250508502295FCABCBF676C09470B27443868C8E5F70E26DC337288AF";
const RESULT_XXHASH3: i64 = 0xf3c34bf11915e869_u64 as i64;
const RESULT_XXHASH32: u32 = 0x31b7405d;
const RESULT_XXHASH64: i64 = 0xf58336a78b6f9476_u64 as i64;

macro_rules! make_hash_test {
    ($sql_fn:ident, $expected:ident) => {
        #[test]
        fn $sql_fn() {
            let conn = &mut get_db_connection(SETUP);
            let fn_name = stringify!($sql_fn);

            // Single result
            let res: String = conn
                .exec_first(&format!("select {fn_name}(?)"), (TEST,))
                .unwrap()
                .unwrap();

            assert_eq!(res, $expected);

            // Multiple arguments
            let res: String = conn
                .exec_first(&format!("select {fn_name}(?, ?, ?)"), PARTS)
                .unwrap()
                .unwrap();

            assert_eq!(res, $expected);

            // check the bin function
            let res: String = conn
                .exec_first(&format!("select hex({fn_name}_bin(?))"), (TEST,))
                .unwrap()
                .unwrap();

            assert_eq!(res, $expected);
        }
    };
}

make_hash_test!(blake2b512, RESULT_BLAKE2B512);
make_hash_test!(blake2s256, RESULT_BLAKE2S256);
make_hash_test!(blake3, RESULT_BLAKE3);
make_hash_test!(blake3_thd, RESULT_BLAKE3);
make_hash_test!(md5_u, RESULT_MD5);
make_hash_test!(sha1_u, RESULT_SHA1);
make_hash_test!(sha224, RESULT_SHA224);
make_hash_test!(sha256, RESULT_SHA256);
make_hash_test!(sha384, RESULT_SHA384);
make_hash_test!(sha512, RESULT_SHA512);
make_hash_test!(keccak224, RESULT_KECCAK224);
make_hash_test!(keccak256, RESULT_KECCAK256);
make_hash_test!(sha3_224, RESULT_SHA3_224);
make_hash_test!(sha3_256, RESULT_SHA3_256);
make_hash_test!(sha3_384, RESULT_SHA3_384);
make_hash_test!(sha3_512, RESULT_SHA3_512);

// xxhash uses integers so we can't use our macro

#[test]
fn test_xxhash3() {
    let conn = &mut get_db_connection(SETUP);

    let res: i64 = conn
        .exec_first("select xxhash3(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH3);

    let res: i64 = conn
        .exec_first("select xxhash3(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH3);
}

#[test]
fn test_xxhash32() {
    let conn = &mut get_db_connection(SETUP);

    let res: u32 = conn
        .exec_first("select xxhash32(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH32);

    let res: u32 = conn
        .exec_first("select xxhash32(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH32);
}

#[test]
fn test_xxhash64() {
    let conn = &mut get_db_connection(SETUP);

    let res: i64 = conn
        .exec_first("select xxhash64(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH64);

    let res: i64 = conn
        .exec_first("select xxhash64(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH64);

    // check the alias
    let res: i64 = conn
        .exec_first("select xxhash(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH64);

    let res: i64 = conn
        .exec_first("select xxhash(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_XXHASH64);
}
