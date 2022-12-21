use mini_redis::client;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // Spawn two task, one gets a key, the other sets a key
    let t1 = tokio::spawn(async {
        let res = client.get("foo").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
