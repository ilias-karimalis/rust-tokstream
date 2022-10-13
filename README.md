# Token Stream Library

This library provides a notion of a token streams for the [nom crate](https://docs.rs/nom/latest/nom/) that
facilitates keeping track of corresponding source spans.

Token streams are sequences of tokens produces by a lexer.


## License

see the LICENSE file.

## Authors

Reto Achermann


## Using TokStream

Simply add the tokstream library as one of the dependencies:

```
tokstream = { git = 'https://github.com/achreto/rust-tokstream' }
```

## Building

Building the library using the `cargo build` command:

```
$ cargo build
```

## Documentation

To build the documentation run the following command:

```
$ cargo doc --no-deps
```


## Contributing

Please follow the [naming and formatting conventions](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) of Rust.

Run `cargo fmt` before committing and ensure the tests are passing `cargo test`.