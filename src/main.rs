#[macro_use] extern crate rocket;
use serde::Deserialize;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Deserialize)]
struct Res {
    results: Vec<Song>
}

#[derive(Deserialize)]
struct Song {
    media_preview_url: String,
}

#[options("/one/<title>")]
fn coryy(title: &str) -> String {
    format!("fuck cors by {}", title)
}

#[get("/one/<title>")]
fn hello(title: &str) -> String {
    let res: Res = ureq::get(&format!("https://www.jiosaavn.com/api.php?_format=json&n=5&p=1&_marker=0&ctx=android&__call=search.getResults&q={}", title))
        .call().unwrap()
        .into_json().unwrap();
    format!("{}", res.results[0].media_preview_url.clone().replace("preview", "aac").replace("_96_p.mp4", "_320.mp4"))

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, coryy]).attach(CORS)
}
