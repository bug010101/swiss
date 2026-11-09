use anyhow::Result;
use uuid::Uuid;

use crate::cli::{UuidArgs, UuidVersion};

pub fn run(args: &UuidArgs) -> Result<()> {
    for _ in 0..args.count {
        let id = match args.version {
            UuidVersion::V4 => Uuid::new_v4(),
            UuidVersion::V7 => Uuid::now_v7(),
        };
        println!("{id}");
    }
    Ok(())
}
