use clap::{Parser, Subcommand};

// pub use pb::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Subcommand)]
pub enum Commands {
    /// init your password store
    Init,

    /// add password
    Add,

    /// search password
    Search,

    /// delete password
    Delete,

    /// update password
    Update,
}


#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};
    use prost::Message;
    use crate::pb::PwdKey;

    #[test]
    fn test() {
        let key = PwdKey::new("http://www.baidu.com".to_string(),
                              "username".to_string());

        let a: Bytes = key.into();
        println!("{:?}", a);
        let c = BytesMut::from(a.to_vec().as_slice());
        let b = PwdKey::try_from(c).unwrap();
        println!("{}", b.user);
        println!("{}", b.url);
    }
}
