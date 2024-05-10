use std::{
    borrow::Cow,
    error::Error,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use jaffi::Jaffi;

fn class_path() -> PathBuf {
    // TODO: Look in the android.jar
    //PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set")).join("java/classes")
    PathBuf::from("android-src/")
}

fn main() -> Result<(), Box<dyn Error>> {
    // only need this if you need to compile the java, this is needed for the integration tests...
    //compile_java();

    let class_path = class_path();
    let classes = vec![
        //Cow::from("android.annotation.AttrRes"),
    ];
    let classes_to_wrap = vec![
        //Cow::from("android.annotation.AttrRes"),
        //Cow::from("android.widget.EditText"),
        //Cow::from("android.widget.TextView"),
        Cow::from("android.view.View"),
        //Cow::from("android.app.Activity"),
    ];
    let output_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let output_file = Cow::from(Path::new("generated_jaffi.rs"));

    let jaffi = Jaffi::builder()
        .output_dir(&output_dir)
        .output_filename(&output_file)
        .native_classes(classes)
        .classes_to_wrap(classes_to_wrap)
        .classpath(vec![Cow::from(class_path)])
        .build();

    jaffi.generate()?;

    // let's format the file to help with debugging build issues
    let jaffi_file = output_dir.join(output_file);

    let mut cmd = Command::new("rustfmt");
    cmd.arg("--emit").arg("files").arg(jaffi_file);

    eprintln!("cargo fmt: {cmd:?}");
    let output = cmd.output();

    match output {
        Ok(output) => {
            std::io::stderr().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            eprintln!("cargo fmt failed to execute: {e}");
        }
    }

    Ok(())
}
