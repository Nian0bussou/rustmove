use crate::{movingfn, scrambling, temps_file, utils, CmdsOptions, TimingGuard};
use std::{io, thread};

pub fn threads_sorting(path: String, opt: CmdsOptions) {
    let dirs: Vec<String> = utils::get_folders(&path);

    // + removed dirs
    let mut newdirs: Vec<String>;
    loop {
        newdirs = utils::removedDirs(dirs.clone());
        println!("Are the dirs correct (Y/n)");
        let mut str = String::new();
        _ = io::stdin().read_line(&mut str);
        let str = str.trim();
        if str != "n" {
            break;
        }
    }
    let dirs = newdirs;
    // -

    let _t = TimingGuard::new();

    use CmdsOptions::Move;
    use CmdsOptions::Scramble;

    // + threads -->
    match opt {
        Move => {
            let handles: Vec<_> = dirs
                .clone()
                .into_iter()
                .map(|source| thread::spawn(move || movingfn::move_stuff(source)))
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }
        }
        Scramble => {
            let handles: Vec<_> = dirs
                .clone()
                .into_iter()
                .map(|source| thread::spawn(move || scrambling::scramble(source)))
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }
        }
        _ => (),
    }
    // -
}

pub fn threads_tmps(path: String, printmsg: bool) {
    println!("removing tmps files");

    let vvtmp: Vec<String> = utils::get_folders(&path)
        .iter()
        .map(|t| {
            let s: String = format!("{}/", t);
            s
        })
        .collect::<Vec<String>>()
        .iter()
        .flat_map(|sub| utils::get_folders(sub))
        .collect();

    let _t = TimingGuard::new();
    let handles: Vec<_> = vvtmp
        .into_iter()
        .map(|source: String| thread::spawn(move || temps_file::remove_tmps(&source, printmsg)))
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
