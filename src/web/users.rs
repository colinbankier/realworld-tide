
#[cfg(test)]
mod tests {
use realworld_tide::test_helpers::generate;
use realworld_tide::Repo;
use tokio_async_await_test::async_test;
mod test_server;

#[async_test]
async fn register_and_login_integration() {
    let server = test_server::new(Repo::new());

    let user = generate::new_user();
    let req = http::Request::post("/api/register")
        .body(Body::empty())
        .unwrap();
    let res = server.simulate(req).unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(foo, 3);
}
}