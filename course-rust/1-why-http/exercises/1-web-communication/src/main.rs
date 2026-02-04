use serde::Deserialize;

fn main() {
    let users = get_user_data();

    log_users();
}

// don't touch below this line

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
}

fn get_user_data() -> Vec<User> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .expect("Failed to send request");

    response.json().expect("Failed to parse JSON")
}

fn log_users(users: &[User]) {
    for user in users {
        println!("{}", user.name);
    }
}
