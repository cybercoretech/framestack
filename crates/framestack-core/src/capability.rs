//! The initial capability vocabulary from ADR-0006.
//!
//! The list is intentionally small. An identifier is added only when a
//! reference integration or a concrete project needs it.

/// Every capability identifier this version of FrameStack recognizes.
pub const VOCABULARY: &[&str] = &[
    "frontend.browser",
    "frontend.spa",
    "frontend.ssr",
    "frontend.ssg",
    "frontend.hydration",
    "http.server",
    "http.api",
    "http.middleware",
    "api.rest",
    "api.graphql",
    "rpc.grpc",
    "realtime.websocket",
    "realtime.sse",
    "worker.background",
    "persistence.sql",
    "persistence.nosql",
    "messaging.publish",
    "messaging.subscribe",
];

/// Whether `id` is a recognized capability identifier.
pub fn is_known(id: &str) -> bool {
    VOCABULARY.contains(&id)
}
