// import the generated boilerplate
varnish::boilerplate!();
// run tests. parameter is always the filename without extension in tests/
varnish::vtc!(test_sanity);

use std::error::Error;
use varnish::vcl::ctx::Ctx;
// this import is only needed for tests
#[cfg(test)]
use varnish::vcl::ctx::TestCtx;

pub fn greet(_: &Ctx, value: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("Hello, {value}!"))
}

#[test]
fn greet_test() {
    let mut test_ctx = TestCtx::new(100);
    let ctx = test_ctx.ctx();

    assert_eq!("Hello, World!", greet(&ctx, "World").unwrap_or("".to_string()));
}
