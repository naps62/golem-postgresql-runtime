use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::config::PGConf;
use postgres::{Client, NoTls};

macro_rules! sql {
    ($client:ident, $query:expr) => {{
        $client.execute($query.as_str(), &[]).unwrap()
    }};
}

pub fn create(conf: &PGConf) -> (String, String) {
    let mut rng = ChaCha20Rng::from_entropy();

    let username = gen_random_readable_string(&mut rng);
    let password = gen_random_readable_string(&mut rng);

    println!("{}", conf.db_url().as_str());
    let mut client = Client::connect(conf.db_url().as_str(), NoTls).unwrap();

    sql!(client, format!("DROP ROLE IF EXISTS {}", username));
    sql!(client, format!("DROP DATABASE IF EXISTS {}", username));
    sql!(
        client,
        format!("CREATE USER {} PASSWORD '{}'", username, password)
    );
    sql!(client, format!("CREATE DATABASE {}", username));
    sql!(
        client,
        format!("GRANT CONNECT ON DATABASE {} TO {}", username, username)
    );

    (username, password)
}

pub fn destroy(conf: &PGConf, username: &String) {
    let mut client = Client::connect(conf.db_url().as_str(), NoTls).unwrap();

    sql!(client, format!("DROP DATABASE {}", username));
    sql!(client, format!("DROP USER {}", username));
}

fn gen_random_readable_string<T: Rng + Sized>(rng: &mut T) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
        .to_lowercase()
}
