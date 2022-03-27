use std::{
    env, fs,
    path::{Path, PathBuf},
};

const COPY_DIR: &'static str = "icons";

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        }
    }
}

fn main() {
    // Request the output directory
    let profile = env::var("PROFILE").unwrap();

    let out = env::current_dir()
        .unwrap()
        .join(PathBuf::from(format!("../target/{}/{}", profile, COPY_DIR)));

    // If it is already in the output directory, delete it and start over
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    // Create the out directory
    fs::create_dir(&out).unwrap();

    // Copy the directory
    copy_dir(COPY_DIR, &out);
}
