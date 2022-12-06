use crate::apis::configuration::Configuration;
use crate::apis::org_service_api::{
    AddUserToOrgParams, AllowOrgToEnableSamlParams, ChangeUserRoleInOrgParams, CreateOrgParams,
    DisallowSamlParams, FetchOrgParams, FetchOrgsByQueryParams, FetchUsersInOrgParams,
    RemoveUserFromOrgParams, UpdateOrgParams,
};
use crate::models::{
    AddUserToOrgRequest, ChangeUserRoleInOrgRequest, CreateOrgRequest, CreateOrgResponse,
    FetchOrgsResponse, RemoveUserFromOrgRequest, UpdateOrgRequest, UserPagedResponse,
};
use crate::propelauth::errors::{
    CreateOrgError, ErrorsWithNotFound, FetchOrgsByQueryError, FetchUsersInOrgError,
    OrgMissingOrRoleError, UpdateOrgError,
};
use crate::propelauth::helpers::{is_valid_id, map_autogenerated_error};

pub struct OrgService<'a> {
    pub(crate) config: &'a Configuration,
}

impl OrgService<'_> {
    /// Fetch an organization by it's ID
    pub async fn fetch_org(&self, org_id: String) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&org_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        let params = FetchOrgParams { org_id };
        crate::apis::org_service_api::fetch_org(&self.config, params)
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

    /// Fetch and page over organizations
    pub async fn fetch_orgs_by_query(
        &self,
        params: FetchOrgsByQueryParams,
    ) -> Result<FetchOrgsResponse, FetchOrgsByQueryError> {
        let response = crate::apis::org_service_api::fetch_orgs_by_query(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    FetchOrgsByQueryError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::org_service_api::FetchOrgsByQueryError::Status400(
                                bad_request,
                            )),
                        ) => FetchOrgsByQueryError::BadRequest(bad_request),
                        (401, _) => FetchOrgsByQueryError::InvalidApiKey,
                        _ => FetchOrgsByQueryError::UnexpectedException,
                    },
                )
            })?;
        Ok(response)
    }

    pub async fn fetch_users_in_org(
        &self,
        params: FetchUsersInOrgParams,
    ) -> Result<UserPagedResponse, FetchUsersInOrgError> {
        if !is_valid_id(&params.org_id) {
            return Ok(UserPagedResponse::default());
        }

        let response = crate::apis::org_service_api::fetch_users_in_org(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    FetchUsersInOrgError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::org_service_api::FetchUsersInOrgError::Status400(
                                bad_request,
                            )),
                        ) => FetchUsersInOrgError::BadRequest(bad_request),
                        (401, _) => FetchUsersInOrgError::InvalidApiKey,
                        _ => FetchUsersInOrgError::UnexpectedException,
                    },
                )
            })?;
        Ok(response)
    }

    pub async fn add_user_to_org(
        &self,
        add_user_to_org_request: AddUserToOrgRequest,
    ) -> Result<(), OrgMissingOrRoleError> {
        let params = AddUserToOrgParams {
            add_user_to_org_request,
        };
        crate::apis::org_service_api::add_user_to_org(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    OrgMissingOrRoleError::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        400 => OrgMissingOrRoleError::UnknownRoleError,
                        401 => OrgMissingOrRoleError::InvalidApiKey,
                        404 => OrgMissingOrRoleError::NotFound,
                        _ => OrgMissingOrRoleError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn change_user_role_in_org(
        &self,
        change_user_role_in_org_request: ChangeUserRoleInOrgRequest,
    ) -> Result<(), OrgMissingOrRoleError> {
        let params = ChangeUserRoleInOrgParams {
            change_user_role_in_org_request,
        };
        crate::apis::org_service_api::change_user_role_in_org(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    OrgMissingOrRoleError::UnexpectedException,
                    |status_code, _| match status_code.as_u16() {
                        400 => OrgMissingOrRoleError::UnknownRoleError,
                        401 => OrgMissingOrRoleError::InvalidApiKey,
                        404 => OrgMissingOrRoleError::NotFound,
                        _ => OrgMissingOrRoleError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn remove_user_from_org(
        &self,
        remove_user_from_org_request: RemoveUserFromOrgRequest,
    ) -> Result<(), ErrorsWithNotFound> {
        let params = RemoveUserFromOrgParams {
            remove_user_from_org_request,
        };
        crate::apis::org_service_api::remove_user_from_org(&self.config, params)
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

    pub async fn create_org(
        &self,
        create_org_request: CreateOrgRequest,
    ) -> Result<CreateOrgResponse, CreateOrgError> {
        let params = CreateOrgParams { create_org_request };
        let org = crate::apis::org_service_api::create_org(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    CreateOrgError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::org_service_api::CreateOrgError::Status400(
                                bad_request,
                            )),
                        ) => CreateOrgError::BadRequest(bad_request),
                        (401, _) => CreateOrgError::InvalidApiKey,
                        _ => CreateOrgError::UnexpectedException,
                    },
                )
            })?;
        Ok(org)
    }

    pub async fn update_org(
        &self,
        org_id: String,
        update_org_request: UpdateOrgRequest,
    ) -> Result<(), UpdateOrgError> {
        if !is_valid_id(&org_id) {
            return Err(UpdateOrgError::NotFound);
        }

        let params = UpdateOrgParams {
            org_id,
            update_org_request,
        };
        crate::apis::org_service_api::update_org(&self.config, params)
            .await
            .map_err(|err| {
                map_autogenerated_error(
                    err,
                    UpdateOrgError::UnexpectedException,
                    |status_code, err_entity| match (status_code.as_u16(), err_entity) {
                        (
                            _,
                            Some(crate::apis::org_service_api::UpdateOrgError::Status400(
                                bad_request,
                            )),
                        ) => UpdateOrgError::BadRequest(bad_request),
                        (401, _) => UpdateOrgError::InvalidApiKey,
                        (404, _) => UpdateOrgError::NotFound,
                        _ => UpdateOrgError::UnexpectedException,
                    },
                )
            })?;
        Ok(())
    }

    pub async fn allow_org_to_enable_saml(
        &self,
        params: AllowOrgToEnableSamlParams,
    ) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&params.org_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        crate::apis::org_service_api::allow_org_to_enable_saml(&self.config, params)
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

    pub async fn disallow_saml(
        &self,
        params: DisallowSamlParams,
    ) -> Result<(), ErrorsWithNotFound> {
        if !is_valid_id(&params.org_id) {
            return Err(ErrorsWithNotFound::NotFound);
        }

        crate::apis::org_service_api::disallow_saml(&self.config, params)
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
}