# UUID Module

Provide SQL functions to generate various types of UUIDs. This is designed to
mimic Postgres' [uuid-osp] library.

[uuid-osp]: https://www.postgresql.org/docs/current/uuid-ossp.html

## UUID Types

There are four common UUID types:

* v1: MAC address + timestamp + small random portion. The MAC address and
  timestamp can be determined from a v1 UUID
* v3: a MD5 hash of a "namespace" UUID and "name" data. This is fully
  deterministic, there is no random component.
* v4: a fully random UUID
* v5: same as v3, uses SHA1 instead

And three newer UUID types (still nearing formal adoption, but already widely
used) that sort better:

* v6: like v1 but rearranged to sort properly (timestamp first and node address
  last). The node address may be random. (The UUID specifications recommended
  that v7 UUIDs be preferred over v6 if there is no need for compatibility with
  v1 UUIDs)
* v7: a UUID that starts with the current unix timestamp, the rest contains
  random data
* v8: a UUID entirely of desired data, with the exception of a version marking

This library is able to generate v1, v4, v6, and v7 UUIDs. Support for v3 and v5
will be added in the future.

**Note** if for whatever reason the U6-U8 specification changes before it is
finalized (unlikely), these implementations will also change.

## Available Functions

The available functions that return a variable UUID are:

* `uuid_generate_v1()`: Generate a v1 UUID using this node's MAC address
* `uuid_generate_v1mc()`: Generate a v1 UUID using a random multicast MAC address
<!-- * `uuid_generate_v1arg(some_mac)`: Generate a v1 UUID using a specified MAC
  address
* `uuid_generate_v3(namespace, name)`: Generate a v3 UUID from a `namespace`
  UUID and `name` data. For example, `uuid_generate_v3(uuid_ns_url(), 'some
  text')` -->
* `uuid_generate_v4()`: Generate a random v4 UUID
<!-- * `uuid_generate_v5(namespace, name)`: Generate a v5 UUID. This is similar to v3
  but uses SHA1 instead of MD5. -->
* `uuid_generate_v6()` / `uuid_generate_v6(node_address)` Generate a v6 UUID. If
  a node address is specified it will be used, otherwise it will be randomized.
* `uuid_generate_v7()` Generate a v7 UUID (starts with a UNIX timestamp, the
  rest of the data is random).

There are also some functions that return constant values:

* `uuid_nil()`: Return the `nil` UUID (all zeroes)
* `uuid_ns_max()`: Return the `max` UUID (all ones)
* `uuid_ns_dns()`: Return the DNS namespace UUID (used for v3/v5 UUIDs)
* `uuid_ns_url()`: Return the URL namespace UUID (used for v3/v5 UUIDs)
* `uuid_ns_oid()`: Return the ISO OID namespace UUID
* `uuid_ns_x500()`: Return the X.500 namespace UUID

A helper function:

* `uuid_is_valid(uuid)`: Check whether a given UUID is valid

And conversion functions:

* `uuid_to_bin`: Convert a UUID to binary representation. Optionally rearranges
  the UUID so that v1 UUIDs will be indexable by time. (note: v6 and v7 UUIDs
  are already formatted this way, so should be preferred if possible)
* `uuid_from_bin` (alias `bin_to_uuid`): Convert a binary representation of a
  UUID to a string. Optionally dearranges bytes.

## Usage

Load the functions:

```sql
CREATE FUNCTION uuid_generate_v1 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v1mc RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v4 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v6 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v7 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_nil RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_max RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_dns RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_url RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_oid RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_x500 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_is_valid RETURNS integer SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_to_bin RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_from_bin RETURNS string SONAME 'libudf_uuid.so';
-- alias for 'uuid_from_bin'
CREATE FUNCTION  RETURNS string SONAME 'libudf_uuid.so';
```

Usage is as follows:

```sql
SET @uuid = '6ccd780c-baba-1026-9564-5b8c656024db';

-- Create various UUIDs
SELECT uuid_generate_v1();
SELECT uuid_generate_v1mc();
SELECT uuid_generate_v4();
-- Create a v6 UUID with a random node address
SELECT uuid_generate_v6();
-- Create a v6 UUID with a specified node address
SELECT uuid_generate_v6('123abc');
SELECT uuid_generate_v7();

-- UUID constants
SELECT uuid_nil();
SELECT uuid_max();
SELECT uuid_ns_dns();
SELECT uuid_ns_url();
SELECT uuid_ns_oid();
SELECT uuid_ns_x500();

-- Check UUID validity
SELECT uuid_is_valid(uuid_generate_v4());
SELECT uuid_is_valid(@uuid);
SELECT uuid_is_valid('definitely not valid');

-- Do some conversions
SELECT uuid_to_bin(uuid_generate_v4());
SELECT uuid_from_bin(uuid_to_bin(@uuid));
-- "true" specifies that bytes should be arranged for time sortability
SELECT uuid_from_bin(uuid_to_bin(@uuid, true), true);
```
