use anyhow::Result;
use structopt::StructOpt;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "DateIssue")]
    pub date_issue: String,
    #[serde(rename = "DateForecast")]
    pub date_forecast: String,
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
    #[serde(rename = "ActionDay")]
    pub action_day: bool,
    #[serde(rename = "Discussion")]
    pub discussion: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "Number")]
    pub number: i64,
    #[serde(rename = "Name")]
    pub name: String,
}

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
    println!("{:?}", opts);

    let zip = opts.zipcode_or_latitude;
    let key = env!("AIRNOW_API_KEY");
    let url = format!("http://www.airnowapi.org/aq/forecast/zipCode/?format=application/json&zipCode={}&API_KEY={}", zip, key);
    
    let resp = reqwest::get(&url)
        .await?
        .json::<Vec<Root>>()
        .await?;
    let data = resp.first().expect("no data available");
    println!("{}\t{}\t{}", data.aqi, data.category.name, data.discussion);

    Ok(())
}