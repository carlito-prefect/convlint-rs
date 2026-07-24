use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::ConvlintResult;

/// The name of the config file.
pub const CONF_FILE_NAME: &str = "Convlint.toml";

/// The configuration of the convlint execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ConvlintTOML {}

impl ConvlintTOML {
    /// Creates a new [`ConvlintTOML`].
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Read the configuration file and deserialize it into
    /// a [`ConvlintTOML`] configuration.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file could not
    /// be read or the read content cannot be deserialized.
    pub(crate) fn from_file(path: &Path) -> ConvlintResult<Self> {
        let file_content = fs::read_to_string(path)?;
        Ok(toml::from_str(&file_content)?)
    }

    /// Serialize the current config to string and write it
    /// to the configuration file.
    ///
    /// If the configuration file does not exist, [`fs::write`]
    /// creates a new one.
    ///
    /// # Errors
    ///
    /// This function will return an error if the the configuration
    /// could not be serialized to toml or cannot be written
    /// to the configuration file.
    pub(crate) fn write_to_file(&self, path: &Path) -> ConvlintResult<()> {
        let toml_str = toml::to_string(self)?;
        fs::write(path, &toml_str)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::{fs, io};

    use rstest::{fixture, rstest};
    use tempfile::TempDir;

    use crate::{
        config::{CONF_FILE_NAME, ConvlintTOML},
        error::ConvlintError,
    };

    #[fixture]
    fn temp_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    #[fixture]
    fn valid_config_str() -> String {
        String::new()
    }

    #[fixture]
    fn invalid_config_str() -> String {
        String::from("some invalid toml")
    }

    #[fixture]
    fn valid_config() -> ConvlintTOML {
        ConvlintTOML::new()
    }

    #[rstest]
    fn parse_config_file_success(temp_dir: TempDir, valid_config_str: String) {
        let valid_conf_toml = toml::from_str(&valid_config_str).unwrap();

        let conf_file_path = temp_dir.path().join(CONF_FILE_NAME);
        let _ = dbg!(fs::write(&conf_file_path, &valid_config_str));
        assert!(fs::write(&conf_file_path, valid_config_str).is_ok());

        let conf_res = ConvlintTOML::from_file(&conf_file_path);
        assert!(conf_res.is_ok());
        assert_eq!(conf_res.unwrap(), valid_conf_toml);
    }

    #[rstest]
    fn parse_config_file_not_found(temp_dir: TempDir) {
        let conf_res = ConvlintTOML::from_file(&temp_dir.path().join(CONF_FILE_NAME));
        assert!(conf_res.is_err());
        assert!(matches!(
            conf_res.unwrap_err(),
            ConvlintError::IoError(e) if e.kind() == io::ErrorKind::NotFound
        ));
    }

    #[rstest]
    fn parse_config_file_fail(temp_dir: TempDir, invalid_config_str: String) {
        let conf_file_path = temp_dir.path().join(CONF_FILE_NAME);
        assert!(fs::write(&conf_file_path, invalid_config_str).is_ok());

        let conf_res = ConvlintTOML::from_file(&conf_file_path);
        assert!(conf_res.is_err());
        assert!(matches!(
            conf_res.unwrap_err(),
            ConvlintError::TomlDeserializationError(_)
        ));
    }

    #[rstest]
    fn serialize_write_to_file_success(temp_dir: TempDir, valid_config: ConvlintTOML) {
        let toml_str = toml::to_string(&valid_config);
        assert!(toml_str.is_ok());
        assert!(fs::write(temp_dir.path().join(CONF_FILE_NAME), toml_str.unwrap()).is_ok());
    }

    #[rstest]
    fn serialize_write_to_file_missing_dir(temp_dir: TempDir, valid_config: ConvlintTOML) {
        let toml_str = toml::to_string(&valid_config);
        assert!(toml_str.is_ok());
        assert!(
            fs::write(
                temp_dir
                    .path()
                    .join("some_invalid_dir")
                    .join(CONF_FILE_NAME),
                toml_str.unwrap()
            )
            .is_err()
        );
    }
}
