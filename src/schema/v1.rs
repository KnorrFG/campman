use super::{Error, Result, WithId};

use std::fmt::Display;
use std::path::Path;

pub struct Subject {
    pub name: String,
    pub description: String,
}

pub struct Place {
    pub name: String,
    pub description: String,
    pub parent_place: Option<i64>,
}

pub struct Event {
    pub record_date: u64,
    pub refered_date: String,
    pub description: String,
}

pub struct Group {
    pub name: String,
    pub description: String,
    pub parent_group: Option<i64>,
}

pub struct Tag {
    pub name: String,
}

pub struct Schema {
    conn: sqlite::Connection,
}

impl Schema {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(Self {
            conn: sqlite::open(path)?,
        })
    }

    pub fn create_new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path: &Path = path.as_ref();
        if path.exists() {
            Err(Error::FileExists(path.display().to_string()))
        } else {
            let mut me = Self::open(path)?;
            me.create()?;
            Ok(me)
        }
    }

    fn create_subjects_table(&mut self) -> Result<()> {
        let query = "
        create table subjects(
            id integer primary key,
            name text,
            description text  
        ); ";
        self.conn.execute(query)?;
        Ok(())
    }

    fn create_places_table(&mut self) -> Result<()> {
        let query = "
        create table places(
            id integer primary key,
            name text,
            description text,
            parent_place integer,
            foreign key(parent_place) references places(id)
        ); ";
        self.conn.execute(query)?;
        Ok(())
    }

    fn create_events_table(&mut self) -> Result<()> {
        let query = "
        create table events(
            id integer primary key,
            description text,
            happened_at text,
            edited_at integer
        ); ";
        self.conn.execute(query)?;
        Ok(())
    }

    fn create_groups_table(&mut self) -> Result<()> {
        let query = "
        create table groups(
            id integer primary key,
            name text,
            description text,
            parent_group integer,
            foreign key(parent_group) references groups(id)
        ); ";
        self.conn.execute(query)?;
        Ok(())
    }

    fn create_tags_table(&mut self) -> Result<()> {
        let query = "
        create table tags(
            id integer primary key,
            name text
        ); ";
        self.conn.execute(query)?;
        Ok(())
    }

    fn create_mapping(&mut self, from: &str, to: &str) -> Result<()> {
        let query = format!(
            "
        create table mapping_{from}_{to}(
            kfrom integer,
            kto integer,
            foreign key(kfrom) references {from}(id),
            foreign key(kto) references {to}(id)
        ); "
        );
        self.conn.execute(query)?;
        Ok(())
    }

    fn create(&mut self) -> Result<()> {
        self.create_subjects_table()?;
        self.create_events_table()?;
        self.create_places_table()?;
        self.create_groups_table()?;
        self.create_tags_table()?;
        for to in ["subjects", "groups", "places"] {
            self.create_mapping("subjects", to)?;
        }
        for to in ["subjects", "groups", "places", "tags"] {
            self.create_mapping("events", to)?;
        }
        self.create_mapping("places", "groups")?;
        Ok(())
    }

    pub fn insert_subject(&mut self, x: &Subject) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("insert into subjects (name, description) values (?, ?)")?;
        stmt.bind(&[(1, x.name.as_str()), (2, x.description.as_str())][..])?;
        assert!(stmt.next()? == sqlite::State::Done);
        Ok(())
    }

    pub fn insert_place(&mut self, x: &Place) -> Result<()> {
        self.conn
            .prepare("insert into places (name, description, parent_place) values (?, ?, ?)")?
            .bind(
                &[
                    (1, x.name.as_str()),
                    (2, x.description.as_str()),
                    (3, &opt_to_str(&x.parent_place)),
                ][..],
            )?;
        Ok(())
    }

    pub fn insert_event(&mut self, x: &Event) -> Result<()> {
        self.conn
            .prepare(
                "insert into events (record_date, refered_date, description) values (?, ?, ?)",
            )?
            .bind(
                &[
                    (1, x.record_date.to_string().as_str()),
                    (2, x.refered_date.as_str()),
                    (3, x.description.as_str()),
                ][..],
            )?;
        Ok(())
    }

    pub fn insert_group(&mut self, x: &Group) -> Result<()> {
        self.conn
            .prepare("insert into groups (name, description, parent_group) values (?, ?, ?)")?
            .bind(
                &[
                    (1, x.name.as_str()),
                    (2, x.description.as_str()),
                    (3, &opt_to_str(&x.parent_group)),
                ][..],
            )?;
        Ok(())
    }

    pub fn insert_tag(&mut self, x: &Tag) -> Result<()> {
        self.conn
            .prepare("insert into tags (name) values (?)")?
            .bind(&[(1, x.name.as_str())][..])?;
        Ok(())
    }

    pub fn update_subject(&mut self, x: &WithId<Subject>) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("update subjects set name = ?, description = ? where id == ?;")?;
        stmt.bind(
            &[
                (1, x.name.as_str()),
                (2, x.description.as_str()),
                (3, x.id.to_string().as_str()),
            ][..],
        )?;
        assert!(stmt.next()? == sqlite::State::Done);
        Ok(())
    }

    pub fn update_place(&mut self, x: &WithId<Place>) -> Result<()> {
        self.conn
            .prepare("update places set name = ?, description =? , parent_place = ? where id == ?")?
            .bind(
                &[
                    (1, x.name.as_str()),
                    (2, x.description.as_str()),
                    (3, &opt_to_str(&x.parent_place)),
                    (4, x.id.to_string().as_str()),
                ][..],
            )?;
        Ok(())
    }

    pub fn update_event(&mut self, x: &WithId<Event>) -> Result<()> {
        self.conn.prepare(
            "update events set record_date = ?, refered_date = ?, description = ? where id == ?",
        )?
        .bind(
            &[
                (1, x.record_date.to_string().as_str()),
                (2, x.refered_date.as_str()),
                (3, x.description.as_str()),
                (4, x.id.to_string().as_str()),
            ][..],
        )?;
        Ok(())
    }

    pub fn update_group(&mut self, x: &WithId<Group>) -> Result<()> {
        self.conn
            .prepare("update groups set name = ?, description = ?, parent_group = ? where id == ?")?
            .bind(
                &[
                    (1, x.name.as_str()),
                    (2, x.description.as_str()),
                    (3, &opt_to_str(&x.parent_group)),
                    (4, x.id.to_string().as_str()),
                ][..],
            )?;
        Ok(())
    }

    pub fn query_place_description(&mut self, query: &str) -> Result<Vec<i64>> {
        self.query_x(query, "places", "description")
    }

    fn query_x(&mut self, query: &str, table_name: &str, col_name: &str) -> Result<Vec<i64>> {
        let res = self
            .conn
            .prepare(&format!(
                "select id from ? where ? like '%?%' --case-insensitive"
            ))?
            .into_iter()
            .bind(&[(1, table_name), (2, col_name), (3, query)][..])?
            .map(|r| Ok(r?.read::<i64, _>("id")))
            .collect::<Result<Vec<_>>>()?;
        Ok(res)
    }

    pub fn get_sub_by_name(&mut self, name: &str) -> Result<Option<WithId<Subject>>> {
        let query = "select * from subjects where name == ?;";
        let sub = self
            .conn
            .prepare(query)?
            .into_iter()
            .bind(&[(1, name)][..])?
            .map(|r| {
                let r = r?;
                Ok(WithId {
                    t: Subject {
                        name: r.read::<&str, _>("name").to_string(),
                        description: r.read::<&str, _>("description").to_string(),
                    },
                    id: r.read::<i64, _>("id"),
                })
            })
            .collect::<Result<Vec<WithId<Subject>>>>()?
            .into_iter()
            .next();
        Ok(sub)
    }
}

fn opt_to_str<T: Display>(x: &Option<T>) -> String {
    x.as_ref().map(|x| x.to_string()).unwrap_or("NULL".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn check_queries() -> Result<()> {
        let mut conn = sqlite::Connection::open(":memory:")?;
        create_subjects_table(&mut conn).context("Creating subjects table:")?;
        create_events_table(&mut conn).context("Creating events table:")?;
        create_places_table(&mut conn).context("Creating places table:")?;
        create_groups_table(&mut conn).context("Creating groups table:")?;
        create_tags_table(&mut conn).context("Creating tags table:")?;

        for to in ["subjects", "groups", "places"] {
            create_mapping(&mut conn, "subjects", to)
                .context(format!("create mapping from subjects to {to}"))?;
        }
        for to in ["subjects", "groups", "places", "tags"] {
            create_mapping(&mut conn, "events", to)
                .context(format!("create mapping from events to {to}"))?;
        }
        create_mapping(&mut conn, "places", "groups")
            .context(format!("create mapping from places to groups"))?;
        Ok(())
    }
}
