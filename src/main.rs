#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes};
use sudluko::{generate_as_string, solve_as_string};

#[get("/")]
fn home() -> rocket::response::Redirect {
    rocket::response::Redirect::to("/generate")
}

#[get("/status")]
fn status() -> &'static str {
    "It's Alive!"
}

#[get("/generate")]
fn generate() -> String {
    generate_as_string()
}

#[get("/solve")]
fn solve() -> String {
    "Insert the Sudoku in the URL, with . for spaces.".to_string()
}

#[get("/solve/<sudoku>")]
fn solver(sudoku: String) -> String {
    solve_as_string(sudoku)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![home, status, generate, solve, solver])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn alive_responds() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/status").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("It's Alive!".into()));
    }
}
