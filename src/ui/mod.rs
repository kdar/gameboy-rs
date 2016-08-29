#[cfg(feature = "uipiston")]
mod piston_ui;
#[cfg(feature = "uiglium")]
mod glium_ui;

#[cfg(feature = "uipiston")]
pub use self::piston_ui::PistonUi as Ui;
#[cfg(feature = "uiglium")]
pub use self::glium_ui::GliumUi as Ui;
