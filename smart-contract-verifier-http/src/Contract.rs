use mongodb::{Client, Database, Collection, options::{ClientOptions, ResolverConfig}};
use chrono::{TimeZone, Utc};

pub struct Contract {
    file_name: String
}