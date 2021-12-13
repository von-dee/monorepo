use super::super::super::DeserializeManifestOptions;
use super::{
    migrate_web3_api_manifest, validate_web3_api_manifest, AnyWeb3ApiManifest, Web3ApiManifest,
    LATEST_WEB3_API_MANIFEST_FORMAT,
};

use semver::{Version, VersionReq};

pub fn deserialize_web3_api_manifest(
    manifest: &str,
    options: Option<DeserializeManifestOptions>,
) -> Result<Web3ApiManifest, String> {
    let mut any_web3_api_manifest: AnyWeb3ApiManifest = serde_yaml::from_str(manifest)
        .expect(&format!("Unable to parse Web3ApiManifest: {}", manifest));

    if options.is_some() && !options.clone().unwrap().no_validate.unwrap() {
        validate_web3_api_manifest(&any_web3_api_manifest, options.unwrap().ext_schema).unwrap();
    }

    let version_req = VersionReq::parse(LATEST_WEB3_API_MANIFEST_FORMAT).unwrap();

    let version_compare = match &mut any_web3_api_manifest {
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha1(one) => {
            one.__type = "Web3ApiManifest".to_string();
            version_req.matches(&Version::parse(&one.format).unwrap())
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha2(two) => {
            two.__type = "Web3ApiManifest".to_string();
            version_req.matches(&Version::parse(&two.format).unwrap())
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha3(three) => {
            three.__type = "Web3ApiManifest".to_string();
            version_req.matches(&Version::parse(&three.format).unwrap())
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha4(four) => {
            four.__type = "Web3ApiManifest".to_string();
            version_req.matches(&Version::parse(&four.format).unwrap())
        }
        AnyWeb3ApiManifest::Web3ApiManifest001Prealpha5(five) => {
            five.__type = "Web3ApiManifest".to_string();
            version_req.matches(&Version::parse(&five.format).unwrap())
        }
    };

    if !version_compare {
        // upgrade
        return migrate_web3_api_manifest(
            &mut any_web3_api_manifest,
            LATEST_WEB3_API_MANIFEST_FORMAT,
        );
    } else {
        // downgrade
        let format = match &mut any_web3_api_manifest {
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha1(one) => one.format.clone(),
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha2(two) => two.format.clone(),
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha3(three) => three.format.clone(),
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha4(four) => four.format.clone(),
            AnyWeb3ApiManifest::Web3ApiManifest001Prealpha5(five) => five.format.clone(),
        };
        return Err(format!(
            "Cannot downgrade Web3API version {}, please upgrade your Web3ApiClient package",
            &format
        ));
    }
}
