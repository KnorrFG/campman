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
    Search,
}

pub struct State {
    pub mode: Mode,
    pub db_path: Option<String>,
    pub user_dirs: directories::UserDirs,
}

impl State {
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_active_mode(&mut self, mode: ActiveMode) {
        self.mode = Mode::Active(mode);
    }
}

#[macro_export]
/// used to be able to use results in component-handlers. State is a UseSharedState<State>
macro_rules! attempt {
    ($state:ident $body:block) => {
        let res = || -> Result<(), anyhow::Error> { $body }();
        if let Err(e) = res {
            let s = &mut *$state.write();
            s.mode = crate::Mode::Error {
                err: e.to_string(),
                parent: Box::new(s.mode.clone()),
            };
        }
    };
}

#[macro_export]
macro_rules! comp_try {
    ($state:ident, $res:expr) => {
        match $res {
            Ok(x) => x,
            Err(e) => {
                let s = &mut *$state.write();
                s.mode = crate::Mode::Error {
                    err: e.to_string(),
                    parent: Box::new(s.mode.clone()),
                };
                return None;
            }
        }
    };
}
