use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct PGCli {
    #[allow(unused)]
    path: Option<std::path::PathBuf>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct PGConf {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub admin_db: String,
    pub logfile: std::path::PathBuf,
}

impl PGConf {
    pub fn db_url(&self) -> String {
        self.db_url_for_user(&self.username, &self.password, &self.admin_db)
    }

    pub fn db_url_for_user(&self, username: &String, password: &String, db: &String) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, self.host, self.port, username
        )
    }
}
