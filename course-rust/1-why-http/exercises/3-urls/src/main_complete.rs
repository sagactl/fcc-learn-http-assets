use serde::Deserialize;

fn main() {
    let users_url = "https://jsonplaceholder.typicode.com/users";

    let users = get_data(users_url);

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

fn get_data(url: &str) -> Vec<User> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .send()
        .expect("Failed to send request");

    response.json().expect("Failed to parse JSON")
}

fn log_users(users: &[User]) {
    for user in users {
        println!("{}", user.name);
    }
}
