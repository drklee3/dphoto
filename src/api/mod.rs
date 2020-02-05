pub mod album;
pub mod image;
pub mod index;
pub mod login;
pub mod logout;
pub mod register;

pub use self::album::get_album;
pub use self::image::get_image;
pub use self::index::get_index;
pub use self::login::post_login;
pub use self::logout::post_logout;
pub use self::register::post_register;
