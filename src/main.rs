#![allow(unused)]

use geo::prelude::*;
use geo::point;
use carapax::types::{
	Message, MessageData, InputFile, 
	InlineKeyboardButton,
};
use carapax::methods::SendPhoto;
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;

use crate::catalog::kofe_list;

use serde::{Deserialize, Serialize};

mod catalog;

pub enum InlineKeyboardButtonKind {
    Url(String),
    CallbackData(String),
}

#[derive(Deserialize, Serialize)]
struct CallbackData {
    value: String,
}

impl CallbackData {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: value.into() }
    }
}
// URL
#[derive(Deserialize, Serialize)]
struct Url {
    value: String,
}

impl Url {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: "https://duckduckgo.com/".to_string() }
    }
}

fn distance(lat_user: f32, lon_user: f32) -> (String, String, String, String, String, String) {
    // dbg!(&lat_user);
    // dbg!(&lon_user);
    let mut temporary_collection = vec![];

    let point_user = point!(x: lat_user, y: lon_user);
// ITERATION
    for index in 0..kofe_list().len() {
        let point_destination = point!(x: kofe_list()[index].location_x, y: kofe_list()[index].location_y);
        let calculated_distance: i32 = point_user.haversine_distance(&point_destination).round() as i32;
        //temporary_collection.push((calculated_distance, kofe_list()[index].description.clone() ));
        temporary_collection.push((calculated_distance, kofe_list()[index].description.clone(), kofe_list()[index].photo.clone() ));
    }
    temporary_collection.sort_by(|a, b| a.0.cmp(&b.0));
    let one = format!("{}\n", temporary_collection[0].1);
    let two = format!("{}\n", temporary_collection[1].1);
    let three = format!("{}\n", temporary_collection[2].1);
    (one, temporary_collection[0].2.clone(), two, temporary_collection[1].2.clone(), three, temporary_collection[2].2.clone())
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    // dbg!(&message);
    if let MessageData::Location(location) = message.data {
        let lon = location.longitude;
        let lat = location.latitude;
        let calculated_distance = distance(lon, lat);
        // 1st Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.1.clone())
                    .await
                    .unwrap(),
            )
            .caption(calculated_distance.0),
        )
        .await?;
        // BUTTON №1
        let callback_data = "https://duckduckgo.com/";
        let method = SendMessage::new(chat_id.clone(), "🔗").reply_markup(vec![vec![
            InlineKeyboardButton::with_url("DEMO BUTTON №1", callback_data.to_string()),
        ]]);
        api.execute(method).await?;
        // 2nd Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.3).await.unwrap(),
            )
            .caption(calculated_distance.2),
        )
        .await?;
        // BUTTON №2
        let method = SendMessage::new(chat_id.clone(), "🔗").reply_markup(vec![vec![
            InlineKeyboardButton::with_url("DEMO BUTTON №2", callback_data.to_string()),
        ]]);
        api.execute(method).await?;
        // 3rd Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.5).await.unwrap(),
            )
            .caption(calculated_distance.4),
        )
        .await?;
        // BUTTON №3
        let method = SendMessage::new(chat_id.clone(), "🔗").reply_markup(vec![vec![
            InlineKeyboardButton::with_url("DEMO BUTTON №3", callback_data.to_string()),
        ]]);
        api.execute(method).await?;
        // dbg!("F");
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}
