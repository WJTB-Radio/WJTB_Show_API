#[macro_use] extern crate rocket;
use serde::Serialize;
use chrono::prelude::*;
use chrono_tz::America::New_York;

#[derive(Serialize)]
struct DayResponse {
	day: String,
	shows: Vec<Show>,
}

#[derive(Serialize)]
struct Show {
	name: String,
	desc: String,
	hosts: String,
	poster: String,
	start_time: i64,
	end_time: i64,
	is_running: i64,
}

#[derive(Serialize)]
struct PlayingResponse {
	name: String,
	error: String,
	end_time: i64,
}

#[get("/shows/<day>")]
fn get_shows(day: &str) -> Option<String> {
	// now we dont need to worry about sql injection. yay!!!!
	if !(day == "monday" || day == "tuesday" || day == "wednesday" || day == "thursday" || day == "friday") {
		return None;
	}
	let connection = sqlite::Connection::open_with_flags("database.sqlite3", sqlite::OpenFlags::new().set_read_only()).expect("Couldn't open database.");
	// for whatever reason, sqlite doesnt like it when we bind the table name
	// so we are using format strings for the table name
	// this is really bad in general (sql injection),
	// but fine here because we made sure that day can only be the name of a day of the week.
	let mut statement = connection.prepare(format!("SELECT * FROM {}", day)).expect("prepare query failed.");
	let mut shows:Vec<Show> = Vec::new();
	while let Ok(sqlite::State::Row) = statement.next() {
		let show = Show {
			name: statement.read("name").unwrap(),
			desc: statement.read("desc").unwrap(),
			hosts: statement.read("hosts").unwrap(),
			poster: statement.read("poster").unwrap(),
			start_time: statement.read("start_time").unwrap(),
			end_time: statement.read("end_time").unwrap(),
			is_running: statement.read("is_running").unwrap(),
		};
		shows.push(show);
	}
	Some(
		serde_json::to_string(
			&DayResponse {
				day: day.to_string(),
				shows: shows,
			}
		)
		.expect("couldn't serialize data.")
	)
}

#[get("/playing")]
fn get_playing() -> String {
	// all times are relative to new york time cuz thats where we are
	let date = Utc::now().with_timezone(&New_York);
	let weekday = date.weekday();
	// if we ever have shows that run on the weekends, we need to change this
	if weekday == Weekday::Sat || weekday == Weekday::Sun {
		// no shows run today, return the end of the day
		// note: 86400 = 24*60*60
		return "{\"name\":\"\", \"error\":\"no-show\", \"end_time\":86400}".to_string();
	}
	let day:&str = match weekday {
		Weekday::Mon => {"monday"},
		Weekday::Tue => {"tuesday"},
		Weekday::Wed => {"wednesday"},
		Weekday::Thu => {"thursday"},
		Weekday::Fri => {"friday"},
		Weekday::Sat => {"monday"},
		Weekday::Sun => {"monday"},
	};
	let time = date.time();
	// number of seconds since the start of the day
	let day_seconds:i64 = (time.hour() as i64)*60*60+(time.minute() as i64)*60+(time.second() as i64);
	let connection = sqlite::Connection::open_with_flags("database.sqlite3", sqlite::OpenFlags::new().set_read_only()).expect("Couldn't open database.");
	// select the currently running show
	// for whatever reason, sqlite doesnt like it when we bind the table name
	// so we are using format strings for the table name
	// this is really bad in general (sql injection),
	// but fine here because we made sure that day can only be the name of a day of the week.
	let mut statement = connection.prepare(format!("SELECT name, end_time FROM {} WHERE end_time >= ? AND start_time <= ? AND is_running = 1;", day)).expect("prepare query failed.");
	statement.bind((1, day_seconds)).expect("bind query failed.");
	statement.bind((2, day_seconds)).expect("bind query failed.");
	let result = statement.next();
	// note we are only reading one row, if multiple shows are overlapping,
	// we will return the first one that the query returns
	return match result {
		Ok(sqlite::State::Row) => {
			serde_json::to_string(
				&PlayingResponse {
					name: statement.read("name").unwrap(),
					error: "none".to_string(),
					end_time: statement.read("end_time").unwrap(),
				}
			).expect("couldn't serialize data.")
		}
		Err(_) | Ok(sqlite::State::Done) => {
			// there isnt a show currently playing, we need to figure out when the next show starts
			let mut statement = connection.prepare(format!("SELECT name, start_time FROM {} Where start_time > ? AND end_time > ? ORDER BY start_time;", day)).expect("prepare query failed.");
			statement.bind((1, day_seconds)).expect("bind query failed.");
			statement.bind((2, day_seconds)).expect("bind query failed.");
			let result = statement.next();
			match result{
				Ok(sqlite::State::Row) => {
					serde_json::to_string(&PlayingResponse {
						name: statement.read("name").unwrap(),
						error: "no-show".to_string(),
						end_time: statement.read("start_time").unwrap(),
					}).expect("couldn't serialize data")
				}
				// if there arent any more shows today, we will return the end of the day
				// note: 86400 = 24*60*60
				Err(_) | Ok(sqlite::State::Done) => {
					"{\"name\":\"\", \"error\":\"no-show\", \"end_time\":86400}".to_string()
				}
			}
		}
	};
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/api", routes![get_shows, get_playing])
}
