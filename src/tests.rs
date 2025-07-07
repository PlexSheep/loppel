use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

use serial_test::serial;
use tempfile::tempdir;

use crate::{backup_dir, backup_file, make_archive, read_archive, remove_archive_ending, restore};

const CONTENT: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn filesize(p: &Path) -> io::Result<u64> {
    Ok(fs::metadata(p)?.size())
}

#[test]
#[serial]
fn test_make_archive() -> io::Result<()> {
    let t = tempdir()?;
    let tdir = t.path();
    std::env::set_current_dir(tdir).unwrap(); // NOTE: if multiple tests use this, this
                                              // creates a race condition
    let tfile = PathBuf::from("foo");
    let tfile_a = PathBuf::from("foo.tar.zstd");

    fs::write(&tfile, CONTENT).unwrap();
    assert!(tfile.exists());
    assert!(tfile.is_file());
    assert_eq!(fs::read(&tfile).unwrap(), CONTENT);
    let raw_size = fs::metadata(&tfile).unwrap().size();
    assert!(raw_size > 1, "raw size was {raw_size}");

    // NOTE: append_path needs a relative path
    make_archive(&tfile_a, |a| a.append_path(&tfile)).unwrap();
    assert!(tfile_a.exists());
    assert!(tfile_a.is_file());
    let arch_size = fs::metadata(&tfile_a).unwrap().size();
    assert!(arch_size > 1, "archive size was {arch_size}");

    fs::remove_file(&tfile).unwrap();
    assert!(!tfile.exists());

    read_archive(&tfile_a, |a| a.unpack(tdir)).unwrap();
    assert!(tfile.exists());
    assert!(!tfile.is_dir());
    assert!(tfile.is_file());
    let copy_size = fs::metadata(&tfile).unwrap().size();
    assert!(copy_size > 1, "archive size was {arch_size}");

    let copy_content = fs::read(&tfile).unwrap();
    assert_eq!(CONTENT, copy_content);

    Ok(())
}

#[test]
fn test_simple_bak_restore() -> io::Result<()> {
    let t = tempdir()?;
    let tdir = t.path();
    let tfile = tdir.join("foo");
    let tfile_b = tdir.join("foo.bak");

    fs::write(&tfile, CONTENT).unwrap();
    assert!(tfile.exists());
    assert!(tfile.is_file());
    assert_eq!(fs::read(&tfile).unwrap(), CONTENT);
    let raw_size = filesize(&tfile)?;
    assert!(raw_size > 1, "raw size was {raw_size}");

    backup_file(&tfile, false).unwrap();

    assert!(tfile_b.exists());
    assert!(tfile_b.is_file());
    assert_eq!(fs::read(&tfile_b).unwrap(), CONTENT);
    let raw_size = filesize(&tfile)?;
    assert!(raw_size > 1, "raw size was {raw_size}");

    fs::remove_file(&tfile).unwrap();
    assert!(!tfile.exists());

    restore(&tfile_b, tdir).unwrap();

    assert!(tfile.exists());
    assert!(tfile.is_file());
    assert_eq!(fs::read(&tfile).unwrap(), CONTENT);
    let raw_size = filesize(&tfile)?;
    assert!(raw_size > 1, "raw size was {raw_size}");

    Ok(())
}

#[test]
fn test_dir_bak_restore() -> io::Result<()> {
    let t = tempdir()?;
    let tdir = t.path();
    let tdir_a = tdir.join("ichi");
    let tdir_b = tdir_a.join("ni");
    let dirs = [&tdir_a, &tdir_b];
    let names = ["foo", "bar", "qux"];
    fastrand::seed(133719);

    let mut contents: Vec<[u8; 16]> = vec![];
    for _ in 0..(dirs.len() * names.len()) {
        contents.push(fastrand::u128(0..u128::MAX).to_le_bytes());
    }

    let mut i = 0;
    for sdir in dirs {
        fs::create_dir_all(sdir)?;
        assert!(sdir.exists());
        assert!(sdir.is_dir());
        for fname in names {
            let p = sdir.join(fname);
            fs::write(&p, contents[i])?;
            assert!(p.exists());
            assert!(p.is_file());
            assert!(p.is_file());
            let raw_size = filesize(&p)?;
            assert!(raw_size > 1, "raw size of {} was {raw_size}", p.display());
            i += 1;
        }
    }

    let backup = backup_dir(&tdir_a, false)?;
    dbg!(&tdir_a);
    dbg!(fs::metadata(&tdir_a)?);
    fs::remove_dir_all(&tdir_a)?;
    dbg!(&backup);
    dbg!(fs::metadata(&backup)?);
    restore(&backup, tdir)?;
    dbg!(&tdir_a);
    dbg!(fs::metadata(&tdir_a)?);

    let mut i = 0;
    for sdir in dirs {
        assert!(sdir.exists());
        assert!(sdir.is_dir());
        for fname in names {
            let p = sdir.join(fname);
            assert!(p.exists());
            assert!(p.is_file());
            let actual = fs::read(&p)?;
            assert_eq!(actual, contents[i]);
            i += 1;
        }
    }

    Ok(())
}

