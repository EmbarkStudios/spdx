use anyhow::{Context as _, Result, bail};
use serde_json::{Value, map};
use std::{
    env,
    io::{self, Write},
    process,
};

type Map = map::Map<String, Value>;

#[inline]
fn get<'a>(m: &'a Map, k: &str) -> Result<&'a Value> {
    m.get(k)
        .with_context(|| format!("Malformed JSON: {m:?} lacks {k}"))
}

const IMPRECISE: &str = include_str!("imprecise.rs");
const ROOT: &str = "target/spdx-data";

fn write_exception_texts(
    texts: &mut impl Write,
    exceptions: impl Iterator<Item = impl AsRef<str>>,
) -> Result<()> {
    // Splat the license text into their own file and accumulate
    writeln!(texts, "\npub const EXCEPTION_TEXTS: &[(&str, &str)] = &[")?;

    for exc in exceptions {
        let exc = exc.as_ref();
        let text_path = format!("src/text/exceptions/{}", exc);
        if !std::path::Path::new(&text_path).exists() {
            let json: Map = serde_json::from_str(
                &std::fs::read_to_string(format!("{ROOT}/json/exceptions/{exc}.json"))
                    .with_context(|| format!("unable to open exceptions/{exc}.json"))?,
            )
            .with_context(|| format!("unable to deserialize exceptions/{exc}.json"))?;

            let text = get(&json, "licenseExceptionText")
                .with_context(|| format!("failed to get license exception text for {exc}"))?;

            std::fs::write(
                text_path,
                format!(
                    "r#\"{}\"#",
                    text.as_str()
                        .context("licenseExceptionText is not a string")?
                ),
            )
            .with_context(|| format!("failed to write license exception text for {exc}"))?;
        }

        writeln!(
            texts,
            "    (\"{0}\", include!(\"text/exceptions/{0}\")),",
            exc
        )?;
    }

    writeln!(texts, "];\n")?;

    Ok(())
}

fn write_exceptions(identifiers: &mut impl Write, texts: &mut impl Write) -> Result<()> {
    let json: Map = serde_json::from_str(
        &std::fs::read_to_string(format!("{ROOT}/json/exceptions.json"))
            .context("unable to open exceptions.json")?,
    )
    .context("failed to deserialize exceptions.json")?;

    let exceptions = get(&json, "exceptions")?;
    let exceptions = if let Value::Array(ref v) = exceptions {
        v
    } else {
        bail!("Malformed JSON: {:?}", exceptions)
    };
    eprintln!("#exceptions == {}", exceptions.len());

    let mut v = vec![];
    for exc in exceptions.iter() {
        let exc = if let Value::Object(m) = exc {
            m
        } else {
            bail!("Malformed JSON: {:?}", exc)
        };

        let lic_exc_id = get(exc, "licenseExceptionId")?;
        if let Value::String(s) = lic_exc_id {
            let flags = match get(exc, "isDeprecatedLicenseId") {
                Ok(Value::Bool(val)) => {
                    if *val {
                        "IS_DEPRECATED"
                    } else {
                        "0"
                    }
                }
                _ => "0",
            };

            v.push((s, flags));
        } else {
            bail!("Malformed JSON: {:?}", lic_exc_id)
        };
    }

    writeln!(identifiers, "pub const EXCEPTIONS: &[Exception] = &[")?;
    v.sort_by_key(|v| v.0);
    for (index, (exc, flags)) in v.iter().enumerate() {
        writeln!(identifiers, "    Exception {{ name: \"{exc}\", index: {index}, flags: {flags} }},")?;
    }
    writeln!(identifiers, "];")?;

    write_exception_texts(texts, v.into_iter().map(|(exc, _)| exc))
}

fn is_copyleft(license: &str) -> bool {
    // Copyleft licenses are determined from
    // https://www.gnu.org/licenses/license-list.en.html
    // and no distinction is made between "weak" and "strong"
    // copyleft, for simplicity
    license.starts_with("AGPL-")
        || license.starts_with("CC-BY-NC-SA-")
        || license.starts_with("CC-BY-SA-")
        || license.starts_with("CECILL-")
        || license.starts_with("CPL-")
        || license.starts_with("CDDL-")
        || license.starts_with("EUPL")
        || license.starts_with("GFDL-")
        || license.starts_with("GPL-")
        || license.starts_with("LGPL-")
        || license.starts_with("MPL-")
        || license.starts_with("NPL-")
        || license.starts_with("OSL-")
        || license == "BSD-Protection"
        || license == "MS-PL"
        || license == "MS-RL"
        //|| license == "OpenSSL" <- this one seems to be debated, but not really copyleft
        || license == "Parity-6.0.0"
        || license == "SISSL"
        || license == "xinetd"
        || license == "YPL-1.1"
}

fn is_gnu(license: &str) -> bool {
    license.starts_with("AGPL-")
        || license.starts_with("GFDL-")
        || license.starts_with("GPL-")
        || license.starts_with("LGPL-")
}

fn write_license_texts(
    texts: &mut impl Write,
    licenses: impl Iterator<Item = impl AsRef<str>>,
) -> Result<()> {
    // Splat the license text into their own file and accumulate
    writeln!(texts, "pub const LICENSE_TEXTS: &[(&str, &str)] = &[")?;

    for license in licenses {
        let license = license.as_ref();
        if license == "NOASSERTION" {
            writeln!(texts, "    (\"{license}\", \"\"),")?;
            continue;
        }

        use std::borrow::Cow;
        let license_name = if license.starts_with("GFDL-") {
            if let Some(root) = license.strip_suffix("-invariants") {
                Cow::Owned(format!("{}-invariants-only", root))
            } else {
                Cow::Borrowed(license)
            }
        } else {
            Cow::Borrowed(license)
        };

        let text_path = format!("src/text/licenses/{}", license_name);
        if !std::path::Path::new(&text_path).exists() {
            let json: Map = serde_json::from_str(
                &std::fs::read_to_string(format!("{ROOT}/json/details/{license_name}.json"))
                    .with_context(|| format!("unable to open details/{license_name}.json"))?,
            )
            .with_context(|| format!("unable to deserialize details/{license_name}.json"))?;

            let text = get(&json, "licenseText")
                .with_context(|| format!("failed to get license text for {license_name}"))?;

            std::fs::write(
                text_path,
                format!(
                    "r#\"{}\"#",
                    text.as_str().context("licenseText is not a string")?
                ),
            )
            .with_context(|| format!("failed to write license text for {license_name}"))?;
        }

        writeln!(
            texts,
            "    (\"{license}\", include!(\"text/licenses/{license_name}\")),"
        )?;
    }

    writeln!(texts, "];\n")?;

    Ok(())
}

fn write_licenses(identifiers: &mut impl Write, texts: &mut impl Write) -> Result<()> {
    writeln!(
        identifiers,
        "
use crate::{{Exception, License, flags::*}};
"
    )?;

    let json: Map = serde_json::from_str(
        &std::fs::read_to_string(format!("{ROOT}/json/licenses.json"))
            .context("unable to open licenses.json")?,
    )
    .context("failed to deserialize licenses.json")?;

    let licenses = get(&json, "licenses")?;
    let licenses = if let Value::Array(v) = licenses {
        v
    } else {
        bail!("Malformed JSON: {licenses:?}")
    };
    eprintln!("#licenses == {}", licenses.len());

    let mut v = vec![];
    for lic in licenses.iter() {
        let lic = if let Value::Object(ref m) = *lic {
            m
        } else {
            bail!("Malformed JSON: {lic:?}")
        };

        let lic_id = get(lic, "licenseId")?;
        if let Value::String(id) = lic_id {
            let mut flags = String::with_capacity(100);

            if let Ok(Value::Bool(val)) = get(lic, "isDeprecatedLicenseId") {
                if *val {
                    flags.push_str("IS_DEPRECATED | ");
                }
            }

            if let Ok(Value::Bool(val)) = get(lic, "isOsiApproved") {
                if *val {
                    flags.push_str("IS_OSI_APPROVED | ");
                }
            }

            if let Ok(Value::Bool(val)) = get(lic, "isFsfLibre") {
                if *val {
                    flags.push_str("IS_FSF_LIBRE | ");
                }
            }

            if is_copyleft(id) {
                flags.push_str("IS_COPYLEFT | ");
            }

            if is_gnu(id) {
                flags.push_str("IS_GNU | ");
            }

            if flags.is_empty() {
                flags.push_str("0x0");
            } else {
                // Strip the trailing ` | `
                flags.truncate(flags.len() - 3);
            }

            let full_name = if let Value::String(name) = get(lic, "name")? {
                name
            } else {
                id
            };

            // Add `-invariants` versions of the root GFDL-<version>-invariants-only
            // licenses so that they work slightly nicer
            if id.starts_with("GFDL-") {
                if let Some(id) = id.strip_suffix("-invariants-only") {
                    v.push((format!("{id}-invariants"), full_name, flags.clone()));
                }
            }

            v.push((id.to_owned(), full_name, flags));
        } else {
            bail!("Malformed JSON: {lic_id:?}");
        }
    }

    let name = "NOASSERTION".to_owned();
    // Add NOASSERTION, which is not yet? part of the SPDX spec
    // https://github.com/spdx/spdx-spec/issues/50
    v.push(("NOASSERTION".to_owned(), &name, "0x0".to_owned()));

    v.sort_by(|a, b| a.0.cmp(&b.0));

    let lic_list_ver = get(&json, "licenseListVersion")?;
    if let Value::String(s) = lic_list_ver {
        writeln!(identifiers, "pub const VERSION: &str = {s:?};")?;
    } else {
        bail!("Malformed JSON: {lic_list_ver:?}")
    }
    writeln!(identifiers)?;
    writeln!(identifiers, "pub const LICENSES: &[License] = &[")?;
    for (index, (id, name, flags)) in v.iter().enumerate() {
        writeln!(identifiers, "    License {{ name: \"{id}\", full_name: r#\"{name}\"#, index: {index}, flags: {flags} }},")?;
    }
    writeln!(identifiers, "];\n")?;

    write_license_texts(texts, v.into_iter().map(|(name, _, _)| name))
}

fn real_main() -> Result<()> {
    let mut upstream_tag = None;
    let mut debug = false;
    for e in env::args().skip(1) {
        match e.as_str() {
            "-d" => {
                debug = true;
            }
            s if s.starts_with('v') => upstream_tag = Some(s.to_owned()),
            _ => bail!("Unknown option {:?}", e),
        }
    }

    let upstream_tag = match upstream_tag {
        None => {
            eprintln!(
                "WARN: fetching data from the master branch of spdx/license-list-data; \
                 consider specifying a tag (e.g. v3.0)"
            );

            "master".to_owned()
        }
        Some(ut) => {
            if debug {
                eprintln!("Using tag {ut:?}");
            }
            ut
        }
    };

    if !std::path::Path::new(ROOT).exists() {
        println!("cloning...");
        assert!(
            std::process::Command::new("git")
                .args([
                    "clone",
                    "https://github.com/spdx/license-list-data.git",
                    ROOT
                ])
                .status()
                .unwrap()
                .success()
        );
    } else {
        println!("fetching...");
        assert!(
            std::process::Command::new("git")
                .args(["-C", ROOT, "fetch"])
                .status()
                .unwrap()
                .success()
        );
    }

    println!("checking out...");
    assert!(
        std::process::Command::new("git")
            .args(["-C", ROOT, "checkout", &upstream_tag])
            .status()
            .unwrap()
            .success()
    );

    {
        let mut identifiers = io::BufWriter::new(std::fs::File::create("src/identifiers.rs")?);

        writeln!(
            identifiers,
            "\
/*
 * list fetched from https://github.com/spdx/license-list-data @ {}
 *
 * AUTO-GENERATED BY ./update
 * DO NOT MODIFY
 *
 * cargo run --manifest-path update/Cargo.toml -- v<version> > src/identifiers.rs
*/",
            upstream_tag
        )?;

        std::fs::remove_dir_all("src/text").context("failed to nuke directory")?;

        let mut texts = io::BufWriter::new(std::fs::File::create("src/text.rs")?);

        std::fs::create_dir_all("src/text/licenses")
            .context("failed to create licenses text dir")?;
        write_licenses(&mut identifiers, &mut texts)?;

        // Add the contents or imprecise.rs, which maps invalid identifiers to
        // valid ones
        writeln!(identifiers, "{}", IMPRECISE)?;

        std::fs::create_dir_all("src/text/exceptions")
            .context("failed to create exceptions text dir")?;
        write_exceptions(&mut identifiers, &mut texts)?;
    }

    // Run rustfmt on the final files
    std::process::Command::new("rustfmt")
        .args(["--edition", "2018", "src/identifiers.rs"])
        .status()
        .context("failed to run rustfmt")?;

    std::process::Command::new("rustfmt")
        .args(["--edition", "2018", "src/text.rs"])
        .status()
        .context("failed to run rustfmt")?;

    let readme = std::fs::read_to_string("README.md").context("failed to read README.md")?;

    const VERSION: &str = "SPDX%20Version-";

    let index = readme
        .find(VERSION)
        .context("failed to find SPDX version")?;
    let end_index = readme[index + VERSION.len()..]
        .find('-')
        .context("failed to find version end")?
        + index
        + VERSION.len();

    let mut rmfile = std::io::BufWriter::new(
        std::fs::File::create("README.md").context("failed to open README.md")?,
    );
    rmfile
        .write(&readme.as_bytes()[..index + VERSION.len()])
        .context("failed to write prefix")?;
    rmfile
        .write(&upstream_tag.as_bytes()[1..])
        .context("failed to write version")?;
    rmfile
        .write(&readme.as_bytes()[end_index..])
        .context("failed to write suffix")?;

    Ok(())
}

fn main() {
    if let Err(ref e) = real_main() {
        eprintln!("error: {e:#}");
        process::exit(1);
    }
}
