use diesel::pg::PgConnection;
use failure::Error;
use models::sign_in::SignIn;
use models::user::User;
use services;

pub fn call(conn: &PgConnection, sign_in: SignIn) -> Result<User, Error> {
	let user = User::find_by_email(conn, &sign_in.email).map_err(|e| format_err!("{}", e))?;

	let invalid = format_err!("Invalid email or password");

	let valid = services::passwords::verify::call(&sign_in.password, &user.password_hash)
		.map_err(|_| format_err!("Invalid email or password"))?;

	if valid {
		Ok(user)
	} else {
		Err(format_err!("Invalid email or password"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use models;
	use services::passwords;
	use utils::tests;

	#[test]
	fn it_can_sign_in() {
		tests::with_db(|conn| {
			let password = "password".to_string();

			let password_hash = passwords::encrypt::call(&password).unwrap();

			let client = models::client::factories::client_attrs().save(conn);

			let user = models::user::factories::user_attrs(&client)
				.password_hash(&password_hash)
				.save(conn);

			let sign_in = SignIn {
				email: user.email.clone(),
				password: password,
			};

			let result = call(&conn, sign_in);

			assert!(result.is_ok());

			let returned_user = result.unwrap();

			assert_eq!(returned_user.email, user.email);
		})
	}

	#[test]
	fn it_cant_sign_in_with_wrong_password() {
		tests::with_db(|conn| {
			let password = "password".to_string();

			let password_hash = passwords::encrypt::call(&password).unwrap();

			let client = models::client::factories::client_attrs().save(conn);

			let user = models::user::factories::user_attrs(&client)
				.password_hash(&password_hash)
				.save(conn);

			let sign_in = SignIn {
				email: user.email.clone(),
				password: "other".to_owned(),
			};

			let result = call(&conn, sign_in);

			assert!(result.is_err());
		})
	}
}