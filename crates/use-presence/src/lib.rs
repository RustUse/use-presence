#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_attribution as attribution;
pub use use_campaign as campaign;
pub use use_canonical as canonical;
pub use use_geo as geo;
pub use use_listing as listing;
pub use use_local as local;
pub use use_metadata as metadata;
pub use use_referrer as referrer;
pub use use_robots as robots;
pub use use_schema as schema;
pub use use_seo as seo;
pub use use_sitemap as sitemap;
pub use use_utm as utm;

/// Common presence primitive reexports.
pub mod prelude;
