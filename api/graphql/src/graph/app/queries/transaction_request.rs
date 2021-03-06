use crate::{
	graph::AppContext,
	models::{
		account::Account, cents::Cents, schema as db, transaction_kind::TransactionKind,
		transaction_request::TransactionRequest,
		transaction_request_state::TransactionRequestState,
	},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

graphql_object!(TransactionRequest: AppContext |&self| {
	field id() -> i32 {
		self.id
	}

	field createdAt() -> NaiveDateTime {
		self.created_at
	}

	field account_id() -> i32 {
		self.account_id
	}

	field account(&executor) -> FieldResult<Account> {
		let ctx = &executor.context();
		let conn = &ctx.conn;

		db::accounts::table.find(self.account_id)
			.first::<Account>(&*conn)
			.map_err(|e| FieldError::from(e))
	}

	field kind() -> TransactionKind {
		self.kind
	}

	field amount_in_cents()-> f64 {
		let Cents(cents) = self.amount;
		cents as f64
	}

	field state()-> TransactionRequestState {
		self.state
	}
});
