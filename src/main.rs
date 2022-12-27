#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize };
use serde_json::Value;
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

#[derive(Deserialize, Serialize)]
struct Res {
    results: Vec<Song>
}

#[derive(Deserialize, Serialize)]
struct Song {
    id: String,
    primary_artists: String,
    image: String,
    media_preview_url: String,
    song: String,
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

#[get("/search/<title>")]
fn all(title: &str) -> String {
    let res: Res = ureq::get(&format!("https://www.jiosaavn.com/api.php?_format=json&n=5&p=1&_marker=0&ctx=android&__call=search.getResults&q={}", title))
        .call().unwrap()
        .into_json().unwrap();
     serde_json::to_string(&res).unwrap()
}

#[get("/id/<id>")]
fn get_by_id(id: &str) -> String {
    let res = ureq::get(&format!("https://www.jiosaavn.com/api.php?cc=in&_marker=0%3F_marker%3D0&_format=json&model=Redmi_5A&__call=song.getDetails&pids={}", id))
            .call().unwrap()
            .into_string().unwrap();
    let res:Value = serde_json::from_str(&res).unwrap();
    let res: Value = res.get(id).unwrap().clone();
    let song: Song = serde_json::from_value(res).unwrap();
    println!("{}", &song.media_preview_url);

    format!("{}",&song.media_preview_url.replace("preview", "aac").replace("_96_p.mp4", "_320.mp4"))


}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, coryy, all, get_by_id]).attach(CORS)
}
