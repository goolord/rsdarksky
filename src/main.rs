#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
use clap::{Arg, App, SubCommand};
use serde_json::{Value, Error, Map};
use std::env;
use std::io::{self, Read, Write};

fn main() {
    let matches = clap_app!(rsdarksky =>
        (version: "1.0")
        (author: "Zachary Churchill <zacharyachurchill@gmail.com>")
        (about: "simple dark sky weather (for use in scripted bars)")
        (@arg API: -a --api +required +takes_value "dark sky api key")
        (@arg LATITUDE: -l --latitude +required +takes_value +allow_hyphen_values "current latitude")
        (@arg LONGITUDE: -o --longitude +required +takes_value +allow_hyphen_values "current longitude")
    ).get_matches();

    let api  = matches.value_of("API").unwrap();
    let lat  = matches.value_of("LATITUDE").unwrap();
    let long = matches.value_of("LONGITUDE").unwrap();

    let uri: std::string::String = format!("https://api.darksky.net/forecast/{}/{},{}?units=auto", api, lat, long);

    let resp: Map<String, Value> = reqwest::get(&uri).expect("Failed to send request").json().expect("Failed to deserialize json");
    let currently = Map::get(&resp,"currently").expect("key \"currently\" does not exist");
    let icon = enc_icon(Value::as_str(Map::get(Value::as_object(&currently).unwrap(), "icon").expect("key \"icon\" does not exist")).unwrap());
    let deg = Value::as_f64(
                Map::get(Value::as_object(&currently).unwrap(), "temperature").expect("key \"temperature\" does not exist")
            ).unwrap();

    println!("{} {:.0}°",icon,deg)
}

fn enc_icon(plain: &str) -> &str {
    match plain {
    "clear-day"           => "",
    "clear-night"         => "",
    "rain"                => "",
    "snow"                => "流",
    "sleet"               => "",
    "wind"                => "",
    "fog"                 => "敖",
    "cloudy"              => "",
    "partly-cloudy-day"   => "杖",
    "partly-cloudy-night" => "",
    _                     => ""
    }
}

