use anyhow::{anyhow, Context};

use crate::{
    models::{
        services::UserService,
        traits::*
    },
    views,
    cli
};

pub struct UserController {
    users: UserService
}

impl UserController {
    pub fn new(users: UserService) -> Self {
        Self { users }
    }

    pub fn get_by_id(&self, id: i32) {
        let user = self.users.get_by_id(id)
            .context("user service failed to fetch user")
            .and_then(|maybe_user| maybe_user.ok_or_else(
                || anyhow!("user with the given id was not found")
            ));
        
        match &user {
            Ok(user) => views::display_user_by_id(user),
            Err(err) => views::display_err(err)
        }
    }

    pub fn create(&self, input_user: &cli::CreateUser) {
        let user = self.users.create(input_user)
            .context("user service failed to create user");

        match &user {
            Ok(user) => views::display_new_user(user),
            Err(err) => views::display_err(err)
        }
    }

    pub fn update(&self, user_update: &cli::UserUpdate) {
        let user = self.users.update(user_update)
            .context("user service failed to update user");

        match &user {
            Ok(user) => views::display_updated_user(user),
            Err(err) => views::display_err(err)
        }
    }

    pub fn delete(&self, id: i32) {
        let was_deleted = self.users.delete_by_id(id);

        match was_deleted {
            Ok(was_deleted) => views::display_user_was_deleted(id, was_deleted),
            Err(err) => views::display_err(&err)
        }
    }
}
