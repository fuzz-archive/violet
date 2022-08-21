use serde::{Serialize, Deserialize};
use serde_yaml::{from_reader, Error};
use std::fs::File;


#[derive(Serialize, Deserialize)]
pub struct Configuration {
  pub port: u16,
  pub host: String
}

pub fn read<'a>(path: &'a str) -> Result<Configuration, Error> {
  let reader = File::open(path).unwrap();

  let conf: Result<Configuration, _> = from_reader(reader);

  return conf;
}