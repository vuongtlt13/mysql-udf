#![cfg(feature = "backend")]

mod backend;

// use backend::get_db_connection;
use backend::get_db_connection;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Binary, Integer, Text};
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

    let res: String = sql::<Text>("select uuid_nil()").get_result(conn).unwrap();

    assert_eq!(res, "00000000-0000-0000-0000-000000000000");
    assert_eq!(res, Uuid::nil().hyphenated().to_string());
}

#[test]
fn test_max() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_max()").get_result(conn).unwrap();

    assert_eq!(res, "ffffffff-ffff-ffff-ffff-ffffffffffff");
    assert_eq!(res, Uuid::max().hyphenated().to_string());
}

#[test]
fn test_ns_dns() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_ns_dns()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_DNS.hyphenated().to_string());
}

#[test]
fn test_ns_url() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_ns_url()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res, "6ba7b811-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_URL.hyphenated().to_string());
}

#[test]
fn test_ns_oid() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_ns_oid()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res, "6ba7b812-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_OID.hyphenated().to_string());
}

#[test]
fn test_ns_x500() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_ns_x500()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res, "6ba7b814-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res, Uuid::NAMESPACE_X500.hyphenated().to_string());
}

#[test]
fn test_generate_v1() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_generate_v1()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 1);
}

#[test]
fn test_generate_v1mc() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_generate_v1mc()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 1);
}

#[test]
fn test_generate_v4() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_generate_v4()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 4);
}

#[test]
fn test_generate_v6() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_generate_v6()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 6);

    let node_id = "abcdef";
    let res: String = sql::<Text>("select uuid_generate_v6(")
        .bind::<Text, _>(node_id)
        .sql(")")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.as_str()).unwrap();

    assert_eq!(uuid.get_version_num(), 6);
    assert!(uuid.as_bytes().ends_with(node_id.as_bytes()));
}

#[test]
fn test_generate_v7() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = sql::<Text>("select uuid_generate_v7()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(&res).unwrap();

    assert_eq!(uuid.get_version_num(), 7);
}

#[test]
fn test_valid() {
    let conn = &mut get_db_connection(SETUP);

    let res: i32 = sql::<Integer>("select uuid_is_valid(uuid_generate_v4())")
        .get_result(conn)
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

        let u2b_res: Vec<u8> = sql::<Binary>(&format!("select uuid_to_bin('{INPUT}')"))
            .get_result(conn)
            .unwrap();

        assert_eq!(u2b_res, NORMAL);

        let u2b_swp_res: Vec<u8> = sql::<Binary>(&format!("select uuid_to_bin('{INPUT}', true)"))
            .get_result(conn)
            .unwrap();

        assert_eq!(u2b_swp_res, SWAPPED);

        let b2u_res: String = sql::<Text>(&format!(
            "select {from_fn}(unhex('{}'))",
            hex::encode(NORMAL)
        ))
        .get_result(conn)
        .unwrap();

        assert_eq!(b2u_res, INPUT);

        let b2u_swp_res: String = sql::<Text>(&format!(
            "select {from_fn}(unhex('{}'), true)",
            hex::encode(SWAPPED)
        ))
        .get_result(conn)
        .unwrap();

        assert_eq!(b2u_swp_res, INPUT);

        let roundtrip: String = sql::<Text>(&format!("select {from_fn}(uuid_to_bin('{INPUT}'))"))
            .get_result(conn)
            .unwrap();

        assert_eq!(roundtrip, INPUT);

        let roundtrip_swp: String =
            sql::<Text>(&format!("select {from_fn}(uuid_to_bin('{INPUT}', 1), 1)"))
                .get_result(conn)
                .unwrap();

        assert_eq!(roundtrip_swp, INPUT);
    }
}
