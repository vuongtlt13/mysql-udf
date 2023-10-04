#![cfg(feature = "backend")]

mod backend;

use backend::get_db_connection;
use mysql::prelude::*;

const SETUP: &[&str] = &[
    "create or replace function blake2b512 returns string
        soname 'libudf_blake.so'",
    "create or replace function blake2s256 returns string
        soname 'libudf_blake.so'",
    "create or replace function blake3 returns string
        soname 'libudf_blake.so'",
];

const TEST: &str = "Hello, world!";
const PARTS: (&str, &str, &str) = ("Hello, ", "world", "!");
const RESULT_B512: &str = "A2764D133A16816B5847A737A786F2ECE4C148095C5FAA73E24B4CC5D666C3E4\
                           5EC271504E14DC6127DDFCE4E144FB23B91A6F7B04B53D695502290722953B0F";
const RESULT_S256: &str = "30D8777F0E178582EC8CD2FCDC18AF57C828EE2F89E978DF52C8E7AF078BD5CF";
const RESULT_3: &str = "EDE5C0B10F2EC4979C69B52F61E42FF5B413519CE09BE0F14D098DCFE5F6F98D";

#[test]
fn test_blake2b512() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select hex(blake2b512(?))", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_B512);

    let res: String = conn
        .exec_first("select hex(blake2b512(?, ?, ?))", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_B512);
}

#[test]
fn test_blake2s256() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select hex(blake2s256(?))", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_S256);

    let res: String = conn
        .exec_first("select hex(blake2s256(?, ?, ?))", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_S256);
}

#[test]
fn test_blake3() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select hex(blake3(?))", (TEST,))
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_3);

    let res: String = conn
        .exec_first("select hex(blake3(?, ?, ?))", PARTS)
        .unwrap()
        .unwrap();

    assert_eq!(res, RESULT_3);
}
