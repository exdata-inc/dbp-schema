# dbp-schema

## for Rust
```
cargo build
```

## for Go-lang
```
protoc -I . --go_out=paths=source_relative:. --experimental_allow_proto3_optional  dbp_schema.proto
```
