// import the generated boilerplate
varnish::boilerplate!();
// run tests. parameter is always the filename without extension in tests/
varnish::vtc!(test_sanity);
varnish::vtc!(test_traceparent_remote);

use std::error::Error;
use std::str::FromStr;

// even though we won't use it here, we still need to know what the context type is
use varnish::vcl::ctx::Ctx;

use opentelemetry::trace::{SpanContext, SpanId, TraceFlags, TraceId, TraceState};
use opentelemetry_sdk::trace::{IdGenerator, RandomIdGenerator};

// this import is only needed for tests
#[cfg(test)]
use varnish::vcl::ctx::TestCtx;

const SUPPORTED_VERSION: u8 = 0;
const MAX_VERSION: u8 = 254;
const TRACEPARENT_HEADER: &str = "traceparent";
const TRACESTATE_HEADER: &str = "tracestate";

// we now implement all functions from vmod.vcc, but with rust types.
// Don't forget to make the function public with "pub" in front of them

pub fn generate_trace_version(_: &Ctx) -> String {
    format!("00")
}

#[test]
fn generate_trace_version_test() {
    let mut test_ctx = TestCtx::new(100);
    let ctx = test_ctx.ctx();

    assert_eq!("00", generate_trace_version(&ctx));
}

/// Propagates `SpanContext`s in [W3C TraceContext] format.
///
/// [W3C TraceContext]: https://www.w3.org/TR/trace-context/
#[derive(Clone, Debug, Default)]
pub struct VarnishTraceContextPropagator {
    _private: (),
}

impl VarnishTraceContextPropagator {
    /// Create a new `TraceContextPropagator`.
    pub fn new() -> Self {
        VarnishTraceContextPropagator { _private: () }
    }

    /// Extract span context from w3c trace-context header.
    fn extract_span_context(&self, hdr: String) -> Result<SpanContext, ()> {
        let header_value = hdr.trim();

        let parts = header_value.split_terminator('-').collect::<Vec<&str>>();
        // Ensure parts are not out of range.
        if parts.len() < 4 {
            return Err(());
        }

        // Ensure version is within range, for version 0 there must be 4 parts.
        let version = u8::from_str_radix(parts[0], 16).map_err(|_| ())?;
        if version > MAX_VERSION || version == 0 && parts.len() != 4 {
            return Err(());
        }

        // Ensure trace id is lowercase
        if parts[1].chars().any(|c| c.is_ascii_uppercase()) {
            return Err(());
        }

        // Parse trace id section
        let trace_id = TraceId::from_hex(parts[1]).map_err(|_| ())?;

        // Ensure span id is lowercase
        if parts[2].chars().any(|c| c.is_ascii_uppercase()) {
            return Err(());
        }

        // Parse span id section
        let span_id = SpanId::from_hex(parts[2]).map_err(|_| ())?;

        // Parse trace flags section
        let opts = u8::from_str_radix(parts[3], 16).map_err(|_| ())?;

        // Ensure opts are valid for version 0
        if version == 0 && opts > 2 {
            return Err(());
        }

        // Build trace flags clearing all flags other than the trace-context
        // supported sampling bit.
        let trace_flags = TraceFlags::new(opts) & TraceFlags::SAMPLED;

        let trace_state = TraceState::default();

        // create context
        let span_context = SpanContext::new(trace_id, span_id, trace_flags, true, trace_state);

        // Ensure span is valid
        if !span_context.is_valid() {
            return Err(());
        }

        Ok(span_context)
    }
}

pub fn extract_span_id(_: &Ctx, hdr: &str) -> Result<String, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator
        .extract_span_context(String::from(hdr))
        .unwrap_or(SpanContext::empty_context());
    let propagated_span_id = propagated_ctx.span_id();

    match propagated_span_id {
        SpanId::INVALID => {
            let rng: &dyn IdGenerator = &RandomIdGenerator::default();
            let v = rng.new_span_id();
            Ok(format!("{v:x}"))
        }
        _ => Ok(format!("{propagated_span_id:x}")),
    }
}

pub fn extract_trace_id(_: &Ctx, hdr: &str) -> Result<String, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator
        .extract_span_context(String::from(hdr))
        .unwrap_or(SpanContext::empty_context());
    let propagated_trace_id = propagated_ctx.trace_id();

    match propagated_trace_id {
        TraceId::INVALID => {
            let rng: &dyn IdGenerator = &RandomIdGenerator::default();
            let v = rng.new_trace_id();
            Ok(format!("{v:x}"))
        }
        _ => Ok(format!("{propagated_trace_id:x}")),
    }
}

pub fn extract_trace_flags(_: &Ctx, hdr: &str) -> Result<String, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator
        .extract_span_context(String::from(hdr))
        .unwrap_or(SpanContext::empty_context());
    let propagated_trace_flags = propagated_ctx.trace_flags();

    Ok(format!("{propagated_trace_flags:02x}"))
}

pub fn extract_trace_state(_: &Ctx, hdr: &str) -> Result<String, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator
        .extract_span_context(String::from(hdr))
        .unwrap_or(SpanContext::empty_context());
    let propagated_trace_state = propagated_ctx.trace_state();

    Ok(propagated_trace_state.header())
}

// As we only support one version return a static string.
pub fn extract_span_version(_: &Ctx, _hdr: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("{SUPPORTED_VERSION:02x}"))
}

pub fn traceparent_header(_: &Ctx) -> Result<String, Box<dyn Error>> {
    Ok(String::from(TRACEPARENT_HEADER))
}

pub fn tracestate_header(_: &Ctx) -> Result<String, Box<dyn Error>> {
    Ok(String::from(TRACESTATE_HEADER))
}

pub fn trace_is_remote(_: &Ctx, hdr: &str) -> Result<bool, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator
        .extract_span_context(String::from(hdr))
        .unwrap_or(SpanContext::empty_context());

    Ok(propagated_ctx.is_remote())
}

pub fn trace_is_valid(_: &Ctx, hdr: &str) -> Result<bool, Box<dyn Error>> {
    // we need to first "match" to know if a trace was provided, if not,
    // return a default message, otherwise, build a custom one
    let propagator = VarnishTraceContextPropagator::new();
    let propagated_ctx = propagator.extract_span_context(String::from(hdr));

    match propagated_ctx {
        Ok(_) => Ok(true),
        _ => Ok(false),
    }
}

pub fn inject_trace_state(ctx: &mut Ctx) -> Result<(),Box<dyn Error>> {
    if let Some(ref mut http_req) = ctx.http_req {
        match http_req.header(TRACESTATE_HEADER) {
            // we found a tracestate
            Some(hdr) => {
                let trace_state = TraceState::from_str(hdr);
                let _ = http_req.set_header(TRACESTATE_HEADER, &trace_state?.header());
            }
            // we generate a new tracestate
            None => {
                let kvs = vec![("foo", "bar")];
                let trace_state = TraceState::from_key_value(kvs);

                match trace_state {
                    Ok(v) => {
                        let header_value = v.header();
                        let _ = http_req.set_header(TRACESTATE_HEADER, &header_value);
                    }
                    Err(_) => {
                        todo!();
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn inject_trace_parent(ctx: &mut Ctx) -> Result<(),Box<dyn Error>> {
    let rng: &dyn IdGenerator = &RandomIdGenerator::default();

    if let Some(ref mut http_req) = ctx.http_req {
        match http_req.header(TRACEPARENT_HEADER) {
            // we found a traceparent
            Some(hdr) => {
                let propagator = VarnishTraceContextPropagator::new();
                let propagated_ctx = propagator.extract_span_context(String::from(hdr)).unwrap_or_else(|_| {
                    // traceparent was invalid - regenerate it
                    let rng: &dyn IdGenerator = &RandomIdGenerator::default();
                    let trace_id = rng.new_trace_id();
                    let span_id = rng.new_span_id();

                    SpanContext::new(trace_id, span_id, TraceFlags::default(), false, TraceState::default())
                });

                let header_value = format!(
                    "{:02x}-{:032x}-{:016x}-{:02x}",
                    SUPPORTED_VERSION,
                    propagated_ctx.trace_id(),
                    propagated_ctx.span_id(),
                    propagated_ctx.trace_flags() & TraceFlags::SAMPLED,
                );

                let _ = http_req.set_header(TRACEPARENT_HEADER, &header_value);
            }
            _ => {

            }
        }
    }

    Ok(())
}
