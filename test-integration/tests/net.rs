#![cfg(feature = "backend")]

mod backend;

use backend::get_db_connection;
use mysql::prelude::*;

const SETUP: &[&str] = &[
    "create or replace function ip_validate returns string
        soname 'libudf_net.so'",
    "create or replace function ip_to_canonical returns string
        soname 'libudf_net.so'",
    "create or replace function ip_to_ipv6_mapped returns string
        soname 'libudf_net.so'",
];

const T1: &str = "127.0.0.1";
const T2: &str = "::ffff:127.0.0.1";
const T3: &str = "2001:db8::1:0:0:1";
const T4: &str = "bad";

#[test]
fn test_ip_validate() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select ip_validate(?)", (T1,))
        .unwrap()
        .unwrap();

    assert_eq!(res, "ipv4");

    let res: String = conn
        .exec_first("select ip_validate(?)", (T2,))
        .unwrap()
        .unwrap();

    assert_eq!(res, "ipv6");

    let res: String = conn
        .exec_first("select ip_validate(?)", (T3,))
        .unwrap()
        .unwrap();

    assert_eq!(res, "ipv6");

    let res: Option<String> = conn
        .exec_first("select ip_validate(?)", (T4,))
        .unwrap()
        .unwrap();

    assert_eq!(res, None);
}

#[test]
fn test_ip_to_ipv6_mapped() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select ip_to_ipv6_mapped(?)", (T1,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T2);

    let res: String = conn
        .exec_first("select ip_to_ipv6_mapped(?)", (T2,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T2);

    let res: String = conn
        .exec_first("select ip_to_ipv6_mapped(?)", (T3,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T3);

    let res: Option<String> = conn
        .exec_first("select ip_to_ipv6_mapped(?)", (T4,))
        .unwrap()
        .unwrap();

    assert_eq!(res, None);
}

#[test]
fn test_ip_to_canonical() {
    let conn = &mut get_db_connection(SETUP);

    let res: String = conn
        .exec_first("select ip_to_canonical(?)", (T1,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T1);

    let res: String = conn
        .exec_first("select ip_to_canonical(?)", (T2,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T1);

    let res: String = conn
        .exec_first("select ip_to_canonical(?)", (T3,))
        .unwrap()
        .unwrap();

    assert_eq!(res, T3);

    let res: Option<String> = conn
        .exec_first("select ip_to_canonical(?)", (T4,))
        .unwrap()
        .unwrap();

    assert_eq!(res, None);
}
