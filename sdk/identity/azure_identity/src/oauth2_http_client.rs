// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Implements the oauth2 crate http client interface using an [`azure_core::HttpClient`] instance.
//! <https://docs.rs/oauth2/latest/oauth2/#importing-oauth2-selecting-an-http-client-interface>

use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    Body, Bytes, HttpClient, Request, Url,
};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tracing::warn;

pub(crate) struct Oauth2HttpClient {
    http_client: Arc<dyn HttpClient>,
}

impl Oauth2HttpClient {
    /// Create a new `Oauth2HttpClient`.
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self { http_client }
    }

    pub(crate) async fn request(
        &self,
        oauth2_request: oauth2::HttpRequest,
    ) -> Result<oauth2::HttpResponse, azure_core::error::Error> {
        let method = try_from_method(oauth2_request.method())?;
        let url: Url = oauth2_request.uri().to_string().parse().map_err(|e| {
            Error::full(
                ErrorKind::Other,
                e,
                "Failed to parse the http::Uri as a url::Url",
            )
        })?;
        let mut request = Request::new(url, method);
        for (name, value) in to_headers(oauth2_request.headers()) {
            request.insert_header(name, value);
        }
        request.set_body(Body::Bytes(Bytes::copy_from_slice(
            oauth2_request.body().as_slice(),
        )));
        let response = self.http_client.execute_request(&request).await?;
        let status_code = try_from_status(response.status())?;
        let headers = try_from_headers(response.headers())?;
        let body = response.into_raw_body().collect().await?.to_vec();

        let mut oauth_response = oauth2::HttpResponse::new(body);
        *oauth_response.headers_mut() = headers;
        *oauth_response.status_mut() = status_code;

        Ok(oauth_response)
    }
}

fn try_from_method(method: &oauth2::http::Method) -> azure_core::Result<azure_core::Method> {
    match *method {
        oauth2::http::Method::GET => Ok(azure_core::Method::Get),
        oauth2::http::Method::POST => Ok(azure_core::Method::Post),
        oauth2::http::Method::PUT => Ok(azure_core::Method::Put),
        oauth2::http::Method::DELETE => Ok(azure_core::Method::Delete),
        oauth2::http::Method::HEAD => Ok(azure_core::Method::Head),
        oauth2::http::Method::PATCH => Ok(azure_core::Method::Patch),
        _ => Err(Error::with_message(ErrorKind::DataConversion, || {
            format!("unsupported oauth2::http::Method {method}")
        })),
    }
}

fn try_from_headers(
    headers: &azure_core::headers::Headers,
) -> azure_core::Result<oauth2::http::HeaderMap> {
    let mut header_map = oauth2::http::HeaderMap::new();
    for (name, value) in headers.iter() {
        let name = name.as_str();
        let header_name = oauth2::http::header::HeaderName::from_str(name)
            .with_context(ErrorKind::DataConversion, || {
                format!("unable to convert http header name '{name}'")
            })?;
        let value = value.as_str().to_owned();
        header_map.append(
            header_name,
            oauth2::http::HeaderValue::from_str(&value)
                .with_context(ErrorKind::DataConversion, || {
                    format!("unable to convert http header value for '{name}'")
                })?,
        );
    }
    Ok(header_map)
}

fn try_from_status(status: azure_core::StatusCode) -> azure_core::Result<oauth2::http::StatusCode> {
    oauth2::http::StatusCode::from_u16(*status).map_kind(ErrorKind::DataConversion)
}

fn to_headers(map: &oauth2::http::header::HeaderMap) -> azure_core::headers::Headers {
    let map = map
        .iter()
        .filter_map(|(k, v)| {
            let key = k.as_str();
            if let Ok(value) = v.to_str() {
                Some((
                    azure_core::headers::HeaderName::from(key.to_owned()),
                    azure_core::headers::HeaderValue::from(value.to_owned()),
                ))
            } else {
                warn!("header value for `{key}` is not utf8");
                None
            }
        })
        .collect::<HashMap<_, _>>();
    azure_core::headers::Headers::from(map)
}
