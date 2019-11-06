/*
 * list fetched from https://github.com/spdx/license-list-data @ v3.6
 *
 * AUTO-GENERATED BY ./update
 * DO NOT MODIFY
 *
 * cargo run --manifest-path update/Cargo.toml -- v<version> > src/identifiers.rs
 */

pub const IS_FSF_LIBRE: u8 = 0x1;
pub const IS_OSI_APPROVED: u8 = 0x2;
pub const IS_DEPRECATED: u8 = 0x4;
pub const IS_COPYLEFT: u8 = 0x8;

pub const VERSION: &str = "3.6";

pub const LICENSES: &[(&str, &str, u8)] = &[
    ("0BSD", r#"BSD Zero Clause License"#, IS_OSI_APPROVED),
    ("AAL", r#"Attribution Assurance License"#, IS_OSI_APPROVED),
    ("ADSL", r#"Amazon Digital Services License"#, 0x0),
    ("AFL-1.1", r#"Academic Free License v1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("AFL-1.2", r#"Academic Free License v1.2"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("AFL-2.0", r#"Academic Free License v2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("AFL-2.1", r#"Academic Free License v2.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("AFL-3.0", r#"Academic Free License v3.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("AGPL-1.0", r#"Affero General Public License v1.0"#, IS_DEPRECATED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("AGPL-1.0-only", r#"Affero General Public License v1.0 only"#, IS_COPYLEFT),
    ("AGPL-1.0-or-later", r#"Affero General Public License v1.0 or later"#, IS_COPYLEFT),
    ("AGPL-3.0", r#"GNU Affero General Public License v3.0"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("AGPL-3.0-only", r#"GNU Affero General Public License v3.0 only"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("AGPL-3.0-or-later", r#"GNU Affero General Public License v3.0 or later"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("AMDPLPA", r#"AMD's plpa_map.c License"#, 0x0),
    ("AML", r#"Apple MIT License"#, 0x0),
    ("AMPAS", r#"Academy of Motion Picture Arts and Sciences BSD"#, 0x0),
    ("ANTLR-PD", r#"ANTLR Software Rights Notice"#, 0x0),
    ("APAFML", r#"Adobe Postscript AFM License"#, 0x0),
    ("APL-1.0", r#"Adaptive Public License 1.0"#, IS_OSI_APPROVED),
    ("APSL-1.0", r#"Apple Public Source License 1.0"#, IS_OSI_APPROVED),
    ("APSL-1.1", r#"Apple Public Source License 1.1"#, IS_OSI_APPROVED),
    ("APSL-1.2", r#"Apple Public Source License 1.2"#, IS_OSI_APPROVED),
    ("APSL-2.0", r#"Apple Public Source License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Abstyles", r#"Abstyles License"#, 0x0),
    ("Adobe-2006", r#"Adobe Systems Incorporated Source Code License Agreement"#, 0x0),
    ("Adobe-Glyph", r#"Adobe Glyph List License"#, 0x0),
    ("Afmparse", r#"Afmparse License"#, 0x0),
    ("Aladdin", r#"Aladdin Free Public License"#, 0x0),
    ("Apache-1.0", r#"Apache License 1.0"#, IS_FSF_LIBRE),
    ("Apache-1.1", r#"Apache License 1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Apache-2.0", r#"Apache License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Artistic-1.0", r#"Artistic License 1.0"#, IS_OSI_APPROVED),
    ("Artistic-1.0-Perl", r#"Artistic License 1.0 (Perl)"#, IS_OSI_APPROVED),
    ("Artistic-1.0-cl8", r#"Artistic License 1.0 w/clause 8"#, IS_OSI_APPROVED),
    ("Artistic-2.0", r#"Artistic License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("BSD-1-Clause", r#"BSD 1-Clause License"#, 0x0),
    ("BSD-2-Clause", r#"BSD 2-Clause "Simplified" License"#, IS_OSI_APPROVED),
    ("BSD-2-Clause-FreeBSD", r#"BSD 2-Clause FreeBSD License"#, IS_FSF_LIBRE),
    ("BSD-2-Clause-NetBSD", r#"BSD 2-Clause NetBSD License"#, 0x0),
    ("BSD-2-Clause-Patent", r#"BSD-2-Clause Plus Patent License"#, IS_OSI_APPROVED),
    ("BSD-3-Clause", r#"BSD 3-Clause "New" or "Revised" License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("BSD-3-Clause-Attribution", r#"BSD with attribution"#, 0x0),
    ("BSD-3-Clause-Clear", r#"BSD 3-Clause Clear License"#, IS_FSF_LIBRE),
    ("BSD-3-Clause-LBNL", r#"Lawrence Berkeley National Labs BSD variant license"#, IS_OSI_APPROVED),
    ("BSD-3-Clause-No-Nuclear-License", r#"BSD 3-Clause No Nuclear License"#, 0x0),
    ("BSD-3-Clause-No-Nuclear-License-2014", r#"BSD 3-Clause No Nuclear License 2014"#, 0x0),
    ("BSD-3-Clause-No-Nuclear-Warranty", r#"BSD 3-Clause No Nuclear Warranty"#, 0x0),
    ("BSD-3-Clause-Open-MPI", r#"BSD 3-Clause Open MPI variant"#, 0x0),
    ("BSD-4-Clause", r#"BSD 4-Clause "Original" or "Old" License"#, IS_FSF_LIBRE),
    ("BSD-4-Clause-UC", r#"BSD-4-Clause (University of California-Specific)"#, 0x0),
    ("BSD-Protection", r#"BSD Protection License"#, IS_COPYLEFT),
    ("BSD-Source-Code", r#"BSD Source Code Attribution"#, 0x0),
    ("BSL-1.0", r#"Boost Software License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Bahyph", r#"Bahyph License"#, 0x0),
    ("Barr", r#"Barr License"#, 0x0),
    ("Beerware", r#"Beerware License"#, 0x0),
    ("BitTorrent-1.0", r#"BitTorrent Open Source License v1.0"#, 0x0),
    ("BitTorrent-1.1", r#"BitTorrent Open Source License v1.1"#, IS_FSF_LIBRE),
    ("BlueOak-1.0.0", r#"Blue Oak Model License 1.0.0"#, 0x0),
    ("Borceux", r#"Borceux license"#, 0x0),
    ("CATOSL-1.1", r#"Computer Associates Trusted Open Source License 1.1"#, IS_OSI_APPROVED),
    ("CC-BY-1.0", r#"Creative Commons Attribution 1.0 Generic"#, 0x0),
    ("CC-BY-2.0", r#"Creative Commons Attribution 2.0 Generic"#, 0x0),
    ("CC-BY-2.5", r#"Creative Commons Attribution 2.5 Generic"#, 0x0),
    ("CC-BY-3.0", r#"Creative Commons Attribution 3.0 Unported"#, 0x0),
    ("CC-BY-4.0", r#"Creative Commons Attribution 4.0 International"#, IS_FSF_LIBRE),
    ("CC-BY-NC-1.0", r#"Creative Commons Attribution Non Commercial 1.0 Generic"#, 0x0),
    ("CC-BY-NC-2.0", r#"Creative Commons Attribution Non Commercial 2.0 Generic"#, 0x0),
    ("CC-BY-NC-2.5", r#"Creative Commons Attribution Non Commercial 2.5 Generic"#, 0x0),
    ("CC-BY-NC-3.0", r#"Creative Commons Attribution Non Commercial 3.0 Unported"#, 0x0),
    ("CC-BY-NC-4.0", r#"Creative Commons Attribution Non Commercial 4.0 International"#, 0x0),
    ("CC-BY-NC-ND-1.0", r#"Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic"#, 0x0),
    ("CC-BY-NC-ND-2.0", r#"Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic"#, 0x0),
    ("CC-BY-NC-ND-2.5", r#"Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic"#, 0x0),
    ("CC-BY-NC-ND-3.0", r#"Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported"#, 0x0),
    ("CC-BY-NC-ND-4.0", r#"Creative Commons Attribution Non Commercial No Derivatives 4.0 International"#, 0x0),
    ("CC-BY-NC-SA-1.0", r#"Creative Commons Attribution Non Commercial Share Alike 1.0 Generic"#, IS_COPYLEFT),
    ("CC-BY-NC-SA-2.0", r#"Creative Commons Attribution Non Commercial Share Alike 2.0 Generic"#, IS_COPYLEFT),
    ("CC-BY-NC-SA-2.5", r#"Creative Commons Attribution Non Commercial Share Alike 2.5 Generic"#, IS_COPYLEFT),
    ("CC-BY-NC-SA-3.0", r#"Creative Commons Attribution Non Commercial Share Alike 3.0 Unported"#, IS_COPYLEFT),
    ("CC-BY-NC-SA-4.0", r#"Creative Commons Attribution Non Commercial Share Alike 4.0 International"#, IS_COPYLEFT),
    ("CC-BY-ND-1.0", r#"Creative Commons Attribution No Derivatives 1.0 Generic"#, 0x0),
    ("CC-BY-ND-2.0", r#"Creative Commons Attribution No Derivatives 2.0 Generic"#, 0x0),
    ("CC-BY-ND-2.5", r#"Creative Commons Attribution No Derivatives 2.5 Generic"#, 0x0),
    ("CC-BY-ND-3.0", r#"Creative Commons Attribution No Derivatives 3.0 Unported"#, 0x0),
    ("CC-BY-ND-4.0", r#"Creative Commons Attribution No Derivatives 4.0 International"#, 0x0),
    ("CC-BY-SA-1.0", r#"Creative Commons Attribution Share Alike 1.0 Generic"#, IS_COPYLEFT),
    ("CC-BY-SA-2.0", r#"Creative Commons Attribution Share Alike 2.0 Generic"#, IS_COPYLEFT),
    ("CC-BY-SA-2.5", r#"Creative Commons Attribution Share Alike 2.5 Generic"#, IS_COPYLEFT),
    ("CC-BY-SA-3.0", r#"Creative Commons Attribution Share Alike 3.0 Unported"#, IS_COPYLEFT),
    ("CC-BY-SA-4.0", r#"Creative Commons Attribution Share Alike 4.0 International"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("CC-PDDC", r#"Creative Commons Public Domain Dedication and Certification"#, 0x0),
    ("CC0-1.0", r#"Creative Commons Zero v1.0 Universal"#, IS_FSF_LIBRE),
    ("CDDL-1.0", r#"Common Development and Distribution License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("CDDL-1.1", r#"Common Development and Distribution License 1.1"#, IS_COPYLEFT),
    ("CDLA-Permissive-1.0", r#"Community Data License Agreement Permissive 1.0"#, 0x0),
    ("CDLA-Sharing-1.0", r#"Community Data License Agreement Sharing 1.0"#, 0x0),
    ("CECILL-1.0", r#"CeCILL Free Software License Agreement v1.0"#, IS_COPYLEFT),
    ("CECILL-1.1", r#"CeCILL Free Software License Agreement v1.1"#, IS_COPYLEFT),
    ("CECILL-2.0", r#"CeCILL Free Software License Agreement v2.0"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("CECILL-2.1", r#"CeCILL Free Software License Agreement v2.1"#, IS_OSI_APPROVED | IS_COPYLEFT),
    ("CECILL-B", r#"CeCILL-B Free Software License Agreement"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("CECILL-C", r#"CeCILL-C Free Software License Agreement"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("CERN-OHL-1.1", r#"CERN Open Hardware License v1.1"#, 0x0),
    ("CERN-OHL-1.2", r#"CERN Open Hardware Licence v1.2"#, 0x0),
    ("CNRI-Jython", r#"CNRI Jython License"#, 0x0),
    ("CNRI-Python", r#"CNRI Python License"#, IS_OSI_APPROVED),
    ("CNRI-Python-GPL-Compatible", r#"CNRI Python Open Source GPL Compatible License Agreement"#, IS_COPYLEFT),
    ("CPAL-1.0", r#"Common Public Attribution License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("CPL-1.0", r#"Common Public License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("CPOL-1.02", r#"Code Project Open License 1.02"#, 0x0),
    ("CUA-OPL-1.0", r#"CUA Office Public License v1.0"#, IS_OSI_APPROVED),
    ("Caldera", r#"Caldera License"#, 0x0),
    ("ClArtistic", r#"Clarified Artistic License"#, IS_FSF_LIBRE),
    ("Condor-1.1", r#"Condor Public License v1.1"#, IS_FSF_LIBRE),
    ("Crossword", r#"Crossword License"#, 0x0),
    ("CrystalStacker", r#"CrystalStacker License"#, 0x0),
    ("Cube", r#"Cube License"#, 0x0),
    ("D-FSL-1.0", r#"Deutsche Freie Software Lizenz"#, 0x0),
    ("DOC", r#"DOC License"#, 0x0),
    ("DSDP", r#"DSDP License"#, 0x0),
    ("Dotseqn", r#"Dotseqn License"#, 0x0),
    ("ECL-1.0", r#"Educational Community License v1.0"#, IS_OSI_APPROVED),
    ("ECL-2.0", r#"Educational Community License v2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("EFL-1.0", r#"Eiffel Forum License v1.0"#, IS_OSI_APPROVED),
    ("EFL-2.0", r#"Eiffel Forum License v2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("EPL-1.0", r#"Eclipse Public License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("EPL-2.0", r#"Eclipse Public License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("EUDatagrid", r#"EU DataGrid Software License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("EUPL-1.0", r#"European Union Public License 1.0"#, IS_COPYLEFT),
    ("EUPL-1.1", r#"European Union Public License 1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("EUPL-1.2", r#"European Union Public License 1.2"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("Entessa", r#"Entessa Public License v1.0"#, IS_OSI_APPROVED),
    ("ErlPL-1.1", r#"Erlang Public License v1.1"#, 0x0),
    ("Eurosym", r#"Eurosym License"#, 0x0),
    ("FSFAP", r#"FSF All Permissive License"#, IS_FSF_LIBRE),
    ("FSFUL", r#"FSF Unlimited License"#, 0x0),
    ("FSFULLR", r#"FSF Unlimited License (with License Retention)"#, 0x0),
    ("FTL", r#"Freetype Project License"#, IS_FSF_LIBRE),
    ("Fair", r#"Fair License"#, IS_OSI_APPROVED),
    ("Frameworx-1.0", r#"Frameworx Open License 1.0"#, IS_OSI_APPROVED),
    ("FreeImage", r#"FreeImage Public License v1.0"#, 0x0),
    ("GFDL-1.1", r#"GNU Free Documentation License v1.1"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("GFDL-1.1-only", r#"GNU Free Documentation License v1.1 only"#, IS_FSF_LIBRE),
    ("GFDL-1.1-or-later", r#"GNU Free Documentation License v1.1 or later"#, IS_FSF_LIBRE),
    ("GFDL-1.2", r#"GNU Free Documentation License v1.2"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("GFDL-1.2-only", r#"GNU Free Documentation License v1.2 only"#, IS_FSF_LIBRE),
    ("GFDL-1.2-or-later", r#"GNU Free Documentation License v1.2 or later"#, IS_FSF_LIBRE),
    ("GFDL-1.3", r#"GNU Free Documentation License v1.3"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("GFDL-1.3-only", r#"GNU Free Documentation License v1.3 only"#, IS_FSF_LIBRE),
    ("GFDL-1.3-or-later", r#"GNU Free Documentation License v1.3 or later"#, IS_FSF_LIBRE),
    ("GL2PS", r#"GL2PS License"#, 0x0),
    ("GPL-1.0", r#"GNU General Public License v1.0 only"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-1.0+", r#"GNU General Public License v1.0 or later"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-1.0-only", r#"GNU General Public License v1.0 only"#, IS_COPYLEFT),
    ("GPL-1.0-or-later", r#"GNU General Public License v1.0 or later"#, IS_COPYLEFT),
    ("GPL-2.0", r#"GNU General Public License v2.0 only"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-2.0+", r#"GNU General Public License v2.0 or later"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-2.0-only", r#"GNU General Public License v2.0 only"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-2.0-or-later", r#"GNU General Public License v2.0 or later"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-2.0-with-GCC-exception", r#"GNU General Public License v2.0 w/GCC Runtime Library exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-2.0-with-autoconf-exception", r#"GNU General Public License v2.0 w/Autoconf exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-2.0-with-bison-exception", r#"GNU General Public License v2.0 w/Bison exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-2.0-with-classpath-exception", r#"GNU General Public License v2.0 w/Classpath exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-2.0-with-font-exception", r#"GNU General Public License v2.0 w/Font exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("GPL-3.0", r#"GNU General Public License v3.0 only"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-3.0+", r#"GNU General Public License v3.0 or later"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-3.0-only", r#"GNU General Public License v3.0 only"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-3.0-or-later", r#"GNU General Public License v3.0 or later"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("GPL-3.0-with-GCC-exception", r#"GNU General Public License v3.0 w/GCC Runtime Library exception"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_COPYLEFT),
    ("GPL-3.0-with-autoconf-exception", r#"GNU General Public License v3.0 w/Autoconf exception"#, IS_DEPRECATED | IS_COPYLEFT),
    ("Giftware", r#"Giftware License"#, 0x0),
    ("Glide", r#"3dfx Glide License"#, 0x0),
    ("Glulxe", r#"Glulxe License"#, 0x0),
    ("HPND", r#"Historical Permission Notice and Disclaimer"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("HPND-sell-variant", r#"Historical Permission Notice and Disclaimer - sell variant"#, 0x0),
    ("HaskellReport", r#"Haskell Language Report License"#, 0x0),
    ("IBM-pibs", r#"IBM PowerPC Initialization and Boot Software"#, 0x0),
    ("ICU", r#"ICU License"#, 0x0),
    ("IJG", r#"Independent JPEG Group License"#, IS_FSF_LIBRE),
    ("IPA", r#"IPA Font License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("IPL-1.0", r#"IBM Public License v1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("ISC", r#"ISC License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("ImageMagick", r#"ImageMagick License"#, 0x0),
    ("Imlib2", r#"Imlib2 License"#, IS_FSF_LIBRE),
    ("Info-ZIP", r#"Info-ZIP License"#, 0x0),
    ("Intel", r#"Intel Open Source License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Intel-ACPI", r#"Intel ACPI Software License Agreement"#, 0x0),
    ("Interbase-1.0", r#"Interbase Public License v1.0"#, 0x0),
    ("JPNIC", r#"Japan Network Information Center License"#, 0x0),
    ("JSON", r#"JSON License"#, 0x0),
    ("JasPer-2.0", r#"JasPer License"#, 0x0),
    ("LAL-1.2", r#"Licence Art Libre 1.2"#, 0x0),
    ("LAL-1.3", r#"Licence Art Libre 1.3"#, 0x0),
    ("LGPL-2.0", r#"GNU Library General Public License v2 only"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_COPYLEFT),
    ("LGPL-2.0+", r#"GNU Library General Public License v2 or later"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_COPYLEFT),
    ("LGPL-2.0-only", r#"GNU Library General Public License v2 only"#, IS_OSI_APPROVED | IS_COPYLEFT),
    ("LGPL-2.0-or-later", r#"GNU Library General Public License v2 or later"#, IS_OSI_APPROVED | IS_COPYLEFT),
    ("LGPL-2.1", r#"GNU Lesser General Public License v2.1 only"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-2.1+", r#"GNU Library General Public License v2.1 or later"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-2.1-only", r#"GNU Lesser General Public License v2.1 only"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-2.1-or-later", r#"GNU Lesser General Public License v2.1 or later"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-3.0", r#"GNU Lesser General Public License v3.0 only"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-3.0+", r#"GNU Lesser General Public License v3.0 or later"#, IS_DEPRECATED | IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-3.0-only", r#"GNU Lesser General Public License v3.0 only"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPL-3.0-or-later", r#"GNU Lesser General Public License v3.0 or later"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("LGPLLR", r#"Lesser General Public License For Linguistic Resources"#, 0x0),
    ("LPL-1.0", r#"Lucent Public License Version 1.0"#, IS_OSI_APPROVED),
    ("LPL-1.02", r#"Lucent Public License v1.02"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("LPPL-1.0", r#"LaTeX Project Public License v1.0"#, 0x0),
    ("LPPL-1.1", r#"LaTeX Project Public License v1.1"#, 0x0),
    ("LPPL-1.2", r#"LaTeX Project Public License v1.2"#, IS_FSF_LIBRE),
    ("LPPL-1.3a", r#"LaTeX Project Public License v1.3a"#, IS_FSF_LIBRE),
    ("LPPL-1.3c", r#"LaTeX Project Public License v1.3c"#, IS_OSI_APPROVED),
    ("Latex2e", r#"Latex2e License"#, 0x0),
    ("Leptonica", r#"Leptonica License"#, 0x0),
    ("LiLiQ-P-1.1", r#"Licence Libre du Québec – Permissive version 1.1"#, IS_OSI_APPROVED),
    ("LiLiQ-R-1.1", r#"Licence Libre du Québec – Réciprocité version 1.1"#, IS_OSI_APPROVED),
    ("LiLiQ-Rplus-1.1", r#"Licence Libre du Québec – Réciprocité forte version 1.1"#, IS_OSI_APPROVED),
    ("Libpng", r#"libpng License"#, 0x0),
    ("Linux-OpenIB", r#"Linux Kernel Variant of OpenIB.org license"#, 0x0),
    ("MIT", r#"MIT License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("MIT-0", r#"MIT No Attribution"#, IS_OSI_APPROVED),
    ("MIT-CMU", r#"CMU License"#, 0x0),
    ("MIT-advertising", r#"Enlightenment License (e16)"#, 0x0),
    ("MIT-enna", r#"enna License"#, 0x0),
    ("MIT-feh", r#"feh License"#, 0x0),
    ("MITNFA", r#"MIT +no-false-attribs license"#, 0x0),
    ("MPL-1.0", r#"Mozilla Public License 1.0"#, IS_OSI_APPROVED | IS_COPYLEFT),
    ("MPL-1.1", r#"Mozilla Public License 1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("MPL-2.0", r#"Mozilla Public License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("MPL-2.0-no-copyleft-exception", r#"Mozilla Public License 2.0 (no copyleft exception)"#, IS_OSI_APPROVED | IS_COPYLEFT),
    ("MS-PL", r#"Microsoft Public License"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("MS-RL", r#"Microsoft Reciprocal License"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("MTLL", r#"Matrix Template Library License"#, 0x0),
    ("MakeIndex", r#"MakeIndex License"#, 0x0),
    ("MirOS", r#"MirOS License"#, IS_OSI_APPROVED),
    ("Motosoto", r#"Motosoto License"#, IS_OSI_APPROVED),
    ("Multics", r#"Multics License"#, IS_OSI_APPROVED),
    ("Mup", r#"Mup License"#, 0x0),
    ("NASA-1.3", r#"NASA Open Source Agreement 1.3"#, IS_OSI_APPROVED),
    ("NBPL-1.0", r#"Net Boolean Public License v1"#, 0x0),
    ("NCSA", r#"University of Illinois/NCSA Open Source License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("NGPL", r#"Nethack General Public License"#, IS_OSI_APPROVED),
    ("NLOD-1.0", r#"Norwegian Licence for Open Government Data"#, 0x0),
    ("NLPL", r#"No Limit Public License"#, 0x0),
    ("NOSL", r#"Netizen Open Source License"#, IS_FSF_LIBRE),
    ("NPL-1.0", r#"Netscape Public License v1.0"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("NPL-1.1", r#"Netscape Public License v1.1"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("NPOSL-3.0", r#"Non-Profit Open Software License 3.0"#, IS_OSI_APPROVED),
    ("NRL", r#"NRL License"#, 0x0),
    ("NTP", r#"NTP License"#, IS_OSI_APPROVED),
    ("Naumen", r#"Naumen Public License"#, IS_OSI_APPROVED),
    ("Net-SNMP", r#"Net-SNMP License"#, 0x0),
    ("NetCDF", r#"NetCDF license"#, 0x0),
    ("Newsletr", r#"Newsletr License"#, 0x0),
    ("Nokia", r#"Nokia Open Source License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Noweb", r#"Noweb License"#, 0x0),
    ("Nunit", r#"Nunit License"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("OCCT-PL", r#"Open CASCADE Technology Public License"#, 0x0),
    ("OCLC-2.0", r#"OCLC Research Public License 2.0"#, IS_OSI_APPROVED),
    ("ODC-By-1.0", r#"Open Data Commons Attribution License v1.0"#, 0x0),
    ("ODbL-1.0", r#"ODC Open Database License v1.0"#, IS_FSF_LIBRE),
    ("OFL-1.0", r#"SIL Open Font License 1.0"#, IS_FSF_LIBRE),
    ("OFL-1.1", r#"SIL Open Font License 1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("OGL-UK-1.0", r#"Open Government Licence v1.0"#, 0x0),
    ("OGL-UK-2.0", r#"Open Government Licence v2.0"#, 0x0),
    ("OGL-UK-3.0", r#"Open Government Licence v3.0"#, 0x0),
    ("OGTSL", r#"Open Group Test Suite License"#, IS_OSI_APPROVED),
    ("OLDAP-1.1", r#"Open LDAP Public License v1.1"#, 0x0),
    ("OLDAP-1.2", r#"Open LDAP Public License v1.2"#, 0x0),
    ("OLDAP-1.3", r#"Open LDAP Public License v1.3"#, 0x0),
    ("OLDAP-1.4", r#"Open LDAP Public License v1.4"#, 0x0),
    ("OLDAP-2.0", r#"Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)"#, 0x0),
    ("OLDAP-2.0.1", r#"Open LDAP Public License v2.0.1"#, 0x0),
    ("OLDAP-2.1", r#"Open LDAP Public License v2.1"#, 0x0),
    ("OLDAP-2.2", r#"Open LDAP Public License v2.2"#, 0x0),
    ("OLDAP-2.2.1", r#"Open LDAP Public License v2.2.1"#, 0x0),
    ("OLDAP-2.2.2", r#"Open LDAP Public License 2.2.2"#, 0x0),
    ("OLDAP-2.3", r#"Open LDAP Public License v2.3"#, IS_FSF_LIBRE),
    ("OLDAP-2.4", r#"Open LDAP Public License v2.4"#, 0x0),
    ("OLDAP-2.5", r#"Open LDAP Public License v2.5"#, 0x0),
    ("OLDAP-2.6", r#"Open LDAP Public License v2.6"#, 0x0),
    ("OLDAP-2.7", r#"Open LDAP Public License v2.7"#, IS_FSF_LIBRE),
    ("OLDAP-2.8", r#"Open LDAP Public License v2.8"#, 0x0),
    ("OML", r#"Open Market License"#, 0x0),
    ("OPL-1.0", r#"Open Public License v1.0"#, 0x0),
    ("OSET-PL-2.1", r#"OSET Public License version 2.1"#, IS_OSI_APPROVED),
    ("OSL-1.0", r#"Open Software License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("OSL-1.1", r#"Open Software License 1.1"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("OSL-2.0", r#"Open Software License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("OSL-2.1", r#"Open Software License 2.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("OSL-3.0", r#"Open Software License 3.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("OpenSSL", r#"OpenSSL License"#, IS_FSF_LIBRE),
    ("PDDL-1.0", r#"ODC Public Domain Dedication & License 1.0"#, 0x0),
    ("PHP-3.0", r#"PHP License v3.0"#, IS_OSI_APPROVED),
    ("PHP-3.01", r#"PHP License v3.01"#, IS_FSF_LIBRE),
    ("Parity-6.0.0", r#"The Parity Public License 6.0.0"#, IS_COPYLEFT),
    ("Plexus", r#"Plexus Classworlds License"#, 0x0),
    ("PostgreSQL", r#"PostgreSQL License"#, IS_OSI_APPROVED),
    ("Python-2.0", r#"Python License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("QPL-1.0", r#"Q Public License 1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Qhull", r#"Qhull License"#, 0x0),
    ("RHeCos-1.1", r#"Red Hat eCos Public License v1.1"#, 0x0),
    ("RPL-1.1", r#"Reciprocal Public License 1.1"#, IS_OSI_APPROVED),
    ("RPL-1.5", r#"Reciprocal Public License 1.5"#, IS_OSI_APPROVED),
    ("RPSL-1.0", r#"RealNetworks Public Source License v1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("RSA-MD", r#"RSA Message-Digest License "#, 0x0),
    ("RSCPL", r#"Ricoh Source Code Public License"#, IS_OSI_APPROVED),
    ("Rdisc", r#"Rdisc License"#, 0x0),
    ("Ruby", r#"Ruby License"#, IS_FSF_LIBRE),
    ("SAX-PD", r#"Sax Public Domain Notice"#, 0x0),
    ("SCEA", r#"SCEA Shared Source License"#, 0x0),
    ("SGI-B-1.0", r#"SGI Free Software License B v1.0"#, 0x0),
    ("SGI-B-1.1", r#"SGI Free Software License B v1.1"#, 0x0),
    ("SGI-B-2.0", r#"SGI Free Software License B v2.0"#, IS_FSF_LIBRE),
    ("SHL-0.5", r#"Solderpad Hardware License v0.5"#, 0x0),
    ("SHL-0.51", r#"Solderpad Hardware License, Version 0.51"#, 0x0),
    ("SISSL", r#"Sun Industry Standards Source License v1.1"#, IS_OSI_APPROVED | IS_FSF_LIBRE | IS_COPYLEFT),
    ("SISSL-1.2", r#"Sun Industry Standards Source License v1.2"#, 0x0),
    ("SMLNJ", r#"Standard ML of New Jersey License"#, IS_FSF_LIBRE),
    ("SMPPL", r#"Secure Messaging Protocol Public License"#, 0x0),
    ("SNIA", r#"SNIA Public License 1.1"#, 0x0),
    ("SPL-1.0", r#"Sun Public License v1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("SSPL-1.0", r#"Server Side Public License, v 1"#, 0x0),
    ("SWL", r#"Scheme Widget Library (SWL) Software License Agreement"#, 0x0),
    ("Saxpath", r#"Saxpath License"#, 0x0),
    ("Sendmail", r#"Sendmail License"#, 0x0),
    ("Sendmail-8.23", r#"Sendmail License 8.23"#, 0x0),
    ("SimPL-2.0", r#"Simple Public License 2.0"#, IS_OSI_APPROVED),
    ("Sleepycat", r#"Sleepycat License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Spencer-86", r#"Spencer License 86"#, 0x0),
    ("Spencer-94", r#"Spencer License 94"#, 0x0),
    ("Spencer-99", r#"Spencer License 99"#, 0x0),
    ("StandardML-NJ", r#"Standard ML of New Jersey License"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("SugarCRM-1.1.3", r#"SugarCRM Public License v1.1.3"#, 0x0),
    ("TAPR-OHL-1.0", r#"TAPR Open Hardware License v1.0"#, 0x0),
    ("TCL", r#"TCL/TK License"#, 0x0),
    ("TCP-wrappers", r#"TCP Wrappers License"#, 0x0),
    ("TMate", r#"TMate Open Source License"#, 0x0),
    ("TORQUE-1.1", r#"TORQUE v2.5+ Software License v1.1"#, 0x0),
    ("TOSL", r#"Trusster Open Source License"#, 0x0),
    ("TU-Berlin-1.0", r#"Technische Universitaet Berlin License 1.0"#, 0x0),
    ("TU-Berlin-2.0", r#"Technische Universitaet Berlin License 2.0"#, 0x0),
    ("UPL-1.0", r#"Universal Permissive License v1.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("Unicode-DFS-2015", r#"Unicode License Agreement - Data Files and Software (2015)"#, 0x0),
    ("Unicode-DFS-2016", r#"Unicode License Agreement - Data Files and Software (2016)"#, 0x0),
    ("Unicode-TOU", r#"Unicode Terms of Use"#, 0x0),
    ("Unlicense", r#"The Unlicense"#, IS_FSF_LIBRE),
    ("VOSTROM", r#"VOSTROM Public License for Open Source"#, 0x0),
    ("VSL-1.0", r#"Vovida Software License v1.0"#, IS_OSI_APPROVED),
    ("Vim", r#"Vim License"#, IS_FSF_LIBRE),
    ("W3C", r#"W3C Software Notice and License (2002-12-31)"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("W3C-19980720", r#"W3C Software Notice and License (1998-07-20)"#, 0x0),
    ("W3C-20150513", r#"W3C Software Notice and Document License (2015-05-13)"#, 0x0),
    ("WTFPL", r#"Do What The F*ck You Want To Public License"#, IS_FSF_LIBRE),
    ("Watcom-1.0", r#"Sybase Open Watcom Public License 1.0"#, IS_OSI_APPROVED),
    ("Wsuipa", r#"Wsuipa License"#, 0x0),
    ("X11", r#"X11 License"#, IS_FSF_LIBRE),
    ("XFree86-1.1", r#"XFree86 License 1.1"#, IS_FSF_LIBRE),
    ("XSkat", r#"XSkat License"#, 0x0),
    ("Xerox", r#"Xerox License"#, 0x0),
    ("Xnet", r#"X.Net License"#, IS_OSI_APPROVED),
    ("YPL-1.0", r#"Yahoo! Public License v1.0"#, 0x0),
    ("YPL-1.1", r#"Yahoo! Public License v1.1"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("ZPL-1.1", r#"Zope Public License 1.1"#, 0x0),
    ("ZPL-2.0", r#"Zope Public License 2.0"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("ZPL-2.1", r#"Zope Public License 2.1"#, IS_FSF_LIBRE),
    ("Zed", r#"Zed License"#, 0x0),
    ("Zend-2.0", r#"Zend License v2.0"#, IS_FSF_LIBRE),
    ("Zimbra-1.3", r#"Zimbra Public License v1.3"#, IS_FSF_LIBRE),
    ("Zimbra-1.4", r#"Zimbra Public License v1.4"#, 0x0),
    ("Zlib", r#"zlib License"#, IS_OSI_APPROVED | IS_FSF_LIBRE),
    ("blessing", r#"SQLite Blessing"#, 0x0),
    ("bzip2-1.0.5", r#"bzip2 and libbzip2 License v1.0.5"#, 0x0),
    ("bzip2-1.0.6", r#"bzip2 and libbzip2 License v1.0.6"#, 0x0),
    ("copyleft-next-0.3.0", r#"copyleft-next 0.3.0"#, 0x0),
    ("copyleft-next-0.3.1", r#"copyleft-next 0.3.1"#, 0x0),
    ("curl", r#"curl License"#, 0x0),
    ("diffmark", r#"diffmark license"#, 0x0),
    ("dvipdfm", r#"dvipdfm License"#, 0x0),
    ("eCos-2.0", r#"eCos license version 2.0"#, IS_DEPRECATED | IS_FSF_LIBRE),
    ("eGenix", r#"eGenix.com Public License 1.1.0"#, 0x0),
    ("gSOAP-1.3b", r#"gSOAP Public License v1.3b"#, 0x0),
    ("gnuplot", r#"gnuplot License"#, IS_FSF_LIBRE),
    ("iMatix", r#"iMatix Standard Function Library Agreement"#, IS_FSF_LIBRE),
    ("libpng-2.0", r#"PNG Reference Library version 2"#, 0x0),
    ("libtiff", r#"libtiff License"#, 0x0),
    ("mpich2", r#"mpich2 License"#, 0x0),
    ("psfrag", r#"psfrag License"#, 0x0),
    ("psutils", r#"psutils License"#, 0x0),
    ("wxWindows", r#"wxWindows Library License"#, IS_DEPRECATED),
    ("xinetd", r#"xinetd License"#, IS_FSF_LIBRE | IS_COPYLEFT),
    ("xpp", r#"XPP License"#, 0x0),
    ("zlib-acknowledgement", r#"zlib/libpng License with Acknowledgement"#, 0x0),
];

pub const EXCEPTIONS: &[(&str, u8)] = &[
    ("389-exception", 0),
    ("Autoconf-exception-2.0", 0),
    ("Autoconf-exception-3.0", 0),
    ("Bison-exception-2.2", 0),
    ("Bootloader-exception", 0),
    ("CLISP-exception-2.0", 0),
    ("Classpath-exception-2.0", 0),
    ("DigiRule-FOSS-exception", 0),
    ("FLTK-exception", 0),
    ("Fawkes-Runtime-exception", 0),
    ("Font-exception-2.0", 0),
    ("GCC-exception-2.0", 0),
    ("GCC-exception-3.1", 0),
    ("GPL-CC-1.0", 0),
    ("LLVM-exception", 0),
    ("LZMA-exception", 0),
    ("Libtool-exception", 0),
    ("Linux-syscall-note", 0),
    ("Nokia-Qt-exception-1.1", IS_DEPRECATED),
    ("OCCT-exception-1.0", 0),
    ("OCaml-LGPL-linking-exception", 0),
    ("OpenJDK-assembly-exception-1.0", 0),
    ("PS-or-PDF-font-exception-20170817", 0),
    ("Qt-GPL-exception-1.0", 0),
    ("Qt-LGPL-exception-1.1", 0),
    ("Qwt-exception-1.0", 0),
    ("Swift-exception", 0),
    ("Universal-FOSS-exception-1.0", 0),
    ("WxWindows-exception-3.1", 0),
    ("eCos-exception-2.0", 0),
    ("freertos-exception-2.0", 0),
    ("gnu-javamail-exception", 0),
    ("i2p-gpl-java-exception", 0),
    ("mif-exception", 0),
    ("openvpn-openssl-exception", 0),
    ("u-boot-exception-2.0", 0),
];
