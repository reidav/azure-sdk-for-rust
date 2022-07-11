use crate::prelude::*;
use azure_core::{
    headers::{BLOB_TYPE, PAGE_WRITE, *},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, Utc};

operation! {
    ClearPage,
    client: BlobClient,
    ba512_range: BA512Range,
    ?sequence_number_condition: SequenceNumberCondition,
    ?if_modified_since_condition: IfModifiedSinceCondition,
    ?if_match_condition: IfMatchCondition,
    ?lease_id: LeaseId
}

impl ClearPageBuilder {
    pub fn into_future(mut self) -> ClearPage {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "page");

            let mut headers = Headers::new();
            headers.insert(PAGE_WRITE, "clear");
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.add(self.ba512_range);
            headers.add(self.sequence_number_condition);
            headers.add(self.if_modified_since_condition);
            headers.add(self.if_match_condition);
            headers.add(self.lease_id);

            let mut request =
                self.client
                    .finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            ClearPageResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(ClearPageResponse,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    sequence_number_from_headers => sequence_number: u64,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: DateTime<Utc>
);
