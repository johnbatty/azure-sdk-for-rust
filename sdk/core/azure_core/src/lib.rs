// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
// #![warn(missing_docs, future_incompatible, unreachable_pub)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

mod constants;
pub mod hmac;
mod models;
mod options;
mod pipeline;
mod policies;

pub mod credentials;
pub mod headers;
pub mod lro;
pub mod process;
pub mod request_options;

#[cfg(feature = "test")]
pub mod test;

pub mod tokio;

pub use constants::*;
#[doc(inline)]
pub use models::*;
pub use options::*;
pub use pipeline::*;
pub use policies::*;

// Re-export typespec types that are not specific to Azure.
pub use typespec::{Error, Result};
pub mod error {
    pub use typespec::error::*;
    pub use typespec_client_core::error::*;
}
#[cfg(feature = "xml")]
pub use typespec_client_core::xml;
pub use typespec_client_core::{
    base64, date,
    http::{
        headers::Header,
        new_http_client,
        response::{Model, PinnedStream, Response, ResponseBody},
        AppendToUrlQuery, Body, Context, HttpClient, Method, Pager, Request, RequestContent,
        StatusCode, Url,
    },
    json, parsing,
    sleep::{self, sleep},
    stream::{BytesStream, SeekableStream},
    Bytes, Uuid,
};

/// A unique identifier for a request.
// NOTE: Only used for Storage?
pub type RequestId = Uuid;

/// A unique session token.
// NOTE: Only used for Cosmos?
pub type SessionToken = String;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::new();
