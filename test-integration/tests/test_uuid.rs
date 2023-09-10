#![cfg(feature = "backend")]

mod backend;

use backend::get_db_connection;
use mysql::prelude::*;
use uuid::{Bytes as UuidBytes, Uuid};

const SETUP: &[&str] = &[
    "create or replace function uuid_generate_v1
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v1mc
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v4
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v6
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v7
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_to_bin
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_from_bin
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function bin_to_uuid
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_nil
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_max
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_dns
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_url
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_oid
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_x500
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_is_valid
        returns integer
        soname 'libudf_uuid.so'",
];

#[test]
fn test_nil() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_nil()").unwrap().unwrap();

    assert_eq!(res, "00000000-0000-0000-0000-000000000000");
    assert_eq!(res, Uuid::nil().hyphenated().to_string());
}

#[test]
fn test_max() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_max()").unwrap().unwrap();

    assert_eq!(res, "ffffffff-ffff-ffff-ffff-ffffffffffff");
    assert_eq!(res, Uuid::max().hyphenated().to_string());
}

#[test]
fn test_ns_dns() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_ns_dns()").unwrap().unwrap();

    assert_eq!(res, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_DNS.hyphenated().to_string());
}

#[test]
fn test_ns_url() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_ns_url()").unwrap().unwrap();

    assert_eq!(res, "6ba7b811-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_URL.hyphenated().to_string());
}

#[test]
fn test_ns_oid() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_ns_oid()").unwrap().unwrap();

    assert_eq!(res, "6ba7b812-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_OID.hyphenated().to_string());
}

#[test]
fn test_ns_x500() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn.query_first("select uuid_ns_x500()").unwrap().unwrap();

    assert_eq!(res, "6ba7b814-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_X500.hyphenated().to_string());
}

#[test]
fn test_generate_v1() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .query_first("select uuid_generate_v1()")
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 1);
}

#[test]
fn test_generate_v1mc() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .query_first("select uuid_generate_v1mc()")
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 1);
}

#[test]
fn test_generate_v4() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .query_first("select uuid_generate_v4()")
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 4);
}

#[test]
fn test_generate_v6() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .query_first("select uuid_generate_v6()")
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 6);

    let node_id = "abcdef";
    let res: String = conn
        .exec_first("select uuid_generate_v6(?)", (node_id,))
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(res.as_str()).unwrap();

    assert_eq!(uuid.get_version_num(), 6);
    assert!(uuid.as_bytes().ends_with(node_id.as_bytes()));
}

#[test]
fn test_generate_v7() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .query_first("select uuid_generate_v7()")
        .unwrap()
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 7);
}

#[test]
fn test_valid() {
    let conn = &mut get_db_connection(SETUP);

    let res: i32 = conn
        .query_first("select uuid_is_valid(uuid_generate_v4())")
        .unwrap()
        .unwrap();

    assert_eq!(res, 1);
}

const INPUT: &str = "6ccd780c-baba-1026-9564-5b8c656024db";
const NORMAL: UuidBytes = hex_literal::hex!("6CCD780CBABA102695645B8C656024DB");
const SWAPPED: UuidBytes = hex_literal::hex!("1026BABA6CCD780C95645B8C656024DB");

#[test]
fn test_uuid_to_from_bin() {
    // test everything with both functions
    for from_fn in ["uuid_from_bin", "bin_to_uuid"] {
        eprintln!("testing with '{from_fn}'");

        let conn = &mut get_db_connection(SETUP);

        let u2b_res: Vec<u8> = conn
            .exec_first("select uuid_to_bin(?)", (INPUT,))
            .unwrap()
            .unwrap();

        assert_eq!(u2b_res, NORMAL);

        let u2b_swp_res: Vec<u8> = conn
            .exec_first("select uuid_to_bin(?, true)", (INPUT,))
            .unwrap()
            .unwrap();

        assert_eq!(u2b_swp_res, SWAPPED);

        let b2u_res: String = conn
            .exec_first(
                &format!("select {from_fn}(unhex(?))"),
                (hex::encode(NORMAL),),
            )
            .unwrap()
            .unwrap();

        assert_eq!(b2u_res, INPUT);

        let b2u_swp_res: String = conn
            .exec_first(
                &format!("select {from_fn}(unhex(?), true)"),
                (hex::encode(SWAPPED),),
            )
            .unwrap()
            .unwrap();

        assert_eq!(b2u_swp_res, INPUT);

        let roundtrip: String = conn
            .exec_first(&format!("select {from_fn}(uuid_to_bin(?))"), (INPUT,))
            .unwrap()
            .unwrap();

        assert_eq!(roundtrip, INPUT);

        let roundtrip_swp: String = conn
            .exec_first(&format!("select {from_fn}(uuid_to_bin(?, 1), 1)"), (INPUT,))
            .unwrap()
            .unwrap();

        assert_eq!(roundtrip_swp, INPUT);
    }
}
