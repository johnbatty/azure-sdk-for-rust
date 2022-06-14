use azure_core::error::{Error, Result};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for SetQueueServicePropertiesResponse {
    type Error = Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self> {
        debug!("response == {:?}", response);

        Ok(SetQueueServicePropertiesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
