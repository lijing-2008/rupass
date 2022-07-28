use anyhow::Result;
use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use sled::{Db, IVec};
use sled::Result as SledResult;

use crate::pb::{PwdInfo, PwdKey};

// 新增密码账户
pub fn add_pwd(db: &Db, url: String, user: String, password: String, notes: String) -> SledResult<Option<IVec>> {
    let key = PwdKey::new(url, user);
    let key_bytes: Bytes = key.into();
    let info = PwdInfo::new(password, notes);
    let info_bytes: Bytes = info.into();
    let info_vec = info_bytes.to_vec();
    db.insert(key_bytes, info_vec)
}

// 删除密码账户
pub fn delete_account(db: &Db, url: &str, user: &str) -> SledResult<Option<IVec>> {
    let key = PwdKey::new(url.to_string(), user.to_string());
    let key_bytes: Bytes = key.into();
    db.remove(key_bytes)
}

// 将数据库中的账户信息全部取出来放到DashMap中
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

// website/app名智能提示，用于搜索
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

// 获取数据库中已知的website/app名
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

// 账户信息智能提示，用于删除
pub fn account_suggester(input: &str) -> Vec<String> {
    let input = input.to_lowercase();
    let mut res = Vec::new();
    // let keys = get_existing_key().expect("get existing info error.");
    let keys = get_existing_account().unwrap();
    for item in keys {
        if item.contains(input.as_str()) {
            res.push(item);
        } else { continue; }
    }
    res
}

// 获取数据库中已知的账户，格式为 website/app名<用户名>
pub fn get_existing_account() -> Result<Vec<String>> {
    let mut source_path = dirs::home_dir().expect("home_dir wrong!");
    source_path.push(".rupass");
    let db = sled::open(source_path)?;
    let dp = get_all_to_map(&db)?;
    let mut res = Vec::new();
    for (key, _) in dp {
        res.push(key.url + "<" + key.user.as_str() + ">")
    }
    res.sort();
    Ok(res)
}

// 判断输入的key是不是完整account信息：website/app名<用户名>,如果是就提取出来
pub fn get_entire_account(key: &str, map: DashMap<PwdKey, PwdInfo>) -> Option<PwdKey> {
    let start = key.find("<");
    let end = key.find(">");

    match start {
        None => { return None; }
        Some(i) => {
            match end {
                None => { return None; }
                Some(j) => {
                    if i < j && key.len() - 1 == j {
                        let url = &key[..i];
                        let user = &key[i + 1..j];
                        for (pwd_key, _) in map {
                            if pwd_key.url == url && pwd_key.user == user {
                                return Some(PwdKey::new(url.to_string(), user.to_string()));
                            } else { continue; }
                        }
                    }
                }
            }
        }
    }
    None
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
