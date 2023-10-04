# udf-suite

A collection of UDFs for MariaDB & MySQL, written using the rust [`udf`]
library. For instructions on how to use these libraries, jump to the
[Installation](#installation) section.

New function contributions are welcome!

[`udf`]: http://docs.rs/udf

## Included UDFs

The following UDFs are includes:

- [UUIDs](#uuid): generate and convert v1, v2, v6, and v7 UUIDs
- [xxhash](#xxhash): run `xxhash3`, `xxhash32`, and `xxhash64` hash algorithms
- [blake](#blake): run `blake2s256`, `blake2b512`, and `blake3` hash algorithms
- [Jsonify](#jsonify): convert any data to JSON
- [Lipsum](#lipsum): generate random text

### UUID

Provide UUID functions similar to the Postges [`uuid-osp`] package:

- Generate v1 and v4 UUIDs (v3 & v5 coming soon)
- Generate the new v6 and v7 UUIDs
- Validate UUIDs
- Create namespace UUIDs
- `uuid_to_bin` and `uuid_from_bin`/`bin_to_uuid` functions, including bit
  rearranging options

See the [UUID Readme](/udf-uuid/README.md) for more information

```text
MariaDB [(none)]> select uuid_generate_v6();
+--------------------------------------+
| uuid_generate_v6()                   |
+--------------------------------------+
| 1ede5b09-ea01-6208-bca8-8809c0dd8e70 |
+--------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select hex(uuid_to_bin(uuid_generate_v4()));
+--------------------------------------+
| hex(uuid_to_bin(uuid_generate_v4())) |
+--------------------------------------+
| B1B3AB9D490A4D20BFBD026AB1C045FB     |
+--------------------------------------+
1 row in set (0.002 sec)
```

[`uuid-osp`]: https://www.postgresql.org/docs/current/uuid-ossp.html

### Jsonify

Provide the function `jsonify`, which quickly creates JSON output for any given
inputs.

```text
MariaDB [db]> select jsonify(qty, cost, class) from t1 limit 4;
+-------------------------------------+
| jsonify(qty, cost, class)           |
+-------------------------------------+
| {"class":"a","cost":50.0,"qty":10}  |
| {"class":"c","cost":5.6,"qty":8}    |
| {"class":"a","cost":20.7,"qty":5}   |
| {"class":"b","cost":12.78,"qty":10} |
+-------------------------------------+
4 rows in set (0.000 sec)
```

Aliasing also works to change key names:

```text
MariaDB [db]> select jsonify(uuid() as uuid, qty as quantity, cost) from t1 limit 4;
+----------------------------------------------------------------------------+
| jsonify(uuid() as uuid, qty as quantity, cost)                             |
+----------------------------------------------------------------------------+
| {"cost":50.0,"quantity":10,"uuid":"45952863-5b4d-11ed-b214-0242ac110002"}  |
| {"cost":5.6,"quantity":8,"uuid":"4595291b-5b4d-11ed-b214-0242ac110002"}    |
| {"cost":20.7,"quantity":5,"uuid":"45952953-5b4d-11ed-b214-0242ac110002"}   |
| {"cost":12.78,"quantity":10,"uuid":"4595297a-5b4d-11ed-b214-0242ac110002"} |
+----------------------------------------------------------------------------+
4 rows in set (0.001 sec)
```


### Lipsum

Uses the [lipsum crate] to generate lipsum strings with a specified word count.


```text
MariaDB [(none)]> select lipsum(10);
+------------------------------------------------------------------+
| lipsum(10)                                                       |
+------------------------------------------------------------------+
| Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do. |
+------------------------------------------------------------------+
1 row in set (0.000 sec)
```

[lipsum crate]: https://docs.rs/lipsum/latest/lipsum/

## Blake

Blake hash functions are cyptographic hash algorithms. This library includes
`blake2s256`, `blake2b256`, and `blake3`.

Since the results are binary, you will often want to call `hex()` on the
results (unless storing directly in a `BINARY(32)`/`BINARY(64)`).

```text
MariaDB [(none)]> select hex(blake2b512("Hello, world!"));
+----------------------------------------------------------------------------------------------------------------------------------+
| hex(blake2b512("Hello, world!"))                                                                                                 |
+----------------------------------------------------------------------------------------------------------------------------------+
| A2764D133A16816B5847A737A786F2ECE4C148095C5FAA73E24B4CC5D666C3E45EC271504E14DC6127DDFCE4E144FB23B91A6F7B04B53D695502290722953B0F |
+----------------------------------------------------------------------------------------------------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select hex(blake2s256("Hello, world!"));
+------------------------------------------------------------------+
| hex(blake2s256("Hello, world!"))                                 |
+------------------------------------------------------------------+
| 30D8777F0E178582EC8CD2FCDC18AF57C828EE2F89E978DF52C8E7AF078BD5CF |
+------------------------------------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select hex(blake3("Hello, world!"));
+------------------------------------------------------------------+
| hex(blake3("Hello, world!"))                                     |
+------------------------------------------------------------------+
| EDE5C0B10F2EC4979C69B52F61E42FF5B413519CE09BE0F14D098DCFE5F6F98D |
+------------------------------------------------------------------+
1 row in set (0.000 sec)
```

## xxhash

The xxhash functions are fast non-cryptographic hash algorithms. This libary
includes `xxhash3`, `xxhash32`, `xxhash64`, and `xxhash` (an alias for
`xxhash64`).

```text
MariaDB [(none)]> select xxhash('Hello, world!');
+-------------------------+
| xxhash('Hello, world!') |
+-------------------------+
|     -755700219241327498 |
+-------------------------+
1 row in set (0.000 sec)
```

Multiple arguments are combined to produce a single hash output

```text
MariaDB [(none)]> select xxhash('Hello, ', 0x77, 'orld', '!');
+--------------------------------------+
| xxhash('Hello, ', 0x77, 'orld', '!') |
+--------------------------------------+
|                  -755700219241327498 |
+--------------------------------------+
1 row in set (0.000 sec)
```

Note that in SQL, all integers are an `i64`, all floats are a `f64`, and all
decimals are represented as a string to the UDF API. This library hashes these
types as their little endian representation. (You only need to worry about this
if you have very obscure platform compatibility requirements, and strings and
blobs are always unambiguous).

## Installation

Compiled library binaries can be downloaded from this library's [releases] page.
The desired files can be copied to the plugin directory (usually
`/usr/lib/mysql/plugin`) and selectively loaded:

```sql
CREATE FUNCTION blake2b512 RETURNS string SONAME 'libudf_blake.so';
CREATE FUNCTION blake2s256 RETURNS string SONAME 'libudf_blake.so';
CREATE FUNCTION blake3 RETURNS string SONAME 'libudf_blake.so';

CREATE FUNCTION jsonify RETURNS string SONAME 'libudf_jsonify.so';
CREATE FUNCTION lipsum RETURNS string SONAME 'libudf_lipsum.so';

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
-- `bin_to_uuid` and 'uuid_from_bin' are aliases
CREATE FUNCTION bin_to_uuid RETURNS string SONAME 'libudf_uuid.so';

-- `xxhash` and `xxhash64` are aliases
CREATE FUNCTION xxhash RETURNS integer SONAME 'libudf_xxhash.so';
CREATE FUNCTION xxhash3 RETURNS integer SONAME 'libudf_xxhash.so';
CREATE FUNCTION xxhash32 RETURNS integer SONAME 'libudf_xxhash.so';
CREATE FUNCTION xxhash64 RETURNS integer SONAME 'libudf_xxhash.so';
```

Note that Windows `.dll`s are built but have not been tested - please open an
issue if you encounter any errors.

[releases]: https://github.com/pluots/udf-suite/releases


### Building from Source

To build the binaries yourself, you can clone this repository and run:

```sh
cargo build --release
```

Which will produce the desired dynamic library files in `target/release`.
Specific functions can also be specified with `-p` (e.g.
`cargo build --release -p udf-uuid`).

This repository also comes with a docker file that simplifies getting an image
up and running:

```sh
# build the image
docker build . --tag mdb-udf-suite-img

# run it in the background
docker run --rm -d \
  -e MARIADB_ROOT_PASSWORD=example \
  --name mdb_udf_suite \
  mdb-udf-suite-img

# Enter a SQL shell
docker exec -it mdb_udf_suite mysql -pexample

# Stop the server when done
docker stop mdb_udf_suite
```

The UDFs can then be loaded using the `CREATE FUNCTION` statements above.

This project has a MSRV of 1.65, but makes no commitment to uphold this.


## License

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose either of them if you use this work.

`SPDX-License-Identifier: Apache-2.0 OR GPL-2.0-or-later`
