use crate::structs::user::User;

/// A simple function to get a user by id from a database, then combine it.
/// This is just a Don't Repeat Yourself method. 
pub fn combine_user_by_id(id: String) -> User {
    // Get the user by his id from a database, I'll use those standard user data for now:
    User {
        id: String::from(id),
        username: String::from("mr_elmarjani"),
        name: String::from("Hamza El Marjani"),
        email: String::from("hamzaelmarjani@gmail.com"),
        created_at: String::from("2025-11-09T18:14:10.693Z"),
    }
}