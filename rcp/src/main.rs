use clap::Parser;
use nix;
use nix::fcntl::{
    open, 
    OFlag, 
    splice, 
    SpliceFFlags
};
use nix::unistd::pipe;
use nix::sys::stat::{stat, Mode};
use std::path::Path;
use std::thread;

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct Args {
    src: String,
    dst: String,

    /// force copy
    #[arg(short, long)]
    force: bool,

    // just build symbolic link for src
    #[arg(short, long)]
    link: bool,
}

fn rcp(src: &std::path::Path, dst: &std::path::Path, force: bool, link: bool) -> Result<(), std::io::Error> {
    //判断dst是否存在，如果存在询问是否覆盖，如果不存在直接复制
    if dst.exists() && !force {
        println!("{} already exists, do you want to overwrite it? (y/n)", dst.display());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read from stdin");
        if input.trim() == "n" {
            return Ok(());
        }
    } 
    //判断src是否是symbolic link, 如果是，复制symbolic link
    if src.symlink_metadata()?.file_type().is_symlink() {
        let link_path = std::fs::read_link(src)?;
        std::os::unix::fs::symlink(link_path, dst)?;
        return Ok(());
    } else {
        if link {
            std::os::unix::fs::symlink(src, dst)?;
        } else {
            let fstat = stat(src).expect("failed to stat file");

            let (read_pipe, write_pipe) = pipe()?;

            let source_fd = open(src, OFlag::O_RDONLY, Mode::empty())?;
            let dest_fd = open(dst, OFlag::O_WRONLY | OFlag::O_CREAT, Mode::empty())?;

            let mut remain = fstat.st_size;
            while remain > 0 {
                splice(source_fd, None, write_pipe, None, 4096, SpliceFFlags::SPLICE_F_MOVE)?;
                splice(read_pipe, None, dest_fd, None, 4096, SpliceFFlags::SPLICE_F_MOVE)?;
                remain -= 4096;
            }
        }
    }
    Ok(())
}

fn traversal_cp(src: &std::path::Path, dst: &std::path::Path, force: bool, link: bool) -> Result<(), std::io::Error>  {
    // TODO: 判断src是不是文件夹，如果是，判定dst是不是文件，如果不是，判断dst是不是已存在的路径，如果不是，创建dst，将src下的所有文件复制到dst下
    let mut handles = Vec::new();
    if src.is_dir() && !dst.is_file() {
        if !dst.exists() {
            std::fs::create_dir_all(dst)?;
        }
        let entries = std::fs::read_dir(src)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);

            if entry_path.is_dir() {
                std::fs::create_dir_all(&dst_path)?;
                let handle = thread::spawn(move || {
                    let _ = traversal_cp(&entry_path,  &dst_path, force, link);
                });
                handles.push(handle);
                // traversal_cp(&entry_path,  &dst_path)?;
            } else {
                let _ = rcp(&entry_path, &dst_path, force, link);
            }
        }
    } else if src.is_file() {
        if dst.is_dir() {
            // create a null file of the same name as src in dst directory
            let file_name = src.file_name().unwrap();
            let dst_path = dst.join(file_name);
            let _ = rcp(&src, &dst_path, force, link);
        } else if dst.is_file() {
            if !dst.exists() {
                let _ = rcp(&src, &dst, force, link);
            } 
        }
    }
    for handle in handles {
        handle.join().unwrap();
    } 
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut force = false;
    if args.force {
        force = args.force;

    }
    let mut link = false;
    if args.link {
        link = args.link;
    }
    // Open source file and destination file
    let arg1 = args.src;
    let arg2 = args.dst;
    let src = Path::new(&arg1);
    let dst = Path::new(&arg2);

    traversal_cp(src, dst, force, link)?;
    
    Ok(())   // Handle error: source is not a directory
}

