use crate::{Result, WithId};
use std::fmt::Display;

pub struct Subject {
    pub name: String,
    pub description: String,
}

pub struct Place {
    pub name: String,
    pub description: String,
    pub parent_place: Option<u64>,
}

pub struct Event {
    pub record_date: u64,
    pub refered_date: String,
    pub description: String,
}

pub struct Group {
    pub name: String,
    pub description: String,
    pub parent_group: Option<u64>,
}

pub struct Tag {
    pub name: String,
}

pub fn create_subjects_table(conn: &mut sqlite::Connection) -> Result<()> {
    let query = "
        create table subjects(
            id integer primary key,
            name text,
            description text  
        ); ";
    conn.execute(query)?;
    Ok(())
}

pub fn create_places_table(conn: &mut sqlite::Connection) -> Result<()> {
    let query = "
        create table places(
            id integer primary key,
            name text,
            description text,
            parent_place integer,
            foreign key(parent_place) references places(id)
        ); ";
    conn.execute(query)?;
    Ok(())
}

pub fn create_events_table(conn: &mut sqlite::Connection) -> Result<()> {
    let query = "
        create table events(
            id integer primary key,
            description text,
            happened_at text,
            edited_at integer
        ); ";
    conn.execute(query)?;
    Ok(())
}

pub fn create_groups_table(conn: &mut sqlite::Connection) -> Result<()> {
    let query = "
        create table groups(
            id integer primary key,
            name text,
            description text,
            parent_group integer,
            foreign key(parent_group) references groups(id)
        ); ";
    conn.execute(query)?;
    Ok(())
}

pub fn create_tags_table(conn: &mut sqlite::Connection) -> Result<()> {
    let query = "
        create table tags(
            id integer primary key,
            name text
        ); ";
    conn.execute(query)?;
    Ok(())
}

pub fn create_mapping(conn: &mut sqlite::Connection, from: &str, to: &str) -> Result<()> {
    let query = format!(
        "
        create table mapping_{from}_{to}(
            kfrom integer,
            kto integer,
            foreign key(kfrom) references {from}(id),
            foreign key(kto) references {to}(id)
        ); "
    );
    conn.execute(query)?;
    Ok(())
}

pub fn create(conn: &mut sqlite::Connection) -> Result<()> {
    create_subjects_table(conn)?;
    create_events_table(conn)?;
    create_places_table(conn)?;
    create_groups_table(conn)?;
    create_tags_table(conn)?;
    for to in ["subjects", "groups", "places"] {
        create_mapping(conn, "subjects", to)?;
    }
    for to in ["subjects", "groups", "places", "tags"] {
        create_mapping(conn, "events", to)?;
    }
    create_mapping(conn, "places", "groups")?;
    Ok(())
}

pub fn insert_subject(conn: &mut sqlite::Connection, x: &Subject) -> Result<()> {
    conn.prepare("insert into subjects (name, description) values (?, ?)")?
        .bind((x.name.as_str(), x.description.as_str()))?;
    Ok(())
}

pub fn insert_place(conn: &mut sqlite::Connection, x: &Place) -> Result<()> {
    conn.prepare("insert into places (name, description, parent_place) values (?, ?, ?)")?
        .bind(
            &[
                (1, x.name.as_str()),
                (2, x.description.as_str()),
                (3, &opt_to_str(&x.parent_place)),
            ][..],
        )?;
    Ok(())
}

pub fn insert_event(conn: &mut sqlite::Connection, x: &Event) -> Result<()> {
    conn.prepare("insert into events (record_date, refered_date, description) values (?, ?, ?)")?
        .bind(
            &[
                (1, x.record_date.to_string().as_str()),
                (2, x.refered_date.as_str()),
                (3, x.description.as_str()),
            ][..],
        )?;
    Ok(())
}

pub fn insert_group(conn: &mut sqlite::Connection, x: &Group) -> Result<()> {
    conn.prepare("insert into groups (name, description, parent_group) values (?, ?, ?)")?
        .bind(
            &[
                (1, x.name.as_str()),
                (2, x.description.as_str()),
                (3, &opt_to_str(&x.parent_group)),
            ][..],
        )?;
    Ok(())
}

pub fn insert_tag(conn: &mut sqlite::Connection, x: &Tag) -> Result<()> {
    conn.prepare("insert into tags (name) values (?)")?
        .bind(&[(1, x.name.as_str())][..])?;
    Ok(())
}

pub fn update_subject(conn: &mut sqlite::Connection, x: &WithId<Subject>) -> Result<()> {
    conn.prepare("update subjects set name = ?, description = ? where id == ? limit 1")?
        .bind(
            &[
                (1, x.name.as_str()),
                (2, x.description.as_str()),
                (3, x.id.to_string().as_str()),
            ][..],
        )?;
    Ok(())
}

pub fn update_place(conn: &mut sqlite::Connection, x: &WithId<Place>) -> Result<()> {
    conn.prepare("update places set name = ?, description =? , parent_place = ? where id == ?")?
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

pub fn update_event(conn: &mut sqlite::Connection, x: &WithId<Event>) -> Result<()> {
    conn.prepare(
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

pub fn update_group(conn: &mut sqlite::Connection, x: &WithId<Group>) -> Result<()> {
    conn.prepare("update groups set name = ?, description = ?, parent_group = ? where id == ?")?
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

fn opt_to_str<T: Display>(x: &Option<T>) -> String {
    x.map(|x| x.to_string()).unwrap_or("NULL".into())
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
