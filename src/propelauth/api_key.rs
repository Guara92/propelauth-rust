use crate::apis::api_key_service_api::{ApiKeyError, ApiKeyQueryParams, CreateApiKeyParams, UpdateApiKeyParams, ValidateApiKeyParams};
use crate::apis::configuration::Configuration;
use crate::models::{CreateApiKeyResponse, FetchApiKeyResponse, FetchApiKeysPagedResponse, ValidateApiKeyResponse};
use crate::models::validate_api_key_response::{ValidateOrgApiKeyResponse, ValidatePersonalApiKeyResponse};
use crate::propelauth::helpers::map_autogenerated_error;

pub struct ApiKeyService<'a> {
    pub(crate) config: &'a Configuration,
}

impl ApiKeyService<'_> {
    pub async fn fetch_current_api_keys(&self, params: ApiKeyQueryParams) -> Result<FetchApiKeysPagedResponse, ApiKeyError> {
        crate::apis::api_key_service_api::fetch_current_api_keys(self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::api_key_service_api::ApiKeyError::BadRequest(
                                     bad_request,
                                 )),
                        ) => ApiKeyError::BadRequest(bad_request),
                        (401, _) => ApiKeyError::InvalidIntegrationAPIKey,
                        (404, _) => ApiKeyError::NotFound,
                        _ => ApiKeyError::UnexpectedExceptionWithSDK,
                    },
                )
            })
    }

    pub async fn fetch_archived_api_keys(&self, params: ApiKeyQueryParams) -> Result<FetchApiKeysPagedResponse, ApiKeyError> {
        crate::apis::api_key_service_api::fetch_archived_api_keys(self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::api_key_service_api::ApiKeyError::BadRequest(
                                     bad_request,
                                 )),
                        ) => ApiKeyError::BadRequest(bad_request),
                        (401, _) => ApiKeyError::InvalidIntegrationAPIKey,
                        (404, _) => ApiKeyError::NotFound,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })
    }

    pub async fn fetch_api_key(&self, api_key_id: String) -> Result<FetchApiKeyResponse, ApiKeyError> {
        crate::apis::api_key_service_api::fetch_api_key(self.config, api_key_id)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, _| match status_code.as_u16() {
                        401 => ApiKeyError::InvalidIntegrationAPIKey,
                        404 => ApiKeyError::InvalidAPIKey,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })
    }

    pub async fn create_api_key(&self, params: CreateApiKeyParams) -> Result<CreateApiKeyResponse, ApiKeyError> {
        crate::apis::api_key_service_api::create_api_key(self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, _| match status_code.as_u16() {
                        401 => ApiKeyError::InvalidIntegrationAPIKey,
                        404 => ApiKeyError::NotFound,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })
    }

    pub async fn update_api_key(&self, api_key_id: String, params: UpdateApiKeyParams) -> Result<(), ApiKeyError> {
        crate::apis::api_key_service_api::update_api_key(self.config, api_key_id, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, _| match status_code.as_u16() {
                        401 => ApiKeyError::InvalidIntegrationAPIKey,
                        404 => ApiKeyError::InvalidAPIKey,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })?;

        Ok(())
    }

    pub async fn delete_api_key(&self, api_key_id: String) -> Result<(), ApiKeyError> {
        crate::apis::api_key_service_api::delete_api_key(self.config, api_key_id)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, _| match status_code.as_u16() {
                        401 => ApiKeyError::InvalidIntegrationAPIKey,
                        404 => ApiKeyError::InvalidAPIKey,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })?;

        Ok(())
    }

    pub async fn validate_api_key(&self, params: ValidateApiKeyParams) -> Result<ValidateApiKeyResponse, ApiKeyError> {
        crate::apis::api_key_service_api::validate_api_key(self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    ApiKeyError::UnexpectedExceptionWithSDK,
                    |status_code, _| match status_code.as_u16() {
                        401 => ApiKeyError::InvalidIntegrationAPIKey,
                        404 => ApiKeyError::NotFound,
                        _ => ApiKeyError::UnknownError,
                    },
                )
            })
    }

    pub async fn validate_personal_api_key(&self, params: ValidateApiKeyParams) -> Result<ValidatePersonalApiKeyResponse, ApiKeyError> {
        let resp = self.validate_api_key(params).await?;
        if resp.user.is_none() || resp.org.is_some() {
            return Err(ApiKeyError::InvalidPersonalAPIKey);
        }

        Ok(ValidatePersonalApiKeyResponse {
            metadata: resp.metadata,
            user: resp.user.unwrap(),
        })
    }

    pub async fn validate_org_api_key(&self, params: ValidateApiKeyParams) -> Result<ValidateOrgApiKeyResponse, ApiKeyError> {
        let resp = self.validate_api_key(params).await?;
        if resp.org.is_none() {
            return Err(ApiKeyError::InvalidOrgAPIKey);
        }

        Ok(ValidateOrgApiKeyResponse {
            metadata: resp.metadata,
            user: resp.user,
            org: resp.org.unwrap(),
            user_in_org: resp.user_in_org,
        })
    }
}
