# prost-uuid-doubleint

Custom uuid type for using in protobuf .proto files.

Represents uuid as 2 u64 integers.


[![Rust](https://github.com/evilbluebeaver/prost-uuid-doubleint/actions/workflows/rust.yml/badge.svg)](https://github.com/evilbluebeaver/prost-uuid-doubleint/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/prost-uuid-doubleint)](https://crates.io/crates/prost-uuid-doubleint)


## Usage

### Proto files
You need to have a file with Uuid

```protobuf
syntax = "proto3";
package uuid;

message Uuid {
   uint64 high = 1;
   uint64 low = 2;
}
```
and import it from another .proto file

```protobuf
syntax = "proto3";
package user;

import "uuid.proto";

message User {
    uuid.Uuid id = 1;
    string name = 2;
}
```

### Cargo.toml

``` toml
...
[dependencies]
prost = "0.13"
prost-uuid-doubleint = "0.1.0"
...
```

### Build.rs
``` rust
...
    prost_build::Config::new()
        .extern_path(".uuid.Uuid", "::prost_uuid_doubleint::ProstUuid")
        .compile_protos(&["./proto/user.proto"], &["./proto"])
...
```


## Tnanks to
https://gitlab.com/oss47/prost-uuid