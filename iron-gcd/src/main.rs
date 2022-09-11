extern crate iron;
extern crate router;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");

    println!("Serving on http://localhost:3000....");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut("text/html; charset=Utf8".parse::<Mime>().unwrap());
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n">
            <input type="text" name="m">
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    Ok(response)
}
