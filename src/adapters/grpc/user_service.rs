use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::application::use_cases::user::UserUseCase;

pub mod user_grpc {
    tonic::include_proto!("user");
}

pub struct UserService {
    user_use_case: Arc<UserUseCase>,
}

impl UserService {
    pub fn new(user_use_case: Arc<UserUseCase>) -> Self {
        Self { user_use_case }
    }
}

#[tonic::async_trait]
impl user_grpc::user_service_server::UserService for UserService {
    async fn get_user_profile(
        &self,
        request: Request<user_grpc::GetUserProfileRequest>,
    ) -> Result<Response<user_grpc::UserProfileResponse>, Status> {
        let req = request.into_inner();

        let user = self
            .user_use_case
            .get_user_by_id(&req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        match user {
            Some(u) => {
                let response = user_grpc::UserProfileResponse {
                    id: u.id().to_string(),
                    email: u.email().to_string(),
                    name: u.name().to_string(),
                };
                Ok(Response::new(response))
            }
            None => Err(Status::not_found("User not found")),
        }
    }
}
