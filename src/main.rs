#![allow(unused)]

use geo::prelude::*;
use geo::point;
use carapax::types::{
	Message, MessageData, InputFile, 
	InlineKeyboardButton, TextEntity,
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
use carapax::types::User;
use serde::{Deserialize, Serialize};

use crate::catalog::kofe_list;

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

//fn calculator(lat_user: f64, lon_user: f64, location_x: f64, location_y: f64) -> i32 {
    //let earth_radius_kilometer = 6371.0_f64;
    //let (user_latitude_degrees, user_longitude_degrees) = (lat_user, lon_user);
    //let (caffee_latitude_degrees, caffee_longitude_degrees) = (location_x, location_y);

    //let user_latitude = user_latitude_degrees.to_radians();
    //let caffee_latitude = caffee_latitude_degrees.to_radians();

    //let delta_latitude = (user_latitude_degrees - caffee_latitude_degrees).to_radians();
    //let delta_longitude = (user_longitude_degrees - caffee_longitude_degrees).to_radians();

    //let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        //+ user_latitude.cos() * caffee_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    //let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    //let result = (earth_radius_kilometer * central_angle * 100000.0).round() as i32;
    //result
    //((lat_user * lon_user * 1000000.0) - (location_x * location_y * 1000000.0)).abs() as i32
//}

fn distance(lat_user: f64, lon_user: f64) -> (
String, String, String, 
String, String, String, 
String, String, String, 
String, String, String ) {
    // dbg!(&lat_user);
    // dbg!(&lon_user);
    let mut temporary_collection = vec![];

    let point_user = point!(x: lat_user, y: lon_user);
// ITERATION
    for index in 0..kofe_list().len() {
		//let calculated_distance: i32 = calculator(lat_user, lon_user, kofe_list()[index].location_x, kofe_list()[index].location_y);
        //dbg!(&calculated_distance);
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
		dbg!(&temporary_collection[0].0.clone());
		dbg!(&temporary_collection[0].4.clone());
		dbg!(&temporary_collection[1].0.clone());
		dbg!(&temporary_collection[1].4.clone());
		dbg!(&temporary_collection[2].0.clone());
		dbg!(&temporary_collection[2].4.clone());
    let one   = format!("{}\n", temporary_collection[0].1);
    let two   = format!("{}\n", temporary_collection[1].1);
    let three = format!("{}\n", temporary_collection[2].1);
    (one,  temporary_collection[0].2.clone(), temporary_collection[0].3.clone(), temporary_collection[0].4.clone(), 
    two,   temporary_collection[1].2.clone(), temporary_collection[1].3.clone(), temporary_collection[1].4.clone(), 
    three, temporary_collection[2].2.clone(), temporary_collection[2].3.clone(), temporary_collection[2].4.clone() )
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
	//let method = SendMessage::new(chat_id.clone(), "TEST".to_string()).reply_markup(vec![vec![
        //InlineKeyboardButton::with_url("TEST", "https://duckduckgo.com/".to_string()),
    //]]);
    //api.execute(method).await?;
    // dbg!(&message);
    if let MessageData::Location(location) = message.data {
        let lon: f64 = location.longitude.into();
        let lat: f64 = location.latitude.into();
        let calculated_distance = distance(lon, lat);
        dbg!(&calculated_distance.3);
        dbg!(&calculated_distance.7);
        dbg!(&calculated_distance.11);
// 1st Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.1.clone())
                    .await
                    .unwrap(),
            )
            .caption(calculated_distance.0)
            .caption_entities(&[TextEntity::bold(0..10)]).unwrap(),
        )
        .await?;
// BUTTON №1
        let method = SendMessage::new(
			chat_id.clone(), calculated_distance.3.to_string()
		).reply_markup(vec![vec![
            InlineKeyboardButton::with_url(
            	"📍Посмотреть на карте", calculated_distance.2.to_string()
            ),
        ]]);
        api.execute(method).await?;
// 2nd Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.5).await.unwrap(),
            )
            .caption(calculated_distance.4)
            .caption_entities(&[TextEntity::bold(0..10)]).unwrap(),
        )
        .await?;
// BUTTON №2
        let method = SendMessage::new(
			chat_id.clone(), calculated_distance.7.to_string()
		).reply_markup(vec![vec![
            InlineKeyboardButton::with_url(
				"📍Посмотреть на карте", calculated_distance.6.to_string()
			),
        ]]);
        api.execute(method).await?;
// 3rd Cafe
        api.execute(
            SendPhoto::new(
                chat_id.clone(),
                InputFile::path(calculated_distance.9).await.unwrap(),
            )
            .caption(calculated_distance.8)
            .caption_entities(&[TextEntity::bold(0..10)]).unwrap(),
        )
        .await?;
// BUTTON №3
        let method = SendMessage::new(
			chat_id.clone(), calculated_distance.11.to_string()
		).reply_markup(vec![vec![
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn fabrika_eleven_window() {
        let list_1 = distance(41.709530, 44.802610); // FABRIKA
        let list_2 = distance(41.695242, 44.793238); // Eleven.window
        assert_ne!(list_1, list_2);
	}
	#[test]
	fn lamarzocco_eleven_window() {
        let list_1 = distance(41.710275, 44.766336); // Lamarzocco
        let list_2 = distance(41.695242, 44.793238); // Eleven.window
        assert_ne!(list_1, list_2);
	}
	//#[test]
	//fn calculator_1() {
		//let calc_1 = calculator(41.710275, 44.766336, 41.710275, 44.766336); // Lamarzocco + Lamarzocco
		//let calc_2 = calculator(41.695242, 44.793238, 41.710275, 44.766336); // Eleven.window + Lamarzocco
		//let calc_3 = calculator(41.708954, 44.755021, 41.693138, 44.802658); // Erti Kava + Daily grind
		//assert_ne!(calc_1, calc_2);
		//assert_ne!(calc_1, calc_3);
		//assert_ne!(calc_2, calc_3);
        ////for index in 0..kofe_list().len() {
			////calculator(lat_user: f64, lon_user: f64, kofe_list()[index].location_x, kofe_list()[index].location_y)
		////}
	//}
}
