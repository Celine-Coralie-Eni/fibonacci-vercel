use vercel_runtime::{run, Body, Error, Request, Response};

fn fibonacci_calc(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_calc(n - 1) + fibonacci_calc(n - 2)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Get query param ?n=5
    let n = req
        .uri()
        .query()
        .and_then(|q| {
            q.split('&')
                .find(|kv| kv.starts_with("n="))
                .and_then(|kv| kv.split('=').nth(1))
        })
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10); // default to 10

    let result = fibonacci_calc(n);
    let body = format!("Fibonacci of {} is {}", n, result);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(Body::Text(body))?)
}
