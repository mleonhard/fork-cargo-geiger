use crate::format::pattern::Pattern;
use crate::format::Chunk;
use crate::mapping::{CargoMetadataParameters, GetPackageIdInformation};

use krates::cm::PackageId;
use std::fmt;

pub struct Display<'a> {
    pub cargo_metadata_parameters: &'a CargoMetadataParameters<'a>,
    pub pattern: &'a Pattern,
    pub package: &'a PackageId,
}

impl<'a> fmt::Display for Display<'a> {
    // This clippy recommendation is quite strict, not allowing an error message
    // to be written out when failing to format the struct.
    // Perhaps we shouldn't be using `impl fmt::Display` at all, and instead
    // should define our own trait for anything which is to be constructed for
    // outputting?
    #[allow(clippy::print_in_format_impl)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for chunk in &self.pattern.chunks {
            match *chunk {
                Chunk::License => {
                    if let Some(ref license) =
                        self.package.get_package_id_licence(
                            self.cargo_metadata_parameters.krates,
                        )
                    {
                        (write!(fmt, "{}", license))?
                    }
                }
                Chunk::Package => {
                    if let Some((package_name, package_version)) =
                        self.package.get_package_id_name_and_version(
                            self.cargo_metadata_parameters.krates,
                        )
                    {
                        (write!(fmt, "{} {}", package_name, package_version))?
                    } else {
                        eprintln!("Failed to format Package: {}", self.package)
                    }
                }
                Chunk::Raw(ref s) => (fmt.write_str(s))?,
                Chunk::Repository => {
                    if let Some(ref repository) =
                        self.package.get_package_id_repository(
                            self.cargo_metadata_parameters.krates,
                        )
                    {
                        (write!(fmt, "{}", repository))?
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod display_tests {
    use super::*;

    use crate::format::pattern::Pattern;
    use crate::format::Chunk;

    use krates::cm::{CargoOpt, MetadataCommand};
    use krates::Builder as KratesBuilder;
    use rstest::*;

    #[rstest(
        input_pattern,
        expected_formatted_string,
        case(
            Pattern::new(vec![Chunk::License]),
            "Apache-2.0/MIT"
        ),
        case(
            Pattern::new(vec![Chunk::Raw(String::from("chunk_value"))]),
            "chunk_value"
        ),
        case(
            Pattern::new(vec![Chunk::Repository]),
            "https://github.com/rust-secure-code/cargo-geiger"
        )
    )]
    fn display_format_fmt_test(
        input_pattern: Pattern,
        expected_formatted_string: &str,
    ) {
        let metadata = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let krates = KratesBuilder::new()
            .build_with_metadata(metadata.clone(), |_| ())
            .unwrap();

        let package_id = metadata.root_package().unwrap().id.clone();

        let display = Display {
            cargo_metadata_parameters: &CargoMetadataParameters {
                krates: &krates,
                metadata: &metadata,
            },
            pattern: &input_pattern,
            package: &package_id,
        };

        assert_eq!(format!("{}", display), expected_formatted_string);
    }
}
