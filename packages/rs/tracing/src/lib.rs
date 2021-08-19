//! Polywrap core tracing framework
//! This is WIP

use futures::future::OptionFuture;
use opentelemetry::global::*;
use opentelemetry::sdk::trace::Tracer as OtTracer;
use opentelemetry::sdk::trace::TracerProvider as Tp;
use opentelemetry::sdk::trace::*;
use opentelemetry::trace::Tracer as OpTracer;
use opentelemetry::trace::TracerProvider;
//use opentelemetry_zipkin::Exporter;

pub type MaybeAsync<T> = OptionFuture<T>;

pub fn is_promise<T>(_test: MaybeAsync<T>) -> bool {
    todo!()
}

pub struct Tracer {
    pub trace_enabled: bool,
    tracer: OtTracer,
    provider: Option<Tp>,
    spans: Vec<Span>,
}

impl Tracer {
    pub fn enable_tracing(&mut self, tracer_name: &'static str) {
        self.trace_enabled = true;
        self.init_provider()
            .expect("Error in initializing provider");
        if self.provider.is_some() {
            self.tracer = self
                .provider
                .as_ref()
                .unwrap()
                .tracer(tracer_name, Some("v1"));
        }
    }

    pub fn disable_tracing(&mut self) {
        self.trace_enabled = false;
    }

    pub fn start_span(&mut self, span_name: &'static str) -> Result<(), &str> {
        if !self.trace_enabled {
            return Err("");
        }
        //let current_span = self.current_span();
        // TODO: let span = self.tracer.start_span();
        let span = self
            .tracer
            .start_with_context(span_name, opentelemetry::Context::new());
        self.push_span(span);
        Ok(())
    }

    pub fn end_span(&mut self) -> Result<(), &str> {
        if !self.trace_enabled {
            return Err("");
        }
        let span = self.current_span();
        if span.is_some() {
            //span.end();
            self.pop_span();
        }
        Ok(())
    }

    pub fn set_attribute<T: serde::ser::Serialize>(
        &mut self,
        _attr_name: &str,
        data: T,
    ) -> Result<(), &str> {
        if !self.trace_enabled {
            return Err("");
        }
        let span = self.current_span();
        if span.is_some() {
            let _json_data = serde_json::to_string(&data);
            // TODO: span.unwrap().set_attribute(attr_name, json_data);
        }
        Ok(())
    }

    pub fn add_event<T: serde::ser::Serialize>(
        &mut self,
        _event: &str,
        data: T,
    ) -> Result<(), &str> {
        if !self.trace_enabled {
            return Err("");
        }
        let span = self.current_span();
        if span.is_some() {
            let _json_data = serde_json::to_string(&data);
            // TODO: span.unwrap().set_attribute(event, json_data);
        }
        Ok(())
    }

    pub fn record_exception(&mut self, _error: Error) -> Result<(), &str> {
        if !self.trace_enabled {
            return Err("");
        }
        let span = self.current_span();
        if span.is_some() {
            // record_exception converts the error into a span event
            // TODO: span.unwrap().record_exception(error);
            // If the exception means the operation results in an
            // error state, you can also use it to update the span status.
            // TODO: span.unwrap().set_status({code: SpanStatusCode::ERROR});
        }
        Ok(())
    }

    // TODO: Refactor
    pub fn trace_func<T: Clone + serde::ser::Serialize>(
        &mut self,
        args: T,
        span: &'static str,
        func: fn(args: T) -> Result<T, Error>,
    ) -> Result<(), Error> {
        if let Ok(some_val) = func(args.clone()) {
            self.start_span(span).unwrap();
            self.set_attribute("input", args).unwrap();
            let result = func(some_val);
            // TODO: Use Futures
            match result {
                Ok(output) => {
                    self.set_attribute("output", output).unwrap();
                    self.end_span().unwrap();
                }
                Err(_error) => {}
            }
        } else if let Err(error) = func(args) {
            self.record_exception(error).unwrap();
            self.end_span().unwrap();
        }
        Ok(())
    }

    pub fn init_provider(&mut self) -> Result<(), &str> {
        if self.provider.is_some() {
            return Ok(());
        }

        Ok(())
    }

    pub fn push_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn current_span(&self) -> Option<&Span> {
        Some(&self.spans[0])
    }

    pub fn pop_span(&mut self) {
        let _ = self.spans.pop();
    }
}
