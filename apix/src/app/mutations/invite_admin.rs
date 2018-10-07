use actions::invitations;
use actions::invitations::authorise;
use graphql::AppContext;
use juniper::{Executor, FieldError, FieldResult};
use models::role::Role;
use utils::mutations::failure_to_mutation_errors;
use utils::mutations::MutationError;

#[derive(Deserialize, Clone, GraphQLInputObject)]
pub struct InvitationInput {
	pub email: String,
}

#[derive(GraphQLObject, Clone)]
pub struct InvitationResponse {
	success: bool,
	errors: Vec<MutationError>,
}

pub fn call(
	executor: &Executor<AppContext>,
	input: InvitationInput,
) -> FieldResult<InvitationResponse> {
	let context = executor.context();
	let conn = &context.conn;
	let current_user = &context.user;

	// Authorise
	let can = authorise::call(&conn, &current_user)?;

	if can == false {
		return Err(FieldError::from("Unauthorised"));
	}

	let invitation_result =
		invitations::create::call(&conn, &current_user, &input.email, Role::Admin);

	match invitation_result {
		Ok(invitation) => invitation,
		Err(e) => {
			return Ok(InvitationResponse {
				success: false,
				errors: failure_to_mutation_errors(e),
			});
		}
	};

	Ok(InvitationResponse {
		success: true,
		errors: vec![],
	})
}
