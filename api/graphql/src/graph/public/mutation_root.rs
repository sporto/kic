use juniper::FieldResult;

use crate::graph::{PublicContext};
use crate::graph::public::mutations;
use crate::models::sign_in::SignIn;
use crate::models::sign_up::SignUp;

pub struct PublicMutationRoot;

graphql_object!(PublicMutationRoot: PublicContext | &self | {

	field signUp(&executor, sign_up: SignUp) -> FieldResult<mutations::sign_up::SignUpResponse> {
		mutations::sign_up::call(executor, sign_up)
	}

	field signIn(&executor, sign_in: SignIn) -> FieldResult<mutations::sign_in::SignInResponse> {
		mutations::sign_in::call(executor, sign_in)
	}

	field confirm_email(
		&executor,
		input: mutations::confirm_email::ConfirmEmailInput
		) -> FieldResult<mutations::confirm_email::ConfirmEmailResponse> {
		
		mutations
			::confirm_email
			::call(executor, input)
	}

	field redeem_invitation(
		&executor,
		input: mutations::redeem_invitation::RedeemInvitationInput
	) -> FieldResult<mutations::redeem_invitation::RedeemInvitationResponse> {

		mutations
			::redeem_invitation
			::call(executor, input)
	}

	field request_password_reset(&executor, input: mutations::request_password_reset::RequestPasswordResetInput) -> FieldResult<mutations::request_password_reset::RequestPasswordResetResponse> {

		mutations
			::request_password_reset
			::call(executor, input)
	}

	field reset_password(&executor, input: mutations::reset_password::ResetPasswordInput) -> FieldResult<mutations::reset_password::ResetPasswordResponse> {

		mutations
			::reset_password
			::call(executor, input)
	}

});
