use crate::{
	actions::{emails, passwords},
	models::{
		client::{Client, ClientAttrs},
		schema::users,
		sign_up::SignUp,
		user::{Role, User, UserAttrs},
	},
};
use diesel::{dsl::exists, pg::PgConnection, prelude::*, select};
use failure::Error;
use uuid::Uuid;
use validator::Validate;
use crate::utils::validations;

pub fn call(conn: &PgConnection, sign_up: SignUp) -> Result<User, Error> {
	let password_hash =
		passwords::encrypt::call(&sign_up.password).map_err(|e| format_err!("{}", e))?;

	// Validate the user attrs
	let temp_user_attrs = UserAttrs {
		client_id:                1, // Just to validate
		role:                     Role::Admin,
		username:                 sign_up.username.clone(),
		name:                     sign_up.name.clone(),
		email:                    Some(sign_up.email.clone()),
		password_hash:            password_hash.clone(),
		email_confirmation_token: None,
		email_confirmed_at:       None,
		archived_at:              None,
		password_reset_token:     None,
	};

	temp_user_attrs
		.validate()
		.map_err(|e| format_err!("{}", validations::to_human_error(e)))?;

	// Check if we have a user with this email already
	let filter = users::table.filter(users::email.eq(sign_up.email.clone()));

	let existing = select(exists(filter)).get_result(conn)?;

	if existing {
		return Err(format_err!("Already taken"));
	}

	let client_attrs = ClientAttrs {
		name: sign_up.name.clone(),
	};

	// Create client and then user
	let user = Client::create(conn, client_attrs)
		.and_then(|client| {
			let confirmation_token = Uuid::new_v4().to_string();

			let user_attrs = UserAttrs {
				client_id:                client.id,
				role:                     Role::Admin,
				username:                 sign_up.username,
				name:                     sign_up.name,
				email:                    Some(sign_up.email),
				password_hash:            password_hash,
				email_confirmation_token: Some(confirmation_token),
				email_confirmed_at:       None,
				archived_at:              None,
				password_reset_token:     None,
			};

			User::create(conn, user_attrs)
		})
		.map_err(|e| format_err!("{}", e))?;

	emails::email_confirmation::call(&user)?;

	Ok(user)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests;

	#[test]
	fn it_creates_a_client_and_user() {
		tests::with_db(|conn| {
			let attrs = SignUp {
				name:     "Sam".to_string(),
				username: "username".to_string(),
				email:    "sam@sample.com".to_string(),
				password: "password".to_string(),
			};

			let result = call(conn, attrs);

			assert!(result.is_ok());

			let user = result.unwrap();
			// println!("{:?}", user.password_hash);

			assert_eq!(user.name, "Sam".to_owned());
			assert_eq!(user.email, Some("sam@sample.com".to_owned()));
			assert_eq!(user.role, Role::Admin);
		})
	}

	#[test]
	fn it_fails_with_invalid_username() {
		tests::with_db(|conn| {
			let attrs = SignUp {
				name:     "Sam".to_string(),
				email:    "sam@sample.com".to_string(),
				username: "Hello world".to_string(),
				password: "password".to_string(),
			};

			let result = call(conn, attrs);

			assert!(result.is_err());
		})
	}

	#[test]
	fn it_fails_with_invalid_email() {
		tests::with_db(|conn| {
			let attrs = SignUp {
				name:     "Sam".to_string(),
				email:    "flamingo".to_string(),
				username: "username".to_string(),
				password: "password".to_string(),
			};

			let result = call(conn, attrs);

			assert!(result.is_err());
		})
	}
}
