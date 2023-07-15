use crate::errors::Error;
use std::collections::BTreeMap;
use std::{fs::File, io::Read};

pub fn get_env_parser_from_file(file_path: &str) -> Result<BTreeMap<String, String>, Error> {
    let mut file = File::open(file_path).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error while reading file");
    let env = dotenv_parser::parse_dotenv(&content).map_err(|error| {
        log::error!(
            "Couldn't parse file `{}`.\n    --> Cause: {}",
            file_path,
            error
        );
        Error::ParseEnvFailedNoSuchFile(file_path.to_string(), error.to_string())
    })?;

    Ok(env)
}

pub fn find_key_from_parser<K: std::cmp::Ord + std::fmt::Display, V: Clone>(
    key: &K,
    env: &BTreeMap<K, V>,
) -> Result<V, Error> {
    let result = env
        .get(key)
        .ok_or(Error::ParseEnvFailedNoSuchKey(key.to_string()))
        .cloned()?;
    Ok(result)
}
