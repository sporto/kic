use db;
// use models::clients::{Client, ClientAttrs};
// use models::users::{User, UserAttrs};
use utils::config;

pub fn run(conn: db::Conn) -> bool {
    let app_env = config::app_env();

    match app_env {
        config::AppEnv::Test => {
            println!("Seeding");
            let _ = seed(conn);
            true
        }
        _ => {
            println!("Cannot seed in {:?}", app_env);
            false
        }
    }
}

fn seed(_conn: db::Conn) -> Result<String, String> {
    // let _ = User::delete_all(&conn);
    // let _ = Client::delete_all(&conn);

    // let client_attrs = ClientAttrs {};

    // Client::create(&conn, client_attrs)
    // 	.map_err(|e| format!("{:?}", e))
    // 	.map(|client| {
    // 		let user_attrs = UserAttrs {
    // 			client_id: client.id,
    // 			name: "Sam Sample".to_string(),
    // 			email: "sam@sample.com".to_string(),
    // 			timezone: "Australia/Melbourne".to_string(),
    // 		};

    // 		let _ = User::add(&conn, user_attrs);

    // 		"Ok".to_string()
    // 	})

    Err("Not implemented".to_string())
}
