use graphql_client::GraphQLQuery;

use crate::{client::{RateLimitableQuery, RateLimitInfo}, events};

type JSON = serde_json::Value;
#[allow(non_camel_case_types)]
type EVENTS_JSON = Vec<events::Event>;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "queries/schema.json",
//     query_path = "queries.graphql",
//     response_derives = "Debug"
// )]
// pub struct Regions;
// impl RateLimitableQuery for Regions {
//     fn get_rate_limit_data(response: &regions::ResponseData) -> Option<RateLimitInfo> {
//         if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
//             Some(RateLimitInfo {
//                 limit_per_hour: data.limit_per_hour,
//                 points_spent_this_hour: data.points_spent_this_hour,
//                 points_reset_in: data.points_reset_in,
//             })
//         } else {
//             None
//         }
//     }
// }

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/character.graphql",
    response_derives = "Debug"
)]
pub struct IndividualCharacter;
impl RateLimitableQuery for IndividualCharacter {
    fn get_rate_limit_data(response: &individual_character::ResponseData) -> Option<RateLimitInfo> {
        if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
            Some(RateLimitInfo {
                limit_per_hour: data.limit_per_hour,
                points_spent_this_hour: data.points_spent_this_hour,
                points_reset_in: data.points_reset_in,
            })
        } else {
            None
        }
    }
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/fights.graphql",
    response_derives = "Debug"
)]
pub struct ReportFights;
impl RateLimitableQuery for ReportFights {
    fn get_rate_limit_data(response: &report_fights::ResponseData) -> Option<RateLimitInfo> {
        if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
            Some(RateLimitInfo {
                limit_per_hour: data.limit_per_hour,
                points_spent_this_hour: data.points_spent_this_hour,
                points_reset_in: data.points_reset_in,
            })
        } else {
            None
        }
    }
}



#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/report.graphql",
    response_derives = "Debug"
)]
pub struct ReportEvents;
impl RateLimitableQuery for ReportEvents {
    fn get_rate_limit_data(response: &report_events::ResponseData) -> Option<RateLimitInfo> {
        if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
            Some(RateLimitInfo {
                limit_per_hour: data.limit_per_hour,
                points_spent_this_hour: data.points_spent_this_hour,
                points_reset_in: data.points_reset_in,
            })
        } else {
            None
        }
    }
}



// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.json",
//     query_path = "queries.graphql",
//     response_derives = "Debug"
// )]
// pub struct FightRankings;
// impl RateLimitableQuery for FightRankings {
//     fn get_rate_limit_data(response: &fight_rankings::ResponseData) -> Option<RateLimitInfo> {
//         if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
//             Some(RateLimitInfo {
//                 limit_per_hour: data.limit_per_hour,
//                 points_spent_this_hour: data.points_spent_this_hour,
//                 points_reset_in: data.points_reset_in,
//             })
//         } else {
//             None
//         }
//     }
// }

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.json",
//     query_path = "queries.graphql",
//     response_derives = "Debug"
// )]
// pub struct Guilds;
// impl RateLimitableQuery for Guilds {
//     fn get_rate_limit_data(response: &guilds::ResponseData) -> Option<RateLimitInfo> {
//         if let Some(data) = response.rate_limit.rate_limit_data.as_ref() {
//             Some(RateLimitInfo {
//                 limit_per_hour: data.limit_per_hour,
//                 points_spent_this_hour: data.points_spent_this_hour,
//                 points_reset_in: data.points_reset_in,
//             })
//         } else {
//             None
//         }
//     }
// }