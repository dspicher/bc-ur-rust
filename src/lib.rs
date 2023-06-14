#![doc(html_root_url = "https://docs.rs/bc-ur/0.1.0")]
#![warn(rust_2018_idioms)]

mod ur;
pub use crate::ur::UR;

mod utils;
mod error;

mod ur_encodable;
pub use ur_encodable::UREncodable;

mod ur_decodable;
pub use ur_decodable::URDecodable;

mod ur_codable;
pub use ur_codable::URCodable;
