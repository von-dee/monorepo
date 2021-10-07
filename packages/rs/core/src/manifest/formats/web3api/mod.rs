pub mod migrate;
pub mod migrators;
pub mod prealpha_001_1;
pub mod prealpha_001_2;
pub mod prealpha_001_3;
pub mod prealpha_001_4;
pub mod prealpha_001_5;

pub use prealpha_001_1::Web3ApiManifest as Web3ApiManifest001Prealpha1;
pub use prealpha_001_2::Web3ApiManifest as Web3ApiManifest001Prealpha2;
pub use prealpha_001_3::Web3ApiManifest as Web3ApiManifest001Prealpha3;
pub use prealpha_001_4::Web3ApiManifest as Web3ApiManifest001Prealpha4;
pub use prealpha_001_5::Web3ApiManifest as Web3ApiManifest001Prealpha5;

pub enum Web3ApiManifestFormats {
    Prealpha001_1(&'static str),
    Prealpha001_2(&'static str),
    Prealpha001_3(&'static str),
    Prealpha001_4(&'static str),
    Prealpha001_5(&'static str),
}

pub enum AnyWeb3ApiManifest {
    Web3ApiManifest001Prealpha1(Web3ApiManifest001Prealpha1),
    Web3ApiManifest001Prealpha2(Web3ApiManifest001Prealpha2),
    Web3ApiManifest001Prealpha3(Web3ApiManifest001Prealpha3),
    Web3ApiManifest001Prealpha4(Web3ApiManifest001Prealpha4),
    Web3ApiManifest001Prealpha5(Web3ApiManifest001Prealpha5),
}

pub type Web3ApiManifest = Web3ApiManifest001Prealpha5;

pub const LATEST_WEB3_API_MANIFEST_FORMAT: Web3ApiManifestFormats =
    Web3ApiManifestFormats::Prealpha001_5("0.0.1-prealpha.5");