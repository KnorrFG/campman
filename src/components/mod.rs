pub mod color {
    pub const PRIMARY: &str = "#26B3D1";
    pub const SECONDARY: &str = "#1D7C92";
    pub const WHITE: &str = "#FFFFFF";
    pub const GREY: &str = "#F5F5F5";
}

mod dashboard;
pub use dashboard::Dashboard;

mod sidebar;
pub use sidebar::Sidebar;

mod error;
pub use error::Error;

mod events;
pub use events::Events;

mod new_subject;
pub use new_subject::NewSubject;

mod button;
pub use button::*;

mod editing_subject;
pub use editing_subject::*;

mod subject;
pub use subject::*;
