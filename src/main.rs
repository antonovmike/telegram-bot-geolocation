#![allow(unused)]

use geo::prelude::*;
use geo::point;
use carapax::types::{Message, MessageData, InputFile};
use carapax::methods::SendPhoto;
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;

use std::path::Path;

use crate::catalog::CoffeeHouse;
use crate::catalog::kofe_list;

mod catalog;

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
        let method = SendMessage::new(chat_id.clone(), calculated_distance.0);
        api.execute(method).await?;
		let method = SendPhoto::new(
			chat_id.clone(),
			InputFile::path(calculated_distance.1)
				.await
				.unwrap(),
		);
		api.execute(method).await?;
// 2nd Cafe
		let method = SendMessage::new(chat_id.clone(), calculated_distance.2);
        api.execute(method).await?;
        let method = SendPhoto::new(
			chat_id.clone(),
			InputFile::path(calculated_distance.3)
				.await
				.unwrap(),
		);
		api.execute(method).await?;
// 3rd Cafe
		let method = SendMessage::new(chat_id.clone(), calculated_distance.4);
        api.execute(method).await?;
        let method = SendPhoto::new(
			chat_id.clone(),
			InputFile::path(calculated_distance.5)
				.await
				.unwrap(),
		);
		api.execute(method).await?;
    };
    // dbg!("F");
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
