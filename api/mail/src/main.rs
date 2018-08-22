#[macro_use]
extern crate failure;
extern crate aws_lambda as lambda;
extern crate rusoto_core;
extern crate rusoto_ses;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate askama;
extern crate reqwest;

use askama::Template;
use failure::Error;
use lambda::event::sns::SnsEvent;
use reqwest::header::{Authorization, Basic};
use reqwest::StatusCode;
use rusoto_core::Region;
use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, Ses, SesClient};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::time::Duration;

#[derive(Template)]
#[template(path = "invite.html")]
struct InviteTemplate<'a> {
	inviter: &'a str,
	invitation_token: &'a str,
}

// https://github.com/srijs/rust-aws-lambda/blob/88904328b9f6a6ad016645a73a7acb41c08000cd/aws_lambda_events/src/generated/fixtures/example-sns-event.json
// https://github.com/srijs/rust-aws-lambda/blob/739e46049651576e366fadd9073c2e269d11baa2/aws_lambda_events/src/generated/sns.rs
fn main() {
	lambda::start(|event: SnsEvent| {
		let task = get_task(&event)?;

		generate_intermidiate(&task)
			.and_then(|intermediate| generate_html(&intermediate))
			.and_then(|html| send_email(&task, &html))
	})
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Task {
	Invite {
		inviter: String,
		email: String,
		invitation_token: String,
	},
}

fn get_task(event: &SnsEvent) -> Result<Task, Error> {
	let record = event
		.records
		.first()
		.ok_or(format_err!("Failed to get first record"))?;

	let message = record
		.clone()
		.sns
		.message
		.ok_or(format_err!("No message found"))?;

	let task: Task = serde_json::from_str(&message)?;

	Ok(task)
}

fn generate_intermidiate(task: &Task) -> Result<String, Error> {
	let template = match task {
		Task::Invite {
			inviter,
			invitation_token,
			..
		} => InviteTemplate {
			inviter: inviter,
			invitation_token: invitation_token,
		},
	};

	template.render().map_err(|e| format_err!("{}", e))
}

#[derive(Deserialize)]
struct intermediateResponse {
	html: String,
}

fn generate_html(intermediate: &str) -> Result<String, Error> {
	// let api_url = "https://api.intermediate.io/v1";

	// let client = reqwest::Client::new();

	// let mut params = HashMap::new();

	// params.insert("intermediate", intermediate);

	// let credentials = Basic {
	// 	username: "user".to_string(),
	// 	password: Some("passwd".to_string()),
	// };

	// let mut response = client
	// 	.post(api_url)
	// 	.header(Authorization(credentials))
	// 	.json(&params)
	// 	.send()?;

	// match response.status() {
	// 	StatusCode::Ok => {
	// 		let resp:intermediateResponse = response.json()?;
	// 		Ok(resp.html)
	// 	},
	// 	s => Err(format_err!("intermediate api responded with {}", s))
	// }
	Ok(intermediate.to_owned())
}

fn send_email(task: &Task, html: &str) -> Result<(), Error> {
	let config = get_config()?;

	let email = email_for_task(task);
	let from = config.system_email.to_owned();
	let to = vec![email.to_owned()];
	let subject = subject_for_task(task);

	let client = SesClient::new(Region::ApSoutheast1);

	let body = Body {
		html: Some(Content {
			charset: Some(String::from("UTF-8")),
			data: html.to_owned(),
		}),
		text: None,
	};

	let message = Message {
		body: body,
		subject: Content {
			charset: Some(String::from("UTF-8")),
			data: subject,
		},
	};

	let mut email: SendEmailRequest = Default::default();

	email.source = from;

	email.destination = Destination {
		to_addresses: Some(to),
		bcc_addresses: None,
		cc_addresses: None,
	};

	email.message = message;

	client
		.send_email(email)
		.with_timeout(Duration::from_secs(20))
		.sync()
		.map_err(|_| format_err!("Failed to send email"))
		.map(|_response| ())
}

fn email_for_task(task: &Task) -> String {
	match task {
		Task::Invite{email,..} => email.to_owned(),
	}
}

fn subject_for_task(task: &Task) -> String {
	match task {
		Task::Invite{..} => "You have been invited".to_owned(),
	}
}


struct Config {
	system_email: String,
}

fn get_config() -> Result<Config, Error> {
	let system_email = env::var("SYSTEM_EMAIL").map_err(|_| format_err!("SYSTEM_EMAIL not found"))?;

	Ok(Config {
		system_email: system_email,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::prelude::*;
	use lambda::event::sns::{SnsEntity, SnsEvent, SnsEventRecord};

	fn build_event(message: &str) -> SnsEvent {
		let message_attributes = HashMap::new();
		let timestamp = Utc::now();

		let record = SnsEventRecord {
			event_source: None,
			event_subscription_arn: None,
			event_version: None,
			sns: SnsEntity {
				signature: None,
				message_id: None,
				message: Some(message.to_owned()),
				message_attributes: message_attributes,
				signature_version: None,
				signing_cert_url: None,
				subject: None,
				timestamp: timestamp,
				topic_arn: None,
				type_: None,
				unsubscribe_url: None,
			},
		};

		SnsEvent {
			records: vec![record],
		}
	}

	#[test]
	fn it_gets_and_invite() {
		let bytes = include_bytes!("fixtures/invite.json");
		let json = String::from_utf8_lossy(bytes);

		let event = build_event(&json);

		let task = get_task(&event).unwrap();

		let expected = Task::Invite {
			inviter: "sam@sample.com".to_owned(),
			email: "sally@sample.com".to_owned(),
			invitation_token: "abc".to_owned(),
		};

		assert_eq!(task, expected);
	}

	#[test]
	fn it_builds_intermediate() {
		let task = Task::Invite {
			inviter: "sam@sample.com".to_owned(),
			email: "sally@sample.com".to_owned(),
			invitation_token: "abc".to_owned(),
		};

		let result = generate_intermidiate(&task).unwrap();
	}

	#[test]
	fn it_generates_html() {
		let intermediate = "<intermediate>Hello</intermediate>";
		let result = generate_html(intermediate).unwrap();

		assert_eq!(result, intermediate)
	}
}