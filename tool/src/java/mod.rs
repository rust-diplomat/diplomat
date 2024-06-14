use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use diplomat_core::hir::TypeContext;

use crate::c2;
use crate::common::FileMap;

const TMP_C_DIR: &str = "tmp";
const LIBRARY: &str = "somelib"; // todo: build from conf. Ensure that name is not the same as any
                                 // type
const GROUP: &str = "dev.diplomattest"; // todo: config
const TMP_LIB_NAME: &str = "dev/diplomattest/somelib"; // todo: build from conf
const JAVA_DIR: &str = "src/main/java/";

pub fn run(
    tcx: &TypeContext,
    _conf_path: Option<&Path>,
    out_folder: &Path,
) -> std::io::Result<FileMap> {
    let files = FileMap::default();
    let mut context = c2::CContext::new(tcx, files, false);
    context.run();

    let errors = context.errors.take_all();

    if !errors.is_empty() {
        eprintln!("Found errors when generating c  code");
        for error in errors {
            eprintln!("\t{}: {}", error.0, error.1);
        }
    }

    let out_files = context.files.take_files();

    let tmp_path = out_folder.join(TMP_C_DIR);
    std::fs::create_dir(&tmp_path)?;
    let mut include_files = HashSet::new();
    for (subpath, text) in out_files {
        let out_path = tmp_path.join(&subpath);
        if !subpath.ends_with(".d.h") && subpath.ends_with(".h") {
            include_files.insert(subpath);
        }
        let parent = out_path
            .parent()
            .expect("Cannot create files at top level dir /");
        std::fs::create_dir_all(parent)?;
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
    }

    let lib_path = tmp_path.join(format!("{LIBRARY}.h"));

    let mut lib_file = File::create(&lib_path)?;
    for include in include_files {
        writeln!(lib_file, "#include \"{include}\"")?;
    }

    // jextract \
    //   --include-dir /path/to/mylib/include \
    //   --output src \
    //   --target-package org.jextract.mylib \
    //   --library mylib \
    //   /path/to/mylib/include/mylib.h

    let package = format!("{GROUP}.{LIBRARY}.ntv");
    let mut command = std::process::Command::new("jextract");
    command
        .arg("--include-dir")
        .arg(&tmp_path)
        .arg("--output")
        .arg(out_folder)
        .arg("--target-package")
        .arg(package)
        .arg("--library")
        .arg(LIBRARY)
        .arg(lib_path);

    println!("Running: {:?}", command);

    // todo: delete directory

    match command.output() {
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Check that jextract is in your path and all directories exist. See https://github.com/openjdk/jextract/blob/5715737be0a1a9de24cce3ee7190881cfc8b1350/doc/GUIDE.md");
                return Err(err);
            }
            _ => return Err(err),
        },
        Ok(ok) => {
            let stdout = String::from_utf8_lossy(&ok.stdout);
            println!("Output from jextract:\n{stdout}");

            let stderr = String::from_utf8_lossy(&ok.stderr);
            println!("Std Err from jextract:\n{stderr}");
        }
    }

    let files = FileMap::default();
    Ok(files)
}
