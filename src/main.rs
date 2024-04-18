use nostr_indexeddb::database::{NostrDatabase, Order};
use nostr_indexeddb::nostr::prelude::*;
use nostr_indexeddb::WebDatabase;

#[tokio::main]
async fn main() {
    let secret_key =
        SecretKey::from_bech32("nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99")
            .unwrap();
    let keys_a = Keys::new(secret_key);

    let database = WebDatabase::open("nostr-sdk-indexeddb-test").await.unwrap();

    let metadata = Metadata::new().name("Name");
    let event = EventBuilder::metadata(&metadata).to_event(&keys_a).unwrap();
    database.save_event(&event).await.unwrap();

    let events = database
        .query(
            vec![Filter::new()
                .kinds(vec![Kind::Metadata, Kind::Custom(123), Kind::TextNote])
                .limit(20)
                .author(keys_a.public_key())],
            Order::Desc,
        )
        .await
        .unwrap();
}
