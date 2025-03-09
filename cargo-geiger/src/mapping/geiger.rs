use super::metadata::package_id::{GetPackageIdRepr, ToCargoMetadataPackage};
use super::ToCargoGeigerSource;

use krates::cm::Metadata;
use url::Url;

use cargo_geiger_serde::Source as CargoGeigerSerdeSource;
use krates::cm::PackageId as CargoMetadataPackageId;

impl ToCargoGeigerSource for CargoMetadataPackageId {
    fn to_cargo_geiger_source(
        &self,
        metadata: &Metadata,
    ) -> CargoGeigerSerdeSource {
        let package = self.to_cargo_metadata_package(metadata).unwrap();

        match package.source {
            Some(source) => handle_source_repr(&source.repr),
            None => handle_path_source(self),
        }
    }
}

fn handle_source_repr(source_repr: &str) -> CargoGeigerSerdeSource {
    let mut source_repr_vec = source_repr.split('+').collect::<Vec<&str>>();

    let source_type = source_repr_vec[0];

    match SourceType::from(source_type) {
        SourceType::Registry => {
            CargoGeigerSerdeSource::Registry {
                // It looks like cargo metadata drops this information
                name: String::from("crates.io"),
                url: Url::parse(source_repr_vec.pop().unwrap()).unwrap(),
            }
        }
        SourceType::Git => {
            let raw_git_representation = source_repr_vec.pop().unwrap();
            let raw_git_url = Url::parse(raw_git_representation).unwrap();
            let git_url_without_query = format!(
                "{}://{}{}",
                raw_git_url.scheme(),
                raw_git_url.host_str().unwrap(),
                raw_git_url.path()
            );
            let revision = raw_git_url
                .query_pairs()
                .find(|(query_key, _)| query_key == "rev")
                .map(|(_, rev)| String::from(rev))
                .unwrap_or_default();

            CargoGeigerSerdeSource::Git {
                url: Url::parse(&git_url_without_query).unwrap(),
                rev: revision,
            }
        }
        _ => {
            panic!("Unrecognised source type: {}", source_type)
        }
    }
}

fn handle_path_source<T: GetPackageIdRepr>(
    package_id: &T,
) -> CargoGeigerSerdeSource {
    let raw_repr = package_id.get_package_id_repr();
    let raw_path_repr = raw_repr[1..raw_repr.len() - 1]
        .split("+file://")
        .skip(1)
        .collect::<Vec<&str>>()
        .pop()
        .unwrap();

    let source_url = if cfg!(windows) {
        Url::from_file_path(&raw_path_repr[1..]).unwrap()
    } else {
        Url::from_file_path(raw_path_repr).unwrap()
    };

    CargoGeigerSerdeSource::Path(source_url)
}

enum SourceType {
    Registry,
    Git,
    Unrecognised,
}

impl SourceType {
    fn from(raw: &str) -> Self {
        match raw {
            "registry" => SourceType::Registry,
            "git" => SourceType::Git,
            _ => SourceType::Unrecognised,
        }
    }
}

#[cfg(test)]
mod geiger_tests {
    use super::*;

    use rstest::*;
    use url::Url;

    #[rstest(
        input_source_repr,
        expected_source,
        case(
            "registry+https://github.com/rust-lang/crates.io-index",
            CargoGeigerSerdeSource::Registry {
                name: String::from("crates.io"),
                url: Url::parse("https://github.com/rust-lang/crates.io-index").unwrap()
            }
        ),
        case(
            "git+https://github.com/rust-itertools/itertools.git?rev=98d3978",
            CargoGeigerSerdeSource::Git {
                url: Url::parse("https://github.com/rust-itertools/itertools.git").unwrap(),
                rev: String::from("98d3978")
            }
        ),
        case(
            "git+https://github.com/rust-itertools/itertools.git",
            CargoGeigerSerdeSource::Git {
                url: Url::parse("https://github.com/rust-itertools/itertools.git").unwrap(),
                rev: String::from("")
            }
        )
    )]
    fn handle_source_repr_test(
        input_source_repr: &str,
        expected_source: CargoGeigerSerdeSource,
    ) {
        let source = handle_source_repr(input_source_repr);
        assert_eq!(source, expected_source);
    }

    #[rstest]
    fn handle_path_source_test() {
        if !cfg!(windows) {
            let package_id = CargoMetadataPackageId {
                repr: String::from("(path+file:///cargo_geiger/test_crates/test1_package_with_no_deps)"),
            };

            let expected_source = CargoGeigerSerdeSource::Path(
                Url::from_file_path(
                    "/cargo_geiger/test_crates/test1_package_with_no_deps",
                )
                .unwrap(),
            );

            let source = handle_path_source(&package_id);
            assert_eq!(source, expected_source);
        }
    }
}
