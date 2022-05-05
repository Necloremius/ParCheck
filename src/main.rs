#[macro_use] extern crate rocket; // imports the rocket crate
use rocket::{Rocket, Build};

#[get("/check/<num>")]
fn pch(num: i128) -> &'static str { if num % 2 == 0 { "even" } else { "odd" }} // checks if the number is even or odd, then returns this result


#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build().mount("/check/<num>", routes![pch])
}
