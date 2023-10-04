#![cfg(feature = "backend")]

mod backend;

use backend::get_db_connection;
use mysql::prelude::*;

const SETUP: &[&str] = &[
    "create or replace function xxhash3 returns integer
        soname 'libudf_xxhash.so'",
    "create or replace function xxhash32 returns integer
        soname 'libudf_xxhash.so'",
    "create or replace function xxhash64 returns integer
        soname 'libudf_xxhash.so'",
    "create or replace function xxhash returns integer
        soname 'libudf_xxhash.so'",
];

const TEST: &str = "Hello, world!";
const PARTS: (&str, &str, &str) = ("Hello, ", "world", "!");
const RESULT3: i64 = 0xf3c34bf11915e869_u64 as i64;
const RESULT32: u32 = 0x31b7405d;
const RESULT64: i64 = 0xf58336a78b6f9476_u64 as i64;

#[test]
fn test_xxhash3() {
    let conn = &mut get_db_connection(SETUP);

    let res: i64 = conn
        .exec_first("select xxhash3(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT3);

    let res: i64 = conn
        .exec_first("select xxhash3(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT3);
}

#[test]
fn test_xxhash32() {
    let conn = &mut get_db_connection(SETUP);

    let res: u32 = conn
        .exec_first("select xxhash32(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT32);

    let res: u32 = conn
        .exec_first("select xxhash32(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT32);
}

#[test]
fn test_xxhash64() {
    let conn = &mut get_db_connection(SETUP);

    let res: i64 = conn
        .exec_first("select xxhash64(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT64);

    let res: i64 = conn
        .exec_first("select xxhash64(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT64);

    // check the alias
    let res: i64 = conn
        .exec_first("select xxhash(?)", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT64);

    let res: i64 = conn
        .exec_first("select xxhash(?, ?, ?)", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT64);
}
