use crate::{blob::PageRangeList, prelude::*};
use azure_core::{headers::*, prelude::*, RequestId};
use chrono::{DateTime, Utc};
use std::str::from_utf8;

pub struct GetPageRangesBuilder {
    blob_client: BlobClient,
    blob_versioning: Option<BlobVersioning>,
    lease_id: Option<LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> GetPageRangesBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            lease_id: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        blob_versioning: BlobVersioning => Some(blob_versioning),
        lease_id: LeaseId => Some(lease_id),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.blob_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "pagelist");
            self.blob_versioning.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.blob_client
                    .prepare_request(url.as_str(), http::Method::GET, None)?;
            request.add_optional_header(&self.lease_id);
            request.add_optional_header(&self.client_request_id);

            let response = self
                .blob_client
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            GetPageRangesResponse::from_response(response.headers(), response.body())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetPageRangesResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub page_list: PageRangeList,
}

impl GetPageRangesResponse {
    pub(crate) fn from_response(
        headers: &Headers,
        body: &[u8],
    ) -> azure_core::Result<GetPageRangesResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let body = from_utf8(body)?;
        let page_list = PageRangeList::try_from_xml(&body[3..] as &str)?;

        Ok(GetPageRangesResponse {
            etag,
            last_modified,
            request_id,
            date,
            page_list,
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetPageRangesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetPageRangesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
