use anyhow::Result;
use structopt::StructOpt;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "DateObserved")]
    pub date_observed: String,
    #[serde(rename = "HourObserved")]
    pub hour_observed: i64,
    #[serde(rename = "LocalTimeZone")]
    pub local_time_zone: String,
    #[serde(rename = "ReportingArea")]
    pub reporting_area: String,
    #[serde(rename = "StateCode")]
    pub state_code: String,
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[serde(rename = "AQI")]
    pub aqi: i64,
    #[serde(rename = "Category")]
    pub category: Category,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "Number")]
    pub number: i64,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Retrieve AQI info from [AirNow](https://www.airnow.gov/)
///
/// Pass either the zipcode, or the lat/long pair (as seperate arguments, may need)
/// to use `--` to clarify for the command line parser that a negative longitude
/// is not an option :-)
///
/// Examples:
/// 
///     # aqi 19808
/// 
///     # aqi -- 47.63105303247004 -122.51181783709531
#[derive(StructOpt, Debug)]
struct Opt {
    /// Zipcode or latitude
    zipcode_or_latitude: String,

    /// Longitude
    longitude: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    let env = env_logger::Env::default().filter_or("AQI_LOG", "info");
    env_logger::init_from_env(env);

    let opts = Opt::from_args();
    let key = "09FA59FF-077F-4012-BD47-D093B4514249";

    let url = match opts.longitude {
        None => format!("http://www.airnowapi.org/aq/observation/zipCode/current/?format=application/json&zipCode={}&API_KEY={}", opts.zipcode_or_latitude, key),
        Some(long) => format!("http://www.airnowapi.org/aq/observation/latLong/current/?format=application/json&latitude={}&longitude={}&distance=25&API_KEY={}", opts.zipcode_or_latitude, long, key),       
    };

    let resp = reqwest::get(&url).await?.json::<Vec<Root>>().await?;

    resp.iter().for_each(|it| {
        println!(
            "{}\t{}\t{}T{}:00",
            it.aqi,
            it.category.name,
            it.date_observed.replace(" ", ""),
            it.hour_observed
        )
    });

    Ok(())
}
