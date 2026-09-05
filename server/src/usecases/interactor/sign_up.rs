use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::user::User,
        errors::AppError
    },
    usecases::{
        dto::sign_up::SignUpInput,
        port::{
            sign_up::SignUpUseCase,
            unit_of_work::{
                UnitOfWork,
                UnitOfWorkExt
            }
        },
        security::password::PasswordManager
    }
};

pub(crate) struct SignUpInteractor {
    uow: Arc<dyn UnitOfWork>,
    password_manager: Arc<dyn PasswordManager>,
}

impl SignUpInteractor {
    pub fn new(
        uow: Arc<dyn UnitOfWork>,
        password_manager: Arc<dyn PasswordManager>,
    ) -> Self {
        Self { uow, password_manager }
    }
}

#[async_trait]
impl SignUpUseCase for SignUpInteractor {
    async fn execute(&self, input: SignUpInput) -> Result<(), AppError> {
        let hashed_password =self.password_manager.hash(&input.password)
            .map_err(|e| AppError::Infrastructure(e.into()))?;

        let email = input.email.clone();

        let user = User::new(
            Uuid::new_v4(),
            input.name,
            input.email,
            hashed_password,
        )?;

        self.uow.execute(|repos| {
            Box::pin(async move {
                if repos.user_repo.find_by_email(&email).await?.is_some() {
                    return Err(anyhow::anyhow!("User already exists"));
                }
                repos.user_repo.create(&user).await?;
                Ok(())
            })
        })
        .await
        .map_err(|e| {
            if e.to_string() == "User already exists" {
                AppError::Conflict("Email already taken".into())
            } else {
                AppError::Infrastructure(e.to_string())
            }
        })?;

        /*
        
        // 例: 作成した User をトランザクションからそのまま受け取る場合
        let created_user: User = self.uow.execute(|repos| {
            Box::pin(async move {
                let user = User::new(Uuid::new_v4(), name, email, hashed_password)?;
                repos.user_repo.create(&user).await?;
                
                // 戻り値として User を返す
                Ok(user)
            })
        }).await.map_err(...)?;
        
         */

        Ok(())
    }
}
