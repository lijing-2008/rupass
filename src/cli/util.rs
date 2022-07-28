use anyhow::Result;
use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use sled::{Db, IVec};
use sled::Result as SledResult;

use crate::pb::{PwdInfo, PwdKey};

pub fn add_pwd(db: &Db, url: String, user: String, password: String, notes: String) -> SledResult<Option<IVec>> {
    let key = PwdKey::new(url, user);
    let key_bytes: Bytes = key.into();
    let info = PwdInfo::new(password, notes);
    let info_bytes: Bytes = info.into();
    let info_vec = info_bytes.to_vec();
    db.insert(key_bytes, info_vec)
}

pub fn get_all_to_map(db: &Db) -> Result<DashMap<PwdKey, PwdInfo>> {
    let pwd_map = DashMap::new();
    for item in db.iter() {
        match item {
            Ok((key, value)) => {
                let key_b = BytesMut::from(key.to_vec().as_slice());
                let key = PwdKey::try_from(key_b)?;
                let value_b = BytesMut::from(value.to_vec().as_slice());
                let value = PwdInfo::try_from(value_b)?;
                if key.url != "primary_key" && key.user != "root" {
                    pwd_map.insert(key, value);
                }
            }
            Err(e) => {
                println!("sorry! An error occur: {}", e);
            }
        }
    }
    Ok(pwd_map)
}

/// This could be faster by using smarter ways to check for matches, when dealing with larger datasets.
pub fn key_suggester(input: &str) -> Vec<String> {
    let input = input.to_lowercase();
    let mut res = Vec::new();
    // let keys = get_existing_key().expect("get existing info error.");
    let keys = get_existing_key().unwrap();
    for item in keys {
        if item.contains(input.as_str()) {
            res.push(item);
        } else { continue; }
    }
    res
}

pub fn get_existing_key() -> Result<Vec<String>> {
    let mut source_path = dirs::home_dir().expect("home_dir wrong!");
    source_path.push(".rupass");
    let db = sled::open(source_path)?;
    let dp = get_all_to_map(&db)?;
    let mut res = Vec::new();
    for (key, _) in dp {
        res.push(key.url)
    }
    res.sort();
    res.dedup();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use sled::Db;

    use crate::cli::{get_all_to_map, get_existing_key, key_suggester};

    #[test]
    fn test() {
        let src = "/Users/lijing/.rupass";
        let db = sled::open(src).unwrap();
        let ds = get_all_to_map(&db).unwrap();

        // for (key, value) in ds {
        //     println!("{}, {}", key, value);
        //     println!("=====================")
        // }
    }

    #[test]
    fn test_key_suggester() {
        let a = get_existing_key();
        println!("{:?}", a);
        let b = key_suggester("q");
        println!("{:?}", b);
    }
}
