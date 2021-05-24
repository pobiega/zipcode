use serde::Deserialize;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    match get_zip_data(&args.country, &args.zipcode) {
        Err(why) => panic!("{:?}", why),
        Ok(value) => {
            println!("{}, {}", value.country, value.places[0].place_name);
        }
    }
}

fn get_zip_data(country: &str, zip: &str) -> Result<Zippotam, ureq::Error> {
    let url = format!("http://api.zippopotam.us/{}/{}", country, zip);
    let response: Zippotam = ureq::get(&url).call()?.into_json()?;
    Ok(response)
}

#[derive(StructOpt)]
struct Cli {
    country: String,
    zipcode: String,
}

#[derive(Deserialize, Debug)]
struct Zippotam {
    #[serde(rename = "post code")]
    post_code: String,
    country: String,
    places: Vec<Place>,
}

#[derive(Deserialize, Debug)]
struct Place {
    #[serde(rename = "place name")]
    place_name: String,
    state: String,
}
