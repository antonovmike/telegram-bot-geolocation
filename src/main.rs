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


#[derive(Deserialize, Serialize)]
struct CallbackData {
    value: String,
}

impl CallbackData {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: value.into() }
    }
}

fn distance(lat_user: f32, lon_user: f32) -> (
String, String, String, 
String, String, String, 
String, String, String, 
String, String, String
) {
    // dbg!(&lat_user);
    // dbg!(&lon_user);
    let mut temporary_collection = vec![];

    let point_user = point!(x: lat_user, y: lon_user);
// ITERATION
    for index in 0..kofe_list().len() {
        let point_destination = point!(x: kofe_list()[index].location_x, y: kofe_list()[index].location_y);
        let calculated_distance: i32 = point_user.haversine_distance(&point_destination).round() as i32;
        temporary_collection.push((
			calculated_distance,                    // 0
			kofe_list()[index].description.clone(), // 1
			kofe_list()[index].photo.clone(),       // 2
			kofe_list()[index].google_maps.clone(), // 3
			kofe_list()[index].address.clone(),     // 4
		));
    }
    temporary_collection.sort_by(|a, b| a.0.cmp(&b.0));
// CAFFEE DESCRIPTION
    let one   = format!("{}\n", temporary_collection[0].1);
    let two   = format!("{}\n", temporary_collection[1].1);
    let three = format!("{}\n", temporary_collection[2].1);
//   0/4/8  1/5/9 PHOTO                        2/6/10 MAPS                        3/7/11 ADDRESS
    (one,   temporary_collection[0].2.clone(), temporary_collection[0].3.clone(), temporary_collection[0].4.clone(), 
     two,   temporary_collection[1].2.clone(), temporary_collection[1].3.clone(), temporary_collection[1].4.clone(), 
     three, temporary_collection[2].2.clone(), temporary_collection[2].3.clone(), temporary_collection[2].4.clone() )
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    // dbg!(&message);
    if let MessageData::Location(location) = message.data {
        let lon = location.longitude;
        let lat = location.latitude;
        let calculated_distance = distance(lon, lat);
// 1st Cafe
// 0 description 1 photo 2 maps 3 address
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
        let method = SendMessage::new(chat_id.clone(), calculated_distance.3.to_string()).reply_markup(vec![vec![
            InlineKeyboardButton::with_url(
            	"📍Посмотреть на карте", calculated_distance.2.to_string()
            ),
        ]]);
        api.execute(method).await?;
// 2nd Cafe
// 4 description 5 photo 6 maps 7 address
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.5.clone())
					.await
					.unwrap(),
            )
            .caption(calculated_distance.4),
        )
        .await?;
        // BUTTON №2
        let method = SendMessage::new(chat_id.clone(), calculated_distance.7.to_string()).reply_markup(vec![vec![
            InlineKeyboardButton::with_url(
            	"📍Посмотреть на карте", calculated_distance.6.to_string()
            ),
        ]]);
        api.execute(method).await?;
// 3rd Cafe
// 8 description 9 photo 10 maps 11 address
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.9.clone())
					.await
					.unwrap(),
            )
            .caption(calculated_distance.8),
        )
        .await?;
// BUTTON №3
        let method = SendMessage::new(chat_id.clone(), calculated_distance.11.to_string()).reply_markup(vec![vec![
            InlineKeyboardButton::with_url(
            	"📍Посмотреть на карте", calculated_distance.10.to_string()
            ),
        ]]);
        api.execute(method).await?;
        // dbg!("F");
    } else {
		let warning_message = "Привет! Чтобы найти ближайшую кофейню, пожалуйста пришли свою гео-локацию в этот чат.".to_string();
		let method = SendMessage::new(chat_id.clone(), warning_message);
        api.execute(method).await?;
//"Добро пожаловать, бла бла, для того, чтобы найти ближайшие кафе, пожалуйста пришлите свою гео-позицию"
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
