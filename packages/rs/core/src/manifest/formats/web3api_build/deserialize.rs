use super::super::super::DeserializeManifestOptions;
use super::{
    migrate_build_manifest, validate_build_manifest, AnyBuildManifest, BuildManifest,
    LATEST_BUILD_MANIFEST_FORMAT,
};
use semver::{Version, VersionReq};

pub fn deserialize_build_manifest(
    manifest: &str,
    options: Option<DeserializeManifestOptions>,
) -> Result<BuildManifest, String> {
    let mut any_build_manifest: AnyBuildManifest = serde_yaml::from_str(manifest)
        .expect(&format!("Unable to parse BuildManifest: {}", manifest));

    if options.is_some() && !options.clone().unwrap().no_validate.unwrap() {
        validate_build_manifest(&any_build_manifest, options.unwrap().ext_schema).unwrap();
    }

    let version_req = VersionReq::parse(LATEST_BUILD_MANIFEST_FORMAT).unwrap();

    let version_compare = match &mut any_build_manifest {
        AnyBuildManifest::BuildManifest001Prealpha1(one) => {
            one.__type = "BuildManifest".to_string();
            version_req.matches(&Version::parse(&one.format).unwrap())
        }
        AnyBuildManifest::BuildManifest001Prealpha2(two) => {
            two.__type = "BuildManifest".to_string();
            version_req.matches(&Version::parse(&two.format).unwrap())
        }
    };

    if !version_compare {
        // upgrade
        return migrate_build_manifest(&mut any_build_manifest, LATEST_BUILD_MANIFEST_FORMAT);
    } else {
        // downgrade
        let format = match &mut any_build_manifest {
            AnyBuildManifest::BuildManifest001Prealpha1(one) => one.format.clone(),
            AnyBuildManifest::BuildManifest001Prealpha2(two) => two.format.clone(),
        };
        return Err(format!(
            "Cannot downgrade Web3API version {}, please upgrade your Web3ApiClient package",
            &format
        ));
    }
}
