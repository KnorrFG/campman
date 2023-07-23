pub mod actions;
pub mod components;
pub mod schema;

pub type Schema = schema::v1::Schema;

#[derive(Clone, PartialEq)]
pub enum Mode {
    Dashboard,
    Active(ActiveMode),
    Error { err: String, parent: Box<Mode> },
    EditingSubject(String),
    Fatal(String),
}

#[derive(Clone, PartialEq)]
pub enum ActiveMode {
    Events,
    NewSubject,
    Subject(String),
}

pub struct State {
    pub mode: Mode,
    pub db_path: Option<String>,
    pub user_dirs: directories::UserDirs,
}

#[macro_export]
/// used to be able to use results in component-handlers. State is a UseSharedState<State>
macro_rules! attempt {
    ($state:ident $body:block) => {
        let res = || -> Result<(), anyhow::Error> { $body }();
        if let Err(e) = res {
            let s = &mut *$state.write();
            s.mode = Mode::Error {
                err: e.to_string(),
                parent: Box::new(s.mode.clone()),
            };
        }
    };
}
