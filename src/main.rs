pub mod json_structs;
pub mod models;
pub mod schema;
pub mod service;
use models::user::NewUser;
use service::database_connection::Database;
use service::user_service::UserService;
use serde_json::json;
use tide::Request;
use std::sync::{Arc, Mutex};
use json_structs::group_json::*;

fn main() -> Result<(), std::io::Error> {
    let version: &'static str = env!("CARGO_PKG_VERSION");

    let f = async {
        let database = Database {};
        let state = Arc::new(Mutex::new(database));
        let mut app = tide::with_state(state);

        app.at("/version")
            .get(move |_| async move { Ok(format!("version: {version}")) });
        app.at("/create-user")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in create user");
                let NewUser { name } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!("Name is {}", name);
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.create_user(&name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error creating user"),
                    )),
                }
            });

        app.at("/create-group")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in create group");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!("Username is {}, Groupname is {}", caller_name, group_name);
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.create_group(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error creating group"),
                    )),
                }
            });

        app.at("/join-group")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in join group");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!("Username is {}, Groupname is {}", caller_name, group_name);
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.join_group(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error joining the group"),
                    )),
                }
            });

        app.at("/set-admin")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in join group");
                let SetAdminJson {
                    caller_name,
                    new_admin_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, New_admin_name is {}, Groupname is {}",
                    caller_name, new_admin_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.set_admin(&caller_name, &new_admin_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error joining the group"),
                    )),
                }
            });

        app.at("/retire")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in retiring");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, Groupname is {}",
                    caller_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.retire(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error retiring"),
                    )),
                }
            });

        app.at("/leave")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in leaving");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, Groupname is {}",
                    caller_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.leave(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error leaving"),
                    )),
                }
            });

        app.at("/delete-group")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in leaving");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, Groupname is {}",
                    caller_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.delete_group(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error deleting the group"),
                    )),
                }
            });

        app.at("/start")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in leaving");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, Groupname is {}",
                    caller_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.start_secret_santa(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error starting the game"),
                    )),
                }
            });

        app.at("/get-ward")
            .put(|mut request: Request<Arc<Mutex<Database>>>| async move {
                println!("Got in leaving");
                let UsernameGroupnameJson {
                    caller_name,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;
                println!(
                    "Caller_name is {}, Groupname is {}",
                    caller_name, group_name
                );
                let state = request.state();
                let _guard = state.lock().unwrap();
                let mut user_service = UserService::new();

                match user_service.get_ward(&caller_name, &group_name) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(..) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!("Error starting the game"),
                    )),
                }
            });

        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}