use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::Write,
    time::Duration,
};

use client::Client;
use events::Event;
use humantime::format_duration;
use positions::PositionHistory;
use queries::report_fights::ReportFightsReportDataReport;
use serde_json::Value;

use crate::video::render_animations;

mod client;
mod events;
mod positions;
mod queries;
mod video;

// const P5S_ENCOUNTER_ID: i64 = 83;
// const P6S_ENCOUNTER_ID: i64 = 84;
// const P7S_ENCOUNTER_ID: i64 = 85;
// const P8S_P1_ENCOUNTER_ID: i64 = 86;
// const P8S_P2_ENCOUNTER_ID: i64 = 87;

#[derive(Debug)]
pub struct ActorInfo {
    pub name: String,
    type_: String,
    subtype: String,
}

fn get_report_codes(character_rankings: &Value) -> Option<Vec<String>> {
    let mut result = Vec::new();

    for report in character_rankings.as_object()?.get("ranks")?.as_array()? {
        let report = report.as_object()?.get("report")?.as_object()?;
        let code = report.get("code")?.as_str()?;

        result.push(code.to_string());
    }

    Some(result)
}

async fn load_all_events(
    client: &Client,
    code: &str,
    start_time: f64,
    end_time: f64,
    fight_id: i64,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut result = Vec::new();

    let mut page_start = start_time;
    while page_start < end_time {
        let events = client
            .query::<queries::ReportEvents>(queries::report_events::Variables {
                code: code.to_string(),
                start_time: page_start,
                end_time: end_time,
                fight_ids: vec![fight_id],
            })
            .await?;

        let mut report = events.report_data.unwrap().report.unwrap().events.unwrap();
        let events = report.data.as_mut().unwrap();
        result.append(events);

        page_start = if let Some(page_start) = report.next_page_timestamp {
            page_start
        } else {
            break;
        };
    }

    Ok(result)
}

async fn handle_fight(
    client: &Client,
    code: &str,
    report_data: &ReportFightsReportDataReport,
    idx: usize,
    actors: &HashMap<i64, ActorInfo>,
) -> Result<(), Box<dyn Error>> {
    let fight_data = report_data.fights.as_ref().unwrap()[idx].as_ref().unwrap();
    let fight_start_time = fight_data.start_time;
    let fight_end_time = fight_data.end_time;
    let fight_length = Duration::from_millis((fight_end_time - fight_start_time) as u64);
    let bounding_box = fight_data.bounding_box.as_ref().unwrap();
    let bounding_box = (
        (bounding_box.min_x as f64, bounding_box.min_y as f64),
        (bounding_box.max_x as f64, bounding_box.max_y as f64),
    );

    println!("Loading report...");

    let events = load_all_events(
        client,
        code,
        fight_data.start_time,
        fight_data.end_time,
        fight_data.id,
    )
    .await?;

    println!(
        "Loaded {} events! ({} events/second of fight)",
        events.len(),
        events.len() as f64 / fight_length.as_secs_f64()
    );

    // id -> position history
    let mut position_history: HashMap<i64, PositionHistory> = HashMap::with_capacity(actors.len());

    for event in events {
        let time = event.get_timestamp();

        if let Some((id, res)) = event.get_source_resources() {
            position_history
                .entry(id)
                .or_default()
                .add_update(time, (res.x as f64, res.y as f64));
        }

        if let Some((id, res)) = event.get_target_resources() {
            position_history
                .entry(id)
                .or_default()
                .add_update(time, (res.x as f64, res.y as f64));
        }
    }

    render_animations(
        &position_history,
        actors,
        fight_data.start_time,
        fight_data.end_time,
        bounding_box,
        1024,
        "output",
    );

    Ok(())
}

async fn read_report(client: &Client, code: &str) -> Result<(), Box<dyn Error>> {
    let fight_data = client
        .query::<queries::ReportFights>(queries::report_fights::Variables {
            code: code.to_string(),
        })
        .await?
        .report_data
        .unwrap()
        .report
        .unwrap();

    let actors = fight_data
        .master_data
        .as_ref()
        .unwrap()
        .actors
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|actor| {
            let actor = actor.as_ref().unwrap();
            let data = ActorInfo {
                name: actor.name.as_ref().unwrap().clone(),
                type_: actor.type_.as_ref().unwrap().clone(),
                subtype: actor.sub_type.as_ref().unwrap().clone(),
            };
            (actor.id.unwrap() as i64, data)
        })
        .collect::<HashMap<_, _>>();

    let fights = fight_data.fights.as_ref().unwrap();
    for (i, fight) in fights.iter().enumerate() {
        let fight = fight.as_ref().unwrap();

        let mut enemies = fight
            .enemy_np_cs
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|npc| {
                let id = npc.as_ref().unwrap().id.unwrap();
                let name = &actors.get(&id).as_ref().unwrap().name;
                if name == "Multiple Enemies" {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        enemies.sort();

        let kill = fight.kill.unwrap();
        let fight_duration = Duration::from_millis((fight.end_time - fight.start_time) as u64);
        let kill_text = if kill {
            format!("killed in {}", format_duration(fight_duration))
        } else {
            format!(
                "wiped at {}% after {}",
                fight.fight_percentage.unwrap(),
                format_duration(fight_duration)
            )
        };

        println!(
            "{i}: Fight {} against {enemies:?} ({})",
            fight.id, kill_text
        );
    }

    let mut buf = String::new();
    let sel: usize = loop {
        print!("Select a fight: ");
        std::io::stdout().flush().unwrap();
        if let Err(e) = std::io::stdin().read_line(&mut buf) {
            println!("{:?}", e);
            continue;
        }
        if let Ok(sel) = buf.trim().parse() {
            if sel < fights.len() {
                break sel;
            }
        }
    };

    handle_fight(client, code, &fight_data, sel, &actors).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let args = std::env::args().collect::<Vec<_>>();

    let api_token = std::env::var("FFLOGS_API_TOKEN").expect("Missing API access token");
    let client = Client::new(&api_token)?;

    // let response = client
    //     .query::<queries::IndividualCharacter>(queries::individual_character::Variables {
    //         id: MY_CHARACTER_ID,
    //         encounter_id: P5S_ENCOUNTER_ID,
    //     })
    //     .await?;

    // // println!("{:#?}", response);

    // let report_codes = get_report_codes(
    //     &response
    //         .character_data
    //         .unwrap()
    //         .character
    //         .unwrap()
    //         .encounter_rankings
    //         .unwrap(),
    // )
    // .unwrap();

    // println!("{:#?}", report_codes);

    read_report(&client, &args[1]).await?;

    // let f = std::io::BufReader::new(std::fs::File::open("test.json").unwrap());
    // let events: Vec<Event> = serde_json::from_reader(f).unwrap();

    // dbg!(events.len());

    Ok(())
}
