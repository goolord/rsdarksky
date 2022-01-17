extern crate reqwest;
extern crate serde;
extern crate serde_json;
use serde_json::{Map, Value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rsdarksky",
    about = "simple dark sky weather (for use in scripted bars)",
    version = "1.0",
    author = "Zachary Churchill <zacharyachurchill@gmail.com>"
)]
struct Opt {
    #[structopt(long, help = "dark sky api key")]
    api: String,
    #[structopt(long, help = "latitudinal posistion of weather")]
    latitude: String,
    #[structopt(long, help = "longitudinal posistion of weather")]
    longitude: String,
}

fn main() {
    let opt = Opt::from_args();

    let uri: std::string::String = format!(
        "https://api.darksky.net/forecast/{}/{},{}?units=auto",
        opt.api, opt.latitude, opt.longitude
    );

    // erros in unwrap here will mean an api change
    let resp: Map<String, Value> = reqwest::get(&uri)
        .expect("Failed to send request")
        .json()
        .expect("Failed to deserialize json");
    let currently = Map::get(&resp, "currently").expect("key \"currently\" does not exist");
    let icon = enc_icon(
        Value::as_str(
            Map::get(Value::as_object(&currently).unwrap(), "icon")
                .expect("key \"icon\" does not exist"),
        )
        .unwrap(),
    );
    let deg = Value::as_f64(
        Map::get(Value::as_object(&currently).unwrap(), "temperature")
            .expect("key \"temperature\" does not exist"),
    )
    .unwrap();

    println!("{}  {:.0}°", icon, deg)
}

fn enc_icon(plain: &str) -> String {
    let (ico, color) = match plain {
        "clear-day"           => ('' , "#fabd2f"),
        "clear-night"         => ('' , "#d5c4a1"),
        "rain"                => ('' , "#83a598"),
        "snow"                => ('流', "#fbf1c7"),
        "sleet"               => ('' , "#d3869b"),
        "wind"                => ('' , "#ebdbb2"),
        "fog"                 => ('敖', "#928374"),
        "cloudy"              => ('' , "#d5c4a1"),
        "partly-cloudy-day"   => ('杖', "#d79921"),
        "partly-cloudy-night" => ('' , "#bdae93"),
        _                     => ('' , "#fb4934"), // missing
    };
    format!("<span color =\"{}\">{}</span>", color, ico)
}
