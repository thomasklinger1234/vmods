# vmod-tracecontext

**WORK IN PROGRESS**

This VMOD provides integration of [OpenTelemetry](https://opentelemetry.io/) and [W3C TraceContext](https://www.w3.org/TR/trace-context/) into Varnish.

# Compiling

You need only two things:
- a stable version of `cargo`/`rust`
- the `libvarnish` development files installed where `pkg-config` can find them
- `python31

From within this directory, run:

```
# build
cargo build
# you should now have a file name target/debug/libvmod_{name}.so

# test (you need to build first!)
cargo test
```

That's it!

# Files

Look around, everything should be decently documented:
- [vmod.vcc](vmod.vcc): your starting point, where you will describe your vmod API
- [src/lib.rs](src/lib.rs): the file contianing the actual implementation and unit tests
- [tests/test_sanity.vtc](tests/test_sanity.vtc): a VTC (full stack) test, actually running Varnish against mock clients and servers
- [Cargo.toml](Cargo.toml): the file describing the name of the vmod, as well as its dependencies
- [build.rs](build.rs): a short program in charge of generating some boilerplate before the compiler starts
