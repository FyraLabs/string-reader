# `string-reader`

`string-reader` is a crate that provides traits and structs for making readers take in and output `&str` and `String` instead of `[u8]`.

## Traits

### `StrRead`

The base trait that both `&str`-like and `String`-like readers implement.

### `RealStrRead`

The trait that `&str`-like readable structs implement.

### `StringRead`

The trait that `String`-like readable structs implement.

### `StrWrite`

`&str`-like writable structs implement this.

### `StringWrite`

`String`-like writable structs implement this.

## Structs

### `StrReader`

A read and write reader that takes in and outputs `&str`s.

### `StringReader`

A read and write reader that takes in and outputs `String`s.
