# Enzyme-sys

This is a minimal crate that just compiles the [Enzyme
AD](https://enzyme.mit.edu/) and provides an env variable
`DEP_ENZYME_SYS_LIBDIR` to point to the directory that `LLDEnzyme-14.so`
`LLVMEnzyme-14.so` are located.

To use this library, you need to have the LLVM library installed. You can
specify the version of LLVM you have installed by adding a dependency to the
[`llvm-sys`](https://gitlab.com/taricorp/llvm-sys.rs) crate with the version you have installed. For example, if you have
LLVM 14 installed, you can add the following to your `Cargo.toml`:

```toml
[dependencies]
enzyme-sys = "0.1.0"
llvm-sys = version = "140.0.2"
```

If LLVM is installed in a non-standard location, you might also need to set the
`LLVM_SYS_140_PREFIX` environment variable to point to the directory where LLVM
is installed.

Please see the [`llvm-sys`](https://gitlab.com/taricorp/llvm-sys.rs) crate for
more information on how to use it.