// Work in Progress

use mustache;
//use schemafy;
use std::env;
use std::fs::{self, File, Metadata};
use std::io;
use std::path::{Path, PathBuf};

async fn _generate_format_types() -> Result<(), String> {
    // Fetch all schemas within the @web3api/manifest-schemas/formats directory
    let formats_dir = Path::new("@web3api/manifest-schemas").join("formats");

    // Get all format types (web3api, web3api.build, etc)
    let mut format_types = fs::read_dir(&formats_dir)
        .expect("Failed to read directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Failed to collect schemas");

    // The order in which `read_dir` returns entries is not guaranteed.
    // We're explicitly sorting the entries, assuming reproducible ordering is required.
    format_types.sort();

    // For each format type
    for path_buf in format_types {
        let format_type_name = path_buf.file_name().unwrap();
        let format_type_dir = Path::new(&formats_dir).join(format_type_name);
        let mut format_modules: Vec<Metadata> = vec![];

        // Get all JSON schemas for this format type (v1, v2, etc)
        let format_schema_files = fs::read_dir(&format_type_dir)
            .expect("Failed to read directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .expect("Failed to collect schemas");
        let mut format_schemas: Vec<PathBuf> = vec![];

        for dir in format_schema_files {
            let mut format_schema_name = dir;
            format_schema_name.set_extension("");
            let format_schema_path = Path::new(&format_type_dir).join(format_schema_name);

            let mut actions = || -> Result<(), String> {
                // Parse the JSON schema

                let reader = File::open(&format_schema_path).expect("Failed to open file");
                let format_schema: PathBuf =
                    serde_json::from_reader(reader).expect("Failed to parse JSON schema");

                // Insert the __type property for introspection
                // TODO

                format_schemas.push(format_schema.clone());

                // Convert the format schema to a Rust type
                // TODO
                let rs_file: Vec<u8> = vec![]; // for now

                // Emit the result
                let rs_output_path = Path::new("/../../src/manifest/formats/")
                    .join(format_type_name)
                    .join(".rs");
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(rs_output_path.clone())
                    .expect("Failed to make a new directory");

                // TODO: write-to-file operation
                fs::write(rs_output_path, rs_file).expect("Failed to write rs_file"); // for now

                // Add metadata for the root mod.rs file to use
                format_modules
                    .push(Path::metadata(&format_schema).expect("Failed to return metadata"));

                Ok(())
            };
            if let Err(error) = actions() {
                return Err(format!(
                    "Error generating the Manifest file {:?}: {}",
                    format_schema_path, &error
                ));
            }
        }

        let _render_template = |name: PathBuf| {
            let current_dir = env::current_dir().expect("Failed to retrieve current directory");
            let rs_template = fs::read(current_dir.join(&name).join("-rs.mustache"))
                .expect("Failed to read file");

            // Render the template
            // Is this correct?
            let template =
                mustache::compile_path(current_dir).expect("Error in compiling current dir");
            let rs_src = template
                .render_to_string(&rs_template)
                .expect("Failed to render template");

            // Emit the source
            let rs_output_path = Path::new("/../../src/manifest/formats/")
                .join(format_type_name)
                .join("/")
                .join(name)
                .join(".rs");
            fs::DirBuilder::new()
                .recursive(true)
                .create(rs_output_path.clone())
                .expect("Failed to make a new directory");
            // TODO: write-to-file operation
            // for now
            fs::write(rs_output_path, rs_src.as_bytes()).expect("Failed to write rs_file");
        };

        let _last_item = |arr: Vec<u8>| arr.last().unwrap().to_owned();
    }
    Ok(())
}

fn main() {}
