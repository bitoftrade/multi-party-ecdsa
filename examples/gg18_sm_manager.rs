use std::collections::HashMap;
use std::sync::RwLock;
use std::{ env };

use rocket::serde::json::Json;
use rocket::{post,get, routes, State};
use uuid::Uuid;

mod common;
use common::{Entry, Index, Key, Params, PartySignup};

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/get", format = "json", data = "<request>")]
fn get(
    db_mtx: &State<RwLock<HashMap<Key, String>>>,
    request: Json<Index>,
) -> Json<Result<Entry, ()>> {
    let index: Index = request.0;
    let hm = db_mtx.read().unwrap();
    match hm.get(&index.key) {
        Some(v) => {
            let entry = Entry {
                key: index.key,
                value: v.clone(),
            };
            Json(Ok(entry))
        }
        None => Json(Err(())),
    }
}

#[post("/set", format = "json", data = "<request>")]
fn set(db_mtx: &State<RwLock<HashMap<Key, String>>>, request: Json<Entry>) -> Json<Result<(), ()>> {
    let entry: Entry = request.0;
    let mut hm = db_mtx.write().unwrap();
    hm.insert(entry.key.clone(), entry.value);
    Json(Ok(()))
}

#[post("/signupkeygen", format = "json")]
fn signup_keygen(db_mtx: &State<RwLock<HashMap<Key, String>>>) -> Json<Result<PartySignup, ()>> {
    println!("signupkeygen");
    let key = "signup-keygen".to_string();
    let params_key = "params-keygen".to_string();

    let party_signup = {
        let hm = db_mtx.read().unwrap();
        let value = hm.get(&key).unwrap();
        let params = hm.get(&params_key).unwrap();
        let params_value: Params = serde_json::from_str(params).unwrap();
        let parties = params_value.parties.parse::<u16>().unwrap();
        let client_signup: PartySignup = serde_json::from_str(value).unwrap();
        if client_signup.number < parties {
            PartySignup {
                number: client_signup.number + 1,
                uuid: client_signup.uuid,
            }
        } else {
            PartySignup {
                number: 1,
                uuid: Uuid::new_v4().to_string(),
            }
        }
    };

    let mut hm = db_mtx.write().unwrap();
    hm.insert(key, serde_json::to_string(&party_signup).unwrap());
    Json(Ok(party_signup))
}

#[post("/signupsign", format = "json")]
fn signup_sign(db_mtx: &State<RwLock<HashMap<Key, String>>>) -> Json<Result<PartySignup, ()>> {
    //read parameters:
    let key = "signup-sign".to_string();
    let params_key = "params-keygen".to_string();

    let party_signup = {
        let hm = db_mtx.read().unwrap();
        let value = hm.get(&key).unwrap();
        let params = hm.get(&params_key).unwrap();
        let params_value: Params = serde_json::from_str(params).unwrap();
        let threshold = params_value.threshold.parse::<u16>().unwrap();
        let client_signup: PartySignup = serde_json::from_str(value).unwrap();
        if client_signup.number < threshold + 1 {
            PartySignup {
                number: client_signup.number + 1,
                uuid: client_signup.uuid,
            }
        } else {
            PartySignup {
                number: 1,
                uuid: Uuid::new_v4().to_string(),
            }
        }
    };

    let mut hm = db_mtx.write().unwrap();
    hm.insert(key, serde_json::to_string(&party_signup).unwrap());
    Json(Ok(party_signup))
}

#[tokio::main]
async fn main() {
    if env::args().nth(3).is_some() {
        panic!("too many arguments")
    }
    if env::args().nth(2).is_none() {
        panic!("too few arguments")
    }
    // let mut my_config = surf::Config::development();
    // my_config.set_port(18001);
    let db: HashMap<Key, String> = HashMap::new();
    let db: HashMap<Key, String> = HashMap::new();
    let db_mtx = RwLock::new(db);
    //rocket::custom(my_config).mount("/", routes![get, set]).manage(db_mtx).launch();

    /////////////////////////////////////////////////////////////////
    //////////////////////////init signups://////////////////////////
    /////////////////////////////////////////////////////////////////

    let keygen_key = "signup-keygen".to_string();
    let sign_key = "signup-sign".to_string();
    let params_key = "params-keygen".to_string();

    let uuid_keygen = Uuid::new_v4().to_string();
    let uuid_sign = Uuid::new_v4().to_string();

    println!("{} - threshold", env::args().nth(1).unwrap());
    println!("{} - parties", env::args().nth(2).unwrap());

    let params = Params {
        parties: env::args().nth(2).unwrap(),
        threshold: env::args().nth(1).unwrap(),
    };
 
    let party1 = 0;
    let party_signup_keygen = PartySignup {
        number: party1,
        uuid: uuid_keygen,
    };
    let party_signup_sign = PartySignup {
        number: party1,
        uuid: uuid_sign,
    };
    {
        let mut hm = db_mtx.write().unwrap();
        hm.insert(
            keygen_key,
            serde_json::to_string(&party_signup_keygen).unwrap(),
        );
        hm.insert(sign_key, serde_json::to_string(&party_signup_sign).unwrap());
        hm.insert(params_key, serde_json::to_string(&params).unwrap());
    }
    /////////////////////////////////////////////////////////////////
    rocket::build()
        .mount("/", routes![ping, get, set, signup_keygen, signup_sign])
        .manage(db_mtx)
        .launch()
        .await
        .unwrap();
}
