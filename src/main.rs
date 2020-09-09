use anyhow::Result;

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

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();

    let env = env_logger::Env::default().filter_or("AQI_LOG", "info");
    env_logger::init_from_env(env);
    log::info!("woof");
    Ok(())
}
