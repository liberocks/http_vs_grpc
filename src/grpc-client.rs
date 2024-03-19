use tonic_web::GrpcWebClientLayer;

use book::library_client::LibraryClient;
use book::BookRequest;

pub mod book {
    tonic::include_proto!("book");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::builder().build_http();
    let svc = tower::ServiceBuilder::new()
    .layer(GrpcWebClientLayer::new())
    .service(client);

    let mut client = LibraryClient::with_origin(svc, "http://127.0.0.1:3000".try_into()?);


    let request = tonic::Request::new(BookRequest {
        id: 42,
        title: "Tonic".into(),
        author: "Tonic".into(),
        description: "Tonic".into(),
        published_at: 2021,
    });

    let response = client.ping_pong(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}