use crate::{blob::operations::put_block::PutBlockResponse, prelude::*};
use azure_core::{headers::*, prelude::*};
use bytes::Bytes;

operation! {
    AppendBlock,
    client: BlobClient,
    body: Bytes,
    ?hash: Hash,
    ?condition_max_size: ConditionMaxSize,
    ?condition_append_position: ConditionAppendPosition,
    ?lease_id: LeaseId
}

impl AppendBlockBuilder {
    pub fn into_future(mut self) -> AppendBlock {
        Box::pin(async move {
            let mut url = self.client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "appendblock");

            let mut headers = Headers::new();
            headers.add(self.hash);
            headers.add(self.condition_max_size);
            headers.add(self.condition_append_position);
            headers.add(self.lease_id);

            let mut request = self.client.finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body.clone()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            PutBlockResponse::from_headers(response.headers())
        })
    }
}

type AppendBlockResponse = PutBlockResponse;
