# vtflib-sys

FFI bindings for VTFLib.

`pkg-config` is used to find VTFLib.
Defaults to dynamic linking, static linking can be enabled
either with the `static` feature or with the `VTFLIB_STATIC` environment variable.
If linking statically, keep in mind that VTFLib is LGPL-licensed.

If the library is not found and static linking is enabled, VTFLib is automatically built.
This requires cmake and a C++ compiler.

The library path can be overridden with the environment variable `VTFLIB_PATH` or `VTFLIB13_PATH`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
