use serde::Serialize;

use self::series::Series;

pub mod series;
mod title;



#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EChartOption {
    // pub title: Title,
    pub series: Vec<Series>
}