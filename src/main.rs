use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Url;
use serde::Deserialize;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Parser)]
#[command(
    name = "rsdarksky",
    about = "Simple Pirate Weather client for scripted status bars",
    version,
    author = "Zachary Churchill <zacharyachurchill@gmail.com>"
)]
struct Opt {
    /// Pirate Weather API key (or set `PIRATE_WEATHER_API_KEY`)
    #[arg(long, env = "PIRATE_WEATHER_API_KEY")]
    api: String,

    /// Latitude of the location
    #[arg(long, value_parser = clap::value_parser!(f64))]
    latitude: f64,

    /// Longitude of the location
    #[arg(long, value_parser = clap::value_parser!(f64))]
    longitude: f64,
}

#[derive(Deserialize)]
struct Forecast {
    currently: Currently,
}

#[derive(Deserialize)]
struct Currently {
    icon: String,
    temperature: f64,
}

fn build_forecast_url(api: &str, latitude: f64, longitude: f64) -> Result<Url> {
    let mut url =
        Url::parse("https://api.pirateweather.net/forecast").context("invalid base URL")?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| anyhow::anyhow!("base URL cannot be a base"))?;
        segments.push(api);
        segments.push(&format!("{latitude},{longitude}"));
    }
    url.query_pairs_mut().append_pair("units", "auto");
    Ok(url)
}

fn http_client() -> Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("failed to build HTTP client")
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let uri = build_forecast_url(&opt.api, opt.latitude, opt.longitude)?;

    let forecast: Forecast = http_client()?
        .get(uri)
        .send()
        .context("failed to send request")?
        .error_for_status()
        .context("request returned an error status")?
        .json()
        .context("failed to deserialize response")?;

    let icon = encode_icon(&forecast.currently.icon);
    println!("{}\u{2005} {:.0}°", icon, forecast.currently.temperature);

    Ok(())
}

fn encode_icon(plain: &str) -> String {
    // Nerd Fonts v3 weather icons (U+E300–E3E3):
    // https://github.com/ryanoasis/nerd-fonts/wiki/Glyph-Sets-and-Code-Points
    //
    // plain                 | glyph
    // clear-day             | weather-day_sunny              (e30d)
    // clear-night           | weather-night_clear            (e32b)
    // rain                  | weather-rain                   (e318)
    // snow                  | weather-snow                   (e31a)
    // sleet                 | weather-sleet                  (e3ad)
    // wind                  | weather-windy                  (e31e)
    // fog                   | weather-fog                    (e313)
    // cloudy                | weather-cloudy                 (e312)
    // partly-cloudy-day     | weather-day_cloudy             (e302)
    // partly-cloudy-night   | weather-night_alt_partly_cloudy (e379)
    // (unknown)             | weather-na                     (e374)
    let (ico, color) = match plain {
        "clear-day" => ('\u{e30d}', "#fabd2f"),
        "clear-night" => ('\u{e32b}', "#d5c4a1"),
        "rain" => ('\u{e318}', "#83a598"),
        "snow" => ('\u{e31a}', "#fbf1c7"),
        "sleet" => ('\u{e3ad}', "#d3869b"),
        "wind" => ('\u{e31e}', "#ebdbb2"),
        "fog" => ('\u{e313}', "#928374"),
        "cloudy" => ('\u{e312}', "#d5c4a1"),
        "partly-cloudy-day" => ('\u{e302}', "#d79921"),
        "partly-cloudy-night" => ('\u{e379}', "#bdae93"),
        _ => ('\u{e374}', "#fb4934"), // unknown icon — weather-na
    };
    format!("<span color =\"{color}\">{ico}</span>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_forecast_url_encodes_path_segments() {
        let url = build_forecast_url("key/with/slash", 37.77, -122.42).unwrap();
        assert_eq!(
            url.as_str(),
            "https://api.pirateweather.net/forecast/key%2Fwith%2Fslash/37.77,-122.42?units=auto"
        );
    }

    #[test]
    fn encode_icon_known_weather_types() {
        let cases = [
            ("clear-day", '\u{e30d}', "#fabd2f"),
            ("clear-night", '\u{e32b}', "#d5c4a1"),
            ("rain", '\u{e318}', "#83a598"),
            ("snow", '\u{e31a}', "#fbf1c7"),
            ("sleet", '\u{e3ad}', "#d3869b"),
            ("wind", '\u{e31e}', "#ebdbb2"),
            ("fog", '\u{e313}', "#928374"),
            ("cloudy", '\u{e312}', "#d5c4a1"),
            ("partly-cloudy-day", '\u{e302}', "#d79921"),
            ("partly-cloudy-night", '\u{e379}', "#bdae93"),
        ];

        for (icon, glyph, color) in cases {
            let got = encode_icon(icon);
            assert_eq!(got, format!("<span color =\"{color}\">{glyph}</span>"));
        }
    }

    #[test]
    fn encode_icon_unknown_falls_back_to_weather_na() {
        let got = encode_icon("hail");
        assert_eq!(got, "<span color =\"#fb4934\">\u{e374}</span>");
    }
}
