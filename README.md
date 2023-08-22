# p256-dynamic-linking-lib

A demo showing how to build a CKB-VM runnable dynamic linking library from a Rust project. We are using p256 as an example here.

# Usage

```
$ # Use prepare.sh to install required Rust components
$ ./prepare.sh
$ # Use build.sh to build p256.so file
$ ./build.sh
$ # Tests are provided to test p256.so file
$ ./test/test.sh
```
