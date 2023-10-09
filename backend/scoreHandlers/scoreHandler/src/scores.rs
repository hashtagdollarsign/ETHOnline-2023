use std::fmt::{Display, Formatter};
use lambda_http::{Error};
use urlencoding::encode;

#[derive(serde::Deserialize)]
pub struct ScoreValue {
    id: u64,
    address: String,
    score: u128,
}

#[derive(serde::Deserialize)]
pub struct ScoreData {
    scores: Vec<ScoreValue>,
}

impl Display for ScoreValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"id:{}, address:{}, score:{}", self.id,self.address,self.score)
    }
}

impl Display for ScoreData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for score in &self.scores {
            write!(f, "{}\n", score)?;
        }
        Ok(())
    }
}

pub async fn get_scores(table_name: &str) -> Result<ScoreData, Error> {
    let sql = format!("SELECT * FROM {table_name}");
    let encoded_sql = encode(&sql);
    let url = format!("https://testnets.tableland.network/api/v1/query?statement={encoded_sql} ");
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    dbg!(&resp);
    let scores: Vec<ScoreValue> = serde_json::from_str(&resp)?;
    let score_data = ScoreData { scores };
    Ok(score_data)
}