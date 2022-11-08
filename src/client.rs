use std::{error::Error, time::Duration};

use graphql_client::GraphQLQuery;
use reqwest::header::HeaderValue;

const FFLOGS_API_URL: &str = "https://www.fflogs.com/api/v2/client";
const RATE_LIMIT_POINT_THRESHOLD: f64 = 100.0;

pub struct RateLimitInfo {
    pub limit_per_hour: i64,
    pub points_spent_this_hour: f64,
    pub points_reset_in: i64,
}
pub trait RateLimitableQuery: GraphQLQuery {
    fn get_rate_limit_data(
        response: &<Self as GraphQLQuery>::ResponseData,
    ) -> Option<RateLimitInfo>;
}

pub struct Client {
    client: reqwest::Client,
}
impl Client {
    pub fn new(api_token: &str) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .user_agent("Mechanic Visualizer")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", api_token)).unwrap(),
                ))
                .collect(),
            )
            .build()?;

        Ok(Client { client })
    }

    pub async fn query<Q: RateLimitableQuery>(
        &self,
        vars: <Q as GraphQLQuery>::Variables,
    ) -> Result<<Q as GraphQLQuery>::ResponseData, Box<dyn Error>> {
        let resp =
            graphql_client::reqwest::post_graphql::<Q, _>(&self.client, FFLOGS_API_URL, vars)
                .await?;

        if let Some(errors) = resp.errors {
            let mut error_text = String::new();
            for err in errors {
                error_text.push_str(&format!("{}", err));
                error_text.push_str("\n");
            }
            return Err(error_text.into());
        }

        let resp = resp.data.expect("no errors and no data in response");

        if let Some(rate_limit_data) = Q::get_rate_limit_data(&resp) {
            Self::enforce_rate_limit(&rate_limit_data).await?;
        }

        Ok(resp)
    }

    async fn enforce_rate_limit(data: &RateLimitInfo) -> Result<(), Box<dyn Error>> {
        let points_remaining = data.limit_per_hour as f64 - data.points_spent_this_hour;

        if points_remaining < RATE_LIMIT_POINT_THRESHOLD {
            let time_to_wait = Duration::from_secs(data.points_reset_in.try_into()?);
            println!("Sleeping for {:?} for rate limit!", time_to_wait);
            tokio::time::sleep(time_to_wait).await;
        }

        Ok(())
    }
}
