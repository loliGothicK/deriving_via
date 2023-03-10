# Contribution Guide

Any contribution to `deriving_via` is more than welcome!

## Reporting Issues
-------------------

The best way to contribute to this project is to send a detailed report when you encounter a problem.

It is preferable to minimize the code as much as possible in problematic situations.
Then include in the report the error code and environment (OS, compiler and its version, compile options) and the error message without omission.

## Pull Request
---------------
Pull requests are welcome, even if they are just a typo fix!

However, any significant improvements should be related to the existing feature request or bug report.

## Start
----------

Clone this repository locally.
Install `cargo-make`.

```shell
cargo install cargo-make
```

## Unit tests
-------------------

Run `nextest` via `cargo-make`.

```shell
$ cargo make test
```

## Make Pretty

Once you change the code, to run `cargo make pretty` is recommended. 

## Documentation
-----------------

If you add a new feature, you will probably want to change the document.
Please change both of `README.md` and `deriving_via/README.md`.
Of course, typo and minor wording fixes are also welcome.
