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
    #[structopt(long, short, help = "dark sky api key")]
    api: String,
    #[structopt(long, short, help = "latitudinal posistion of weather")]
    latitude: String,
    #[structopt(long, short, help = "longitudinal posistion of weather")]
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

fn enc_icon(plain: &str) -> char {
    match plain {
        "clear-day" => '',
        "clear-night" => '',
        "rain" => '',
        "snow" => '流',
        "sleet" => '',
        "wind" => '',
        "fog" => '敖',
        "cloudy" => '',
        "partly-cloudy-day" => '杖',
        "partly-cloudy-night" => '',
        _ => '', // missing
    }
}
