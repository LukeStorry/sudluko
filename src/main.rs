#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes};

/// Declare a handler.
#[get("/alive")]
fn alive() -> &'static str {
    "It's Alive!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![alive])
}

/// Start our server.
fn main() {
    rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn alive_responds() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/alive").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("It's Alive!".into()));
    }
}