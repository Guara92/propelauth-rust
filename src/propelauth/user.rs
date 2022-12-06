use crate::apis::configuration::Configuration;
use crate::apis::user_service_api::{
    CreateMagicLinkParams, CreateUserParams, DeleteUserParams, DisableUser2faParams,
    DisableUserParams, EnableUserParams, FetchUserByEmailParams, FetchUserByIdParams,
    FetchUserByUsernameParams, FetchUsersByEmailsError, FetchUsersByEmailsParams,
    FetchUsersByIdsError, FetchUsersByIdsParams, FetchUsersByQueryError, FetchUsersByQueryParams,
    FetchUsersByUsernamesError, FetchUsersByUsernamesParams, MigrateUserParams,
    UpdateUserEmailParams, UpdateUserMetadataParams, UpdateUserPasswordParams,
};
use crate::models::{
    CreateMagicLinkRequest, CreateUserRequest, CreatedUserResponse, MagicLink, MigrateUserRequest,
    UserMetadata, UserPagedResponse,
};
use crate::propelauth::errors::{
    BatchFetchError, CreateMagicLinkError, CreateUserError, ErrorsWithNotFound, FetchByQueryError,
    MigrateUserError, UpdatePasswordError, UpdateUserEmailError, UpdateUserMetadataError,
};
use crate::propelauth::helpers::{is_valid_id, map_autogenerated_error};
use std::collections::HashMap;

pub struct UserService<'a> {
    pub(crate) config: &'a Configuration,
}

impl UserService<'_> {
    pub async fn fetch_user_by_email(
        &self,
        params: FetchUserByEmailParams,
    ) -> Result<UserMetadata, ErrorsWithNotFound> {
        crate::apis::user_service_api::fetch_user_by_email(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })
    }

    pub async fn fetch_user_by_id(
        &self,
        params: FetchUserByIdParams,
    ) -> Result<UserMetadata, ErrorsWithNotFound> {
        if !is_valid_id(&params.user_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        crate::apis::user_service_api::fetch_user_by_id(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })
    }

    pub async fn fetch_user_by_username(
        &self,
        params: FetchUserByUsernameParams,
    ) -> Result<UserMetadata, ErrorsWithNotFound> {
        crate::apis::user_service_api::fetch_user_by_username(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })
    }

    pub async fn fetch_users_by_ids(
        &self,
        params: FetchUsersByIdsParams,
    ) -> Result<HashMap<String, UserMetadata>, BatchFetchError> {
        let users = crate::apis::user_service_api::fetch_users_by_ids(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    BatchFetchError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (_, Some(FetchUsersByIdsError::Status400(bad_request))) => {
                            match bad_request.query {
                                None => {
                                    BatchFetchError::BadRequest("Request is invalid".to_string())
                                }
                                Some(bad_query_reasons) => BatchFetchError::BadRequest(
                                    bad_query_reasons.join(", ").to_string(),
                                ),
                            }
                        }
                        (401, _) => BatchFetchError::InvalidApiKey,
                        _ => BatchFetchError::UnexpectedException,
                    },
                )
            })?;
        Ok(users
            .into_iter()
            .map(|user| (user.user_id.clone(), user))
            .collect())
    }

    pub async fn fetch_users_by_emails(
        &self,
        params: FetchUsersByEmailsParams,
    ) -> Result<HashMap<String, UserMetadata>, BatchFetchError> {
        let users = crate::apis::user_service_api::fetch_users_by_emails(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    BatchFetchError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (_, Some(FetchUsersByEmailsError::Status400(bad_request))) => {
                            match bad_request.query {
                                None => {
                                    BatchFetchError::BadRequest("Request is invalid".to_string())
                                }
                                Some(bad_query_reasons) => BatchFetchError::BadRequest(
                                    bad_query_reasons.join(", ").to_string(),
                                ),
                            }
                        }
                        (401, _) => BatchFetchError::InvalidApiKey,
                        _ => BatchFetchError::UnexpectedException,
                    },
                )
            })?;
        Ok(users
            .into_iter()
            .map(|user| (user.email.clone(), user))
            .collect())
    }

    pub async fn fetch_users_by_usernames(
        &self,
        params: FetchUsersByUsernamesParams,
    ) -> Result<HashMap<String, UserMetadata>, BatchFetchError> {
        let users = crate::apis::user_service_api::fetch_users_by_usernames(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    BatchFetchError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (_, Some(FetchUsersByUsernamesError::Status400(bad_request))) => {
                            match bad_request.query {
                                None => {
                                    BatchFetchError::BadRequest("Request is invalid".to_string())
                                }
                                Some(bad_query_reasons) => BatchFetchError::BadRequest(
                                    bad_query_reasons.join(", ").to_string(),
                                ),
                            }
                        }
                        (401, _) => BatchFetchError::InvalidApiKey,
                        _ => BatchFetchError::UnexpectedException,
                    },
                )
            })?;
        Ok(users
            .into_iter()
            .filter(|user| user.username.is_some())
            .map(|user| (user.username.clone().unwrap(), user))
            .collect())
    }

    pub async fn fetch_users_by_query(
        &self,
        params: FetchUsersByQueryParams,
    ) -> Result<UserPagedResponse, FetchByQueryError> {
        crate::apis::user_service_api::fetch_users_by_query(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    FetchByQueryError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (_, Some(FetchUsersByQueryError::Status400(bad_request))) => {
                            FetchByQueryError::BadRequest(bad_request)
                        }
                        (401, _) => FetchByQueryError::InvalidApiKey,
                        _ => FetchByQueryError::UnexpectedException,
                    },
                )
            })
    }

    pub async fn create_user(
        &self,
        create_user_request: CreateUserRequest,
    ) -> Result<CreatedUserResponse, CreateUserError> {
        let params = CreateUserParams {
            create_user_request,
        };
        crate::apis::user_service_api::create_user(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    CreateUserError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::user_service_api::CreateUserError::Status400(
                                bad_request,
                            )),
                        ) => CreateUserError::BadRequest(bad_request),
                        (401, _) => CreateUserError::InvalidApiKey,
                        _ => CreateUserError::UnexpectedException,
                    },
                )
            })
    }

    pub async fn delete_user(&self, user_id: String) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&user_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        let params = DeleteUserParams { user_id };
        crate::apis::user_service_api::delete_user(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn disable_user(&self, user_id: String) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&user_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        let params = DisableUserParams { user_id };
        crate::apis::user_service_api::disable_user(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn enable_user(&self, user_id: String) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&user_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        let params = EnableUserParams { user_id };
        crate::apis::user_service_api::enable_user(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn update_user_metadata(
        &self,
        params: UpdateUserMetadataParams,
    ) -> Result<(), UpdateUserMetadataError> {
        if !is_valid_id(&params.user_id) {
            return Err(UpdateUserMetadataError::NotFound);
        }

        crate::apis::user_service_api::update_user_metadata(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    UpdateUserMetadataError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(
                                crate::apis::user_service_api::UpdateUserMetadataError::Status400(
                                    bad_request,
                                ),
                            ),
                        ) => UpdateUserMetadataError::BadRequest(bad_request),
                        (401, _) => UpdateUserMetadataError::InvalidApiKey,
                        (404, _) => UpdateUserMetadataError::NotFound,
                        _ => UpdateUserMetadataError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn update_user_email(
        &self,
        params: UpdateUserEmailParams,
    ) -> Result<(), UpdateUserEmailError> {
        if !is_valid_id(&params.user_id) {
            return Err(UpdateUserEmailError::NotFound);
        }

        crate::apis::user_service_api::update_user_email(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    UpdateUserEmailError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::user_service_api::UpdateUserEmailError::Status400(
                                bad_request,
                            )),
                        ) => UpdateUserEmailError::BadRequest(bad_request),
                        (401, _) => UpdateUserEmailError::InvalidApiKey,
                        (404, _) => UpdateUserEmailError::NotFound,
                        (429, _) => UpdateUserEmailError::EmailSentTooRecently,
                        _ => UpdateUserEmailError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn update_user_password(
        &self,
        params: UpdateUserPasswordParams,
    ) -> Result<(), UpdatePasswordError> {
        if !is_valid_id(&params.user_id) {
            return Err(UpdatePasswordError::NotFound);
        }

        crate::apis::user_service_api::update_user_password(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    UpdatePasswordError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(
                                crate::apis::user_service_api::UpdateUserPasswordError::Status400(
                                    bad_request,
                                ),
                            ),
                        ) => UpdatePasswordError::BadRequest(bad_request),
                        (401, _) => UpdatePasswordError::InvalidApiKey,
                        (404, _) => UpdatePasswordError::NotFound,
                        _ => UpdatePasswordError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn disable_user_2fa(&self, user_id: String) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&user_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        let params = DisableUser2faParams { user_id };
        crate::apis::user_service_api::disable_user2fa(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ErrorsWithNotFound::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        401 => ErrorsWithNotFound::InvalidApiKey,
                        404 => ErrorsWithNotFound::NotFound,
                        _ => ErrorsWithNotFound::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn migrate_user(
        &self,
        migrate_user_request: MigrateUserRequest,
    ) -> Result<CreatedUserResponse, MigrateUserError> {
        let params = MigrateUserParams {
            migrate_user_request,
        };
        crate::apis::user_service_api::migrate_user(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    MigrateUserError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::user_service_api::MigrateUserError::Status400(
                                bad_request,
                            )),
                        ) => MigrateUserError::BadRequest(bad_request),
                        (401, _) => MigrateUserError::InvalidApiKey,
                        _ => MigrateUserError::UnexpectedException,
                    },
                )
            })
    }

    pub async fn create_magic_link(
        &self,
        create_magic_link_request: CreateMagicLinkRequest,
    ) -> Result<MagicLink, CreateMagicLinkError> {
        let params = CreateMagicLinkParams {
            create_magic_link_request,
        };
        crate::apis::user_service_api::create_magic_link(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    CreateMagicLinkError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::user_service_api::CreateMagicLinkError::Status400(
                                bad_request,
                            )),
                        ) => CreateMagicLinkError::BadRequest(bad_request),
                        (401, _) => CreateMagicLinkError::InvalidApiKey,
                        _ => CreateMagicLinkError::UnexpectedException,
                    },
                )
            })
    }
}