use crate::{counting::GLOBAL_COUNTS, utils};
use image::image_dimensions;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn move_stuff(dir: String) {
    let dwall = Path::new(&dir).join("wall");
    let dother = Path::new(&dir).join("other");
    let dsquare = Path::new(&dir).join("square");
    let dbadquality = Path::new(&dir).join("bad_quality");
    let dbadqualitylandscape = Path::new(&dbadquality).join("l");
    let dbadqualitysquare = Path::new(&dbadquality).join("s");
    let dbadqualityportrait = Path::new(&dbadquality).join("p");
    let dvideo = Path::new(&dir).join("video");

    let destinations: Vec<&PathBuf> = vec![
        &dwall,
        &dother,
        &dsquare,
        &dbadquality,
        &dbadqualitylandscape,
        &dbadqualitysquare,
        &dbadqualityportrait,
        &dvideo,
    ];
    make_folders(&destinations);

    if let Ok(entries) = fs::read_dir(&dir) {
        for f in entries {
            if let Ok(f) = f {
                let path = f.path();
                if path.is_dir() {
                    continue;
                }
                move_file(path, &destinations, &dir)
            }
        }
    }
    // match fs::read_dir(&dir) {
    //     Ok(entries) => {
    //         for f in entries {
    //             match f {
    //                 Ok(f) => {
    //                     let path = f.path();
    //                     if path.is_dir() {
    //                         continue;
    //                     }
    //                     move_file(path, &destinations, &dir)
    //                 }
    //                 Err(_) => (), // no need to do anything here
    //             }
    //         }
    //     }
    //     Err(_) => (), // no need to do anything here
    // }
}

fn move_file(file: PathBuf, dests: &Vec<&PathBuf>, source: &str) {
    let dwall = dests[0];
    let dother = dests[1];
    let dsquare = dests[2];
    let dblandscape = dests[4];
    let dbsquare = dests[5];
    let dbportrait = dests[6];
    let dvideo = dests[7];

    let extension = file.extension().unwrap();
    if extension == "mp4" {
        wrap_rename(file, dvideo, "yellow", "video", source);
        return;
    }
    let (width, height) = image_dimensions(&file).unwrap();
    let aspect_ratio = width as f32 / height as f32;
    let (dest, color) = if width >= 1080 && height >= 1080 {
        match aspect_ratio {
            ar if ar > 1.0 => (dwall, "red"),
            ar if ar < 1.0 => (dother, "green"),
            _ => (dsquare, "blue"),
        }
    } else {
        match aspect_ratio {
            ar if ar > 1.0 => (dblandscape, "cyan"),
            ar if ar < 1.0 => (dbportrait, "purple"),
            _ => (dbsquare, "magenta"),
        }
    };
    let label = match aspect_ratio {
        ar if ar > 1.0 => "land",
        ar if ar < 1.0 => "portrait",
        _ => "square",
    };
    wrap_rename(file, dest, color, label, source)
}

fn wrap_rename(file_path: PathBuf, destination: &PathBuf, color: &str, format: &str, source: &str) {
    let mut guard = GLOBAL_COUNTS.lock().unwrap();
    match format {
        "land" => guard.landpp(),
        "portrait" => guard.portpp(),
        "square" => guard.squapp(),
        _ => panic!("invalid entry match format"),
    };
    let file_name = file_path.file_name().unwrap();
    let new_file = destination.join(file_name);
    wrap_move(file_path, new_file, color, format, source)
}

fn wrap_move(file_path: PathBuf, new_path: PathBuf, color: &str, format: &str, source: &str) {
    let tmp = new_path.parent().unwrap().to_owned();
    let parent_new = tmp.to_str().unwrap();
    let mut guard = GLOBAL_COUNTS.lock().unwrap();
    guard.propp();
    match fs::rename(file_path, new_path) {
        Ok(_) => {
            guard.sucpp();
            utils::file_output(source, parent_new, color, format)
        }
        Err(_) => {
            guard.faipp();
            utils::error_maxxing();
        }
    }
}

fn make_folders(dests: &Vec<&PathBuf>) {
    for d in dests {
        let mut guard = GLOBAL_COUNTS.lock().unwrap();
        if let Ok(_) = fs::create_dir(d) {
            guard.dir_countpp()
        }
        // match fs::create_dir(d) {
        //     Ok(_) => guard.dir_countpp(),
        //     Err(_) => (),
        // };
    }
}
