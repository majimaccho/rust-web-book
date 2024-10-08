use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::{id::UserId, model::user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthrorizedUser,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordWithUserId,
        UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};

/// ユーザーを追加する（Admin only）
pub async fn register_user(
    user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    req.validate(&())?;

    let registered_user = registry.user_repository().create(req.into()).await?;
    Ok(Json(registered_user.into()))
}

/// ユーザーの一覧を取得する
pub async fn list_users(
    _user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    let items = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();
    Ok(Json(UsersResponse { items }))
}

/// ユーザーを削除する（Admin onry）
pub async fn delete_user(
    user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<UserId>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::OK)
}

/// ユーザーのロールを変更する（Admin only）
pub async fn change_role(
    user: AuthrorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithUserId::new(user_id, req).into())
        .await?;

    Ok(StatusCode::OK)
}

pub async fn get_current_user(user: AuthrorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

/// ユーザーが自分のパスワードを変更する
pub async fn change_password(
    user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .user_repository()
        .update_password(UpdateUserPasswordWithUserId::new(user.id(), req).into())
        .await?;

    Ok(StatusCode::OK)
}
