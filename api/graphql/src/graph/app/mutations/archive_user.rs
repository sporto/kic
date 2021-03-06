use crate::{actions, graph::AppContext, utils::mutations::MutationError};
use juniper::{Executor, FieldError, FieldResult};

// #[derive(Deserialize, Clone, GraphQLInputObject)]
// pub struct ArchiveUserInput {
// 	pub userId: i32,
// }

#[derive(Clone)]
pub struct ArchiveUserResponse {
	success: bool,
	errors:  Vec<MutationError>,
}

graphql_object!(ArchiveUserResponse: AppContext |&self| {
	field success() -> bool {
		self.success
	}

	field errors() -> &Vec<MutationError> {
		&self.errors
	}
});

pub fn call(executor: &Executor<AppContext>, user_id: i32) -> FieldResult<ArchiveUserResponse> {
	let ctx = executor.context();
	let conn = &ctx.conn;
	let current_user = &ctx.user;

	// Authorise
	let can = actions::users::authorise::can_archive(&conn, &current_user, user_id)?;

	if can == false {
		return Err(FieldError::from("Unauthorised"));
	}

	actions::users::archive::call(&conn, user_id)?;

	let response = ArchiveUserResponse {
		success: true,
		errors:  vec![],
	};

	Ok(response)
}
