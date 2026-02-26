use crate::{
    admin::service::{get_user_by_email, get_user_by_id},
    app_state::SharedState,
    auth::models::{Claims, LoginRequest, LoginResponse},
    errors::AppError,
    utils::verify_password,
};

pub async fn verify_login(
    state: SharedState,
    login_request: LoginRequest,
) -> Result<LoginResponse, AppError> {
    let user_data = get_user_by_email(state.clone(), login_request.email)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid email or password".to_string()))?;
    if verify_password(&login_request.password, &user_data.password_hash)? {
        let user_claims = Claims::new(user_data);
        let token = user_claims.generate_token();
        Ok(LoginResponse { token })
    } else {
        Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ))
    }
}

pub async fn get_user_from_claims(
    state: SharedState,
    claims: Claims,
) -> Result<crate::admin::models::User, AppError> {
    let user = get_user_by_id(state, claims.sub).await?;
    Ok(user)
}
