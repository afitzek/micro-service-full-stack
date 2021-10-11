use rocket::{launch, get, routes, fs::FileServer};

use rocket::{Rocket, Request, Data, Response, Build, Orbit};
use rocket::fairing::{self, Fairing, Info, Kind};

struct MyType{}

#[rocket::async_trait]
impl Fairing for MyType {

    fn info(&self) -> Info {
        Info { name: "CORS", kind: Kind::Response }
    }
    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyType{})
        .mount("/api", routes![hello])
        .mount("/public", FileServer::from("./ui/weather-service-ui/dist"))
}