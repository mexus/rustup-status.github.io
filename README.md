# Rustup components availability tool

A library and some binaries to monitor rustup components availability history
on different platforms.

## The library part

Please refer to [docs.rs](https://docs.rs/rustup-available-packages) for more info on
the library, and to the source code of the binary crate for usage hints.

## The Web part

Under the `web` directory you will find a binary crate that's capable of
producing web-pages like
[https://rust-lang.github.io/rustup-components-history/](https://rust-lang.github.io/rustup-components-history/).

Machine-readable information on the latest availability can be fetched on a
*per-component-per-target* basis, i.e.
`https://rust-lang.github.io/rustup-components-history/$target/$package` where `$target` stands for
a target host architecture, like `x86_64-unknown-linux-gnu`, and `$package` stands for a package
name, like `miri` or `rust-src`. For example, getting the date when `miri` was available for the last
time on `x86_64-apple-darwin` is as simple as running the following command:

```
$ curl https://rust-lang.github.io/rustup-components-history/x86_64-apple-darwin/miri
2019-06-08
```

More information (in a JSON format) can be found at a similar location with a `.json` suffix. This
data will include at least the last date the package was available (if it ever was) and whether the
package was available over a configurable range of dates. E.g.,

```
$ curl https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri.json
{"2019-06-13":true,"2019-06-12":true,"2019-06-11":true,"2019-06-10":false,"2019-06-09":true,"2019-06-08":true,"2019-06-07":true,"last_available":"2019-06-13"}
```

Run the binary with a `--help` flag to see available options.

More info is coming :)

### License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

License: MIT/Apache-2.0
