use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn serve_file() -> Result<Response<Body>, hyper::Error> {
    let mut file_path = std::path::PathBuf::from("src/website/homepage/index.html");
    if !file_path.is_absolute() {
        if let Ok(current_dir) = std::env::current_dir() {
            file_path = current_dir.join(file_path);
        }
    }

    let mut file = match File::open(file_path).await {
        Ok(file) => file,
        Err(_) => return Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("404 Not Found"))
                            .unwrap()),
    };

    let mut contents = vec![];
    if file.read_to_end(&mut contents).await.is_err() {
        return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("500 Internal Server Error"))
                    .unwrap());
    }

    Ok(Response::new(Body::from(contents)))
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    serve_file().await
}

pub async fn run_server() {
    let make_service = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(handle_request))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_service);

    println!("Web-Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    run_server().await;
}