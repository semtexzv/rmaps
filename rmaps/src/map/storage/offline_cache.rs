use ::prelude::*;
use ::common::rusqlite::{Connection, Statement};

use super::{
    resource::{Request, Resource, TileRequestData,}
};

pub struct OfflineCache {
    db: Connection
}

impl OfflineCache {
    pub fn new(path: &str) -> Result<Self> {
        let mut db = Connection::open(path)?;
        db.execute("PRAGMA foreign_keys = ON", &[])?;

        let old_version = db.query_row("PRAGMA user_version", &[], |row| {
            row.get::<_, i32>(0)
        })?;

        db.execute("PRAGMA foreign_keys = ON", &[])?;
        db.execute("PRAGMA auto_vacuum = INCREMENTAL", &[])?;
//        db.execute("PRAGMA journal_mode = DELETE", &[])?;
        db.execute("PRAGMA synchronous = FULL", &[])?;
        db.execute(include_str!("./schema/schema.sql"), &[])?;
        db.execute("PRAGMA user_version = 1", &[])?;


        Ok(OfflineCache {
            db
        })
    }


    pub fn get(&self, req: &Request) -> Result<Option<Resource>> {
        let url = req.url();
        self.db.execute(r#"UPDATE resources SET accessed = ?1 WHERE url = ?2 and expires > strftime('%s', 'now')"#, &[&0, &url])?;
        let mut get_stmt = self.db.prepare("SELECT data FROM resources where url = ?1")?;
        let data = get_stmt.query_map(&[&url], |row| row.get::<_, Vec<u8>>(0))?.map(|x| x.unwrap()).next();

        return Ok(if let Some(data) = data {
            Some(Resource {
                data: data,
                cache_until : u64::max_value(),
                req: req.clone(),
            })
        } else {
            None
        });
    }
    pub fn put(&self, res: &Resource) -> Result<()> {
        let url = res.req.url();
        self.db.execute("INSERT OR REPLACE INTO resources(url,kind,data,expires,accessed) VALUES(?1,?2,?3,?4,?5)", &[
            &url,
            &1,
            &res.data,
            &(res.cache_until as i64),
            &0
        ])?;

        Ok(())
    }
}