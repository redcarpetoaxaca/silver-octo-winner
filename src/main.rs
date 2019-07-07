#[feature(prot_macro_hygiene, dec1_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello, world"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
