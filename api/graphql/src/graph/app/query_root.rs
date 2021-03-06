use crate::{
	actions,
	graph::AppContext,
	models::{
		account::Account, client::Client, role::Role, schema as db,
		transaction_request::TransactionRequest,
		transaction_request_state::TransactionRequestState, user::User,
	},
};
use chrono_tz::{America, Australia};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct AppQueryRoot;

graphql_object!(AppQueryRoot: AppContext |&self| {

	field apiVersion() -> &str {
		"1.0"
	}

	// Only an admin can request this
	field admin(&executor) -> FieldResult<Admin> {
		let ctx = &executor.context();
		let current_user = &ctx.user;

		if current_user.role != Role::Admin {
			return Err(FieldError::from("Unauthorized"))
		};

	 	Ok(Admin {
			investors: vec![],
			account: None,
	 	})
	}

	field investor(&executor) -> FieldResult<Investor> {
		let ctx = &executor.context();
		let current_user = &ctx.user;

		if current_user.role != Role::Investor {
			return Err(FieldError::from("Unauthorized"))
		};

		Ok(Investor {
			accounts: vec![],
			account: None,
		})
	}

	field timezones(&executor) -> FieldResult<Vec<String>> {
		let timezones = vec![
			format!("{:?}", Australia::Adelaide),
			format!("{:?}", Australia::Melbourne),
			format!("{:?}", Australia::Broken_Hill),
			format!("{:?}", America::Lima),
		];

		Ok(timezones)
	}
});

struct Admin {
	investors: Vec<User>,
	account:   Option<Account>,
}

graphql_object!(Admin: AppContext |&self| {

	field investors(&executor) -> FieldResult<Vec<User>> {
		let ctx = &executor.context();
		let client_id = ctx.user.client_id;
		let conn = &ctx.conn;

		let is_investor = db::users::role.eq(Role::Investor);

		let filter = db::users
			::client_id.eq(client_id)
			.and(is_investor);

		db::users::table
			.filter(filter)
			.load::<User>(&*conn)
			.map_err(|e| FieldError::from(e))
	}

	field account(&executor, id: i32) -> FieldResult<Account> {
		let ctx = &executor.context();
		let conn = &ctx.conn;
		let current_user = &ctx.user;

		// Authorise
		let can = actions::accounts::authorise::can_access(&conn, id, current_user)?;

		if can == false {
			return Err(FieldError::from("Unauthorized"))
		};

		Account::find(&conn, id)
			.map_err(|e| FieldError::from(e))
	}

	field pending_requests(&executor) -> FieldResult<Vec<TransactionRequest>> {
		let ctx = &executor.context();
		let conn = &ctx.conn;
		let current_user = &ctx.user;

		let client = db::clients::table.find(current_user.client_id).first::<Client>(&*conn)?;

		let users = User::belonging_to(&client).load::<User>(&*conn)?;

		let accounts = Account::belonging_to(&users).load::<Account>(&*conn)?;

		let is_pending = db::transaction_requests::state.eq(TransactionRequestState::Pending);

		TransactionRequest::belonging_to(&accounts)
			.filter(is_pending)
			.get_results(&*conn)
			.map_err(|e| FieldError::from(e))
	}

});

struct Investor {
	accounts: Vec<Account>,
	account:  Option<Account>,
}

graphql_object!(Investor: AppContext |&self| {

	field accounts(&executor) -> FieldResult<Vec<Account>> {
		let ctx = &executor.context();
		let conn = &ctx.conn;

		let user_id = ctx.user.id;

		let filter = db::accounts
			::user_id.eq(user_id);

		db::accounts::table
			.filter(filter)
			.load::<Account>(&*conn)
			.map_err(|e| FieldError::from(e))
	}

	field account(&executor, id: i32) -> FieldResult<Account> {
		let ctx = &executor.context();
		let conn = &ctx.conn;

		let current_user = &ctx.user;

		// Authorise
		let can = actions::accounts::authorise::can_access(&conn, id, current_user)?;

		if can == false {
			return Err(FieldError::from("Unauthorized"))
		};

		Account::find(&conn, id)
			.map_err(|e| FieldError::from(e))
	}

});
