# Dockerfile to build the udf-examples crate and load it. Usage:
#
# ```
# # build image
# docker build . --tag mdb-udf-suite
# # Run image
# docker run --rm -e MARIADB_ROOT_PASSWORD=example --name mdb-udf-suite-c mdb-udf-suite
# # Open a shell
# docker exec -it mdb-udf-suite-c mariadb -pexample
# ```

FROM rust:bullseye AS build

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && mkdir /output \
    && cp target/release/*.so /output && ls /output -la


FROM vuongtlt13/mysql:8.2-debian as production

COPY --from=build /output/libudf_hash.so /usr/lib/mysql/plugin/
