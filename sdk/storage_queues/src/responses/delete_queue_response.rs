use azure_core::error::{Error, Result};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for DeleteQueueResponse {
    type Error = Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self> {
        debug!("response == {:?}", response);

        Ok(DeleteQueueResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
