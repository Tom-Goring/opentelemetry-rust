//! Trace exporters
use crate::Resource;
use async_trait::async_trait;
use opentelemetry_api::trace::{Event, Link, SpanContext, SpanId, SpanKind, Status, TraceError};
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::SystemTime;

pub mod stdout;

/// Describes the result of an export.
pub type ExportResult = Result<(), TraceError>;

/// `SpanExporter` defines the interface that protocol-specific exporters must
/// implement so that they can be plugged into OpenTelemetry SDK and support
/// sending of telemetry data.
///
/// The goal of the interface is to minimize burden of implementation for
/// protocol-dependent telemetry exporters. The protocol exporter is expected to
/// be primarily a simple telemetry data encoder and transmitter.
#[async_trait]
pub trait SpanExporter: Send + Debug {
    /// Exports a batch of readable spans. Protocol exporters that will
    /// implement this function are typically expected to serialize and transmit
    /// the data to the destination.
    ///
    /// This function will never be called concurrently for the same exporter
    /// instance. It can be called again only after the current call returns.
    ///
    /// This function must not block indefinitely, there must be a reasonable
    /// upper limit after which the call must time out with an error result.
    ///
    /// Any retry logic that is required by the exporter is the responsibility
    /// of the exporter.
    async fn export(&mut self, batch: Vec<SpanData>) -> ExportResult;

    /// Shuts down the exporter. Called when SDK is shut down. This is an
    /// opportunity for exporter to do any cleanup required.
    ///
    /// This function should be called only once for each `SpanExporter`
    /// instance. After the call to `shutdown`, subsequent calls to `export` are
    /// not allowed and should return an error.
    ///
    /// This function should not block indefinitely (e.g. if it attempts to
    /// flush the data and the destination is unavailable). SDK authors
    /// can decide if they want to make the shutdown timeout
    /// configurable.
    fn shutdown(&mut self) {}
}

/// `SpanData` contains all the information collected by a `Span` and can be used
/// by exporters as a standard input.
#[derive(Clone, Debug, PartialEq)]
pub struct SpanData {
    /// Exportable `SpanContext`
    pub span_context: SpanContext,
    /// Span parent id
    pub parent_span_id: SpanId,
    /// Span kind
    pub span_kind: SpanKind,
    /// Span name
    pub name: Cow<'static, str>,
    /// Span start time
    pub start_time: SystemTime,
    /// Span end time
    pub end_time: SystemTime,
    /// Span attributes
    pub attributes: crate::trace::EvictedHashMap,
    /// Span events
    pub events: crate::trace::EvictedQueue<Event>,
    /// Span Links
    pub links: crate::trace::EvictedQueue<Link>,
    /// Span status
    pub status: Status,
    /// Resource contains attributes representing an entity that produced this span.
    pub resource: Cow<'static, Resource>,
    /// Instrumentation library that produced this span
    pub instrumentation_lib: crate::InstrumentationLibrary,
}