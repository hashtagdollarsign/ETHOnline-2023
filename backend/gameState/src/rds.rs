use std::env;

pub fn idk_yet() {
    let user_name = env::var("RDS_USERNAME")
        .expect("User Name not found in environment");
    let password = env::var("RDS_PASSWORD")
        .expect("Password not found in environment");
    let rds_proxy_host = env::var("RDS_PROXY_HOST")
        .expect("Proxy Host not found in environment");
    let db_name = env::var("RDS_DB_NAME")
        .expect("Database Name not found in environment");


}