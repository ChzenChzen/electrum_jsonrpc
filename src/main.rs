use hyper::{Server, Client, Response, Body, Request, Method, StatusCode, Uri};
use hyper::service::{make_service_fn, service_fn};
use tokio::io::{self, AsyncWriteExt};
use hyper::header::{CONTENT_TYPE, AUTHORIZATION};
use base64;


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static URL: &str = "http://127.0.0.1:7000";


async fn handle_incoming(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/get_balance") => Ok(Response::new(Body::empty())),
        _ => {
            let not_found = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())
                .unwrap();
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let creds = base64::encode("test:test");
    let basic = format!("Basic {}", creds);
    let client = Client::new();
    let data = r#"{"jsonrpc":"2.0","id":"blabla","method":"help","params":[]}"#;
    let req = Request::builder()
        .method(Method::POST)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, basic)
        .uri("http://127.0.0.1:7000")
        .body(Body::from(data))?;

    println!("{:?}", req);


    let resp = client.request(req).await.unwrap();
    println!("status: {}", resp.status());
    let buf = hyper::body::to_bytes(resp).await?;
    println!("body: {:?}", buf);


    // let new_service = make_service_fn(move |_| {
    //     async { Ok::<_, GenericError>(service_fn(handle_incoming)) }
    // });

    // let addr = ([127, 0, 0, 1], 7000).into();
    // let server = Server::bind(&addr).serve(new_service);
    //
    // println!("Listening on http://{}", addr);
    // server.await?;
    Ok(())
}