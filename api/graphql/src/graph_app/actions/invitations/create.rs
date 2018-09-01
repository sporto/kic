use diesel::pg::PgConnection;
use failure::Error;
use models::invitation::{Invitation, InvitationAttrs};
use models::user::{User, Role};
use uuid::Uuid;
use validator::Validate;

pub fn call(conn: &PgConnection, user: &User, email: &str) -> Result<Invitation, Error> {
	let token = Uuid::new_v4();

	let invitation_attrs = InvitationAttrs {
		user_id: user.id,
		email: email.to_string(),
		role: Role::Investor,
		token: token.to_string(),
		used_at: None,
	};

	invitation_attrs
		.validate()
		.map_err(|e| format_err!("{}", e.to_string()))?;

	let invitation =
		Invitation::create(conn, invitation_attrs).map_err(|e| format_err!("{}", e.to_string()))?;

	// TODO send to the mail lambda
	// invitations::send_email::call(&user, &invitation)?;

	Ok(invitation)
}

#[cfg(test)]
mod tests {
	use super::*;
	use models;
	use utils::tests;

	#[test]
	fn it_creates_an_invitation() {
		tests::with_db(|conn| {
			let client = models::client::factories::client_attrs().save(conn);
			let user = models::user::factories::user_attrs(&client).save(conn);
			let email = "samantha@sample.com".to_owned();

			let result = call(&conn, &user, &email);

			assert!(result.is_ok());

			let invitation = result.unwrap();

			assert_eq!(invitation.email, email);
		})
	}
}
