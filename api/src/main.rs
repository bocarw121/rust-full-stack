#[macro_use] extern crate rocket;

#[get("/")] 
fn index() -> &'static str {
    "Welcome to the NBA teams api"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
