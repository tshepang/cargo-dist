//! Code for generating installer.sh

use axoasset::LocalAsset;
use serde::Serialize;

use super::InstallerInfo;
use crate::{
    backend::templates::{Templates, TEMPLATE_INSTALLER_RB},
    errors::DistResult,
    installer::ExecutableZipFragment,
};

/// Info about a Homebrew formula
#[derive(Debug, Clone, Serialize)]
pub struct HomebrewInstallerInfo {
    /// The application's name
    pub name: String,
    /// Formula class name
    pub formula_class: String,
    /// The application's license, in SPDX format
    pub license: Option<String>,
    /// The URL to the application's homepage
    pub homepage: Option<String>,
    /// A brief description of the application
    pub desc: Option<String>,
    /// AMD64 artifact
    pub x86_64: Option<ExecutableZipFragment>,
    /// ARM64 artifact
    pub arm64: Option<ExecutableZipFragment>,
    /// Generic installer info
    pub inner: InstallerInfo,
}

pub(crate) fn write_homebrew_formula(
    templates: &Templates,
    info: &HomebrewInstallerInfo,
) -> DistResult<()> {
    let script = templates.render_file_to_clean_string(TEMPLATE_INSTALLER_RB, info)?;
    LocalAsset::write_new(&script, &info.inner.dest_path)?;
    Ok(())
}
