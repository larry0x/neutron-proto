//! Build Neutron proto files.
//!
//! This build script clones the Neutron version specified in the NEUTRON_REV
//! constant and then uses that to build the required proto files for further
//! compilation.
//!
//! Forked from the cosmos-rust repo, which is in turn based on the work found
//! in: github.com/informalsystems/ibc-rs.

use std::{
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
};

use log::info;
use regex::Regex;
use walkdir::WalkDir;

/// The Neutron commit or tag to be cloned and used to build the proto files
///
/// NOTE: The official Neutron repo doesn't comes with a buf.yaml config file,
/// so it doesn't work with this proto-build script. I had to make a fork off
/// v1.0.2, tagged v1.0.2-buf, which adds the buf config.
const NEUTRON_REV: &str = "v1.0.2-buf";

// All paths must end with a / and either be absolute or include a ./ to
// reference the current working directory.

/// The directory generated Neutron proto files go into
const NEUTRON_PROTO_DIR: &str = "./neutron-proto/src/prost/";

/// Directory where the Neutron submodule is located
const NEUTRON_DIR: &str = "./neutron";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/neutron-proto/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];

fn main() {
    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = NEUTRON_PROTO_DIR.parse().unwrap();

    make_temp_dir(&tmp_build_dir);
    update_submodule();
    output_version(&tmp_build_dir);
    compile_protos_and_services(&tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir.join("neutron"));
    run_rustfmt(&proto_dir);
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(process::Stdio::inherit())
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                panic!("error running '{:?}': command not found. Is it installed?", cmd.as_ref())
            },
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(config: &str, proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            config,
            "--include-imports",
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    );
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn update_submodule() {
    info!("Updating neutron-org/neutron submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", NEUTRON_DIR, "fetch"]);
    run_git(["-C", NEUTRON_DIR, "reset", "--hard", NEUTRON_REV]);
}

fn make_temp_dir(dir: &Path) {
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir_all(&dir).unwrap();
}

fn output_version(out_dir: &Path) {
    let path = out_dir.join("NEUTRON_COMMIT");
    fs::write(path, NEUTRON_REV).unwrap();
}

fn compile_protos_and_services(out_dir: &Path) {
    info!("Compiling neutron .proto files to Rust into '{}'...", out_dir.display());
    let proto_path = Path::new(NEUTRON_DIR).join("proto");
    run_buf("proto-build/buf.gen.yaml", &proto_path, out_dir);
    info!("=> Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();

            // skip files that have nothing to do with neutron
            if !filename.to_lowercase().starts_with("neutron") {
                return Ok(());
            }

            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}
