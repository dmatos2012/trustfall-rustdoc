use simd_json::prelude::ValueObjectAccessAsScalar;
use simd_json::Tape;
use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::{bail, Context};
use serde::Deserialize;
use simd_json;

use crate::versioned::VersionedCrate;

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

fn detect_rustdoc_format_version(path: &Path, file_data: &str) -> anyhow::Result<u32> {
    let version = serde_json::from_str::<RustdocFormatVersion>(file_data)
        .with_context(|| format!("unrecognized rustdoc format for file {}", path.display()))?;

    Ok(version.format_version)
}

fn simd_detect_rustdoc_format_version(path: &Path, tape: &Tape) -> anyhow::Result<u32> {
    //let version =
    //    simd_json::serde::from_slice::<RustdocFormatVersion>(&mut file_data.as_bytes().to_owned())
    //        .with_context(|| format!("unrecognized rustdoc format for file {}", path.display()))?;
    let version = tape
        .as_value()
        .get_u32("format_version")
        .with_context(|| format!("unrecognized rustdoc format for file {}", path.display()))?;
    //Ok(version.format_version)
    Ok(version)
}

fn parse_or_report_error<T>(path: &Path, file_data: &str, format_version: u32) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_str(file_data).with_context(|| {
        format!(
            "unexpected parse error for v{format_version} rustdoc for file {}",
            path.display()
        )
    })
}

fn simd_parse_or_report_error<T>(path: &Path, tape: Tape, format_version: u32) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    tape.deserialize().with_context(|| {
        format!(
            "unexpected parse error for v{format_version} rustdoc for file {}",
            path.display()
        )
    })
    //simd_json::serde::from_slice(file_data).with_context(|| {
    //    format!(
    //        "unexpected parse error for v{format_version} rustdoc for file {}",
    //        path.display()
    //    )
    //})
}

pub fn simd_load_rustdoc(path: &Path) -> anyhow::Result<VersionedCrate> {
    //let mut file_data = String::new();
    // Read_to_end does not do utf8 validation, as its not read into a string
    // utf8 validation is done in simd_json
    let mut file_data = Vec::new();
    File::open(path)
        .with_context(|| format!("failed to open rustdoc JSON file {}", path.display()))?
        .read_to_end(&mut file_data)
        .with_context(|| format!("failed to read rustdoc JSON file {}", path.display()))?;

    let tape = simd_json::to_tape(&mut file_data)?;
    let format_version = simd_detect_rustdoc_format_version(path, &tape)?;
    match format_version {
        #[cfg(feature = "v28")]
        28 => Ok(VersionedCrate::V28(simd_parse_or_report_error(
            path,
            tape,
            format_version,
        )?)),

        #[cfg(feature = "v29")]
        29 => Ok(VersionedCrate::V29(simd_parse_or_report_error(
            path,
            tape,
            format_version,
        )?)),

        #[cfg(feature = "v30")]
        30 => Ok(VersionedCrate::V30(simd_parse_or_report_error(
            path,
            tape,
            format_version,
        )?)),

        #[cfg(feature = "v32")]
        32 => Ok(VersionedCrate::V32(simd_parse_or_report_error(
            path,
            tape,
            format_version,
        )?)),

        #[cfg(feature = "v33")]
        33 => Ok(VersionedCrate::V33(simd_parse_or_report_error(
            path,
            tape,
            format_version,
        )?)),

        _ => bail!(
            "rustdoc format v{format_version} for file {} is not supported",
            path.display()
        ),
    }
}

pub fn load_rustdoc(path: &Path) -> anyhow::Result<VersionedCrate> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut file_data = String::new();
    File::open(path)
        .with_context(|| format!("failed to open rustdoc JSON file {}", path.display()))?
        .read_to_string(&mut file_data)
        .with_context(|| format!("failed to read rustdoc JSON file {}", path.display()))?;

    let format_version = detect_rustdoc_format_version(path, &file_data)?;

    match format_version {
        #[cfg(feature = "v28")]
        28 => Ok(VersionedCrate::V28(parse_or_report_error(
            path,
            &file_data,
            format_version,
        )?)),

        #[cfg(feature = "v29")]
        29 => Ok(VersionedCrate::V29(parse_or_report_error(
            path,
            &file_data,
            format_version,
        )?)),

        #[cfg(feature = "v30")]
        30 => Ok(VersionedCrate::V30(parse_or_report_error(
            path,
            &file_data,
            format_version,
        )?)),

        #[cfg(feature = "v32")]
        32 => Ok(VersionedCrate::V32(parse_or_report_error(
            path,
            &file_data,
            format_version,
        )?)),

        #[cfg(feature = "v33")]
        33 => Ok(VersionedCrate::V33(parse_or_report_error(
            path,
            &file_data,
            format_version,
        )?)),

        _ => bail!(
            "rustdoc format v{format_version} for file {} is not supported",
            path.display()
        ),
    }
}
