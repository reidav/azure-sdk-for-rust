#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::test]
async fn list_blobs_with_marker() {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = "streamlistblobs235xx752zdve";

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);
    let container = blob_service.container_client(container_name);

    let page = blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();

    if page
        .containers
        .iter()
        .any(|item| item.name == container_name)
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    container
        .create()
        .public_access(PublicAccess::None)
        .await
        .unwrap();

    // create 10 blobs
    for i in 0..10u8 {
        container
            .blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await
            .unwrap();
    }

    let mut marker = None;
    let mut cnt = 0u32;
    loop {
        let builder = container.list_blobs();
        let builder = if let Some(marker) = marker {
            builder.marker(marker)
        } else {
            builder
        };
        let page = builder
            .max_results(std::num::NonZeroU32::new(3u32).unwrap())
            .into_stream()
            .next()
            .await
            .unwrap()
            .unwrap();

        let len = page.blobs.blobs().count();
        println!("received {} blobs", len);
        match cnt {
            0..=2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
        marker = match page.next_marker {
            Some(m) => Some(m.as_str().to_owned()),
            None => break,
        };
    }

    container.delete().await.unwrap();
}
