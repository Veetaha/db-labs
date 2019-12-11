use crate::models::entities::User;

pub fn display_user_by_id(user: &User) {
    println!("Retrieved user by id: {:#?}", user);
}

pub fn display_new_user(user: &User) {
    println!("New user was created successfully: {:#?}", user);
}

pub fn display_updated_user(user: &User) {
    println!("User was updated successfully: {:#?}", user);
}

pub fn display_user_was_deleted(id: i32, was_deleted: bool) {
    println!(
        "User under id {id} was {indeed} deleted", 
        id = id, 
        indeed = if was_deleted { "successfully" } else { "not" }
    );
}
