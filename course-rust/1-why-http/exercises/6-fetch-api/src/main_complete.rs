use serde::Deserialize;

fn main() {
    let url = get_url();

    let response = reqwest::blocking::Client::new()
        .get(&url)
        .send()
        .expect("Failed to send request");

    let users: Vec<User> = response.json().expect("Failed to parse JSON");

    log_users(&users);
}

// don't touch below this line

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
}

fn get_url() -> String {
    "https://jsonplaceholder.typicode.com/users".to_string()
}

fn log_users(users: &[User]) {
    for user in users {
        println!("{}", user.name);
    }
}
