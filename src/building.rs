use std::{fs, path::Path, process::Command};
use filetime::FileTime;

use crate::argument_handling;

fn gather_directory(directory: String) -> Vec<String> {

    let read_directory = fs::read_dir(directory).unwrap();
    let mut result : Vec<String> = Vec::new();

    for file in read_directory {
        let file_res = file.unwrap();
        if file_res.path().is_file() {
            result.push(file_res.file_name().to_string_lossy().to_string()); // I'm not sure if this is the best way to convert the path
        }

    }

    result
}

pub fn gather_source_file_names(args: &argument_handling::Args) -> Vec<String> {

    gather_directory(format!("{}/src", args.directory))

}

pub fn gather_compiled_file_names(args: &argument_handling::Args) -> Vec<String> {

    gather_directory(format!("{}/imt", args.directory))

}


pub fn generate_outdated_list(source_files: &Vec<String>, generated_objects: &Vec<String>, args: &argument_handling::Args) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();

    'outer: for source in source_files {

        let file_name = Path::new(source);
        for obj in generated_objects {

            let other_file_name = Path::new(obj);

            if file_name.extension().unwrap() == "cpp" && file_name.with_extension("") == other_file_name.with_extension("") {

                let true_source_path = format!("{}/src/{}", args.directory, file_name.display());
                let true_obj_path = format!("{}/imt/{}", args.directory, other_file_name.display());

                let src_meta = fs::metadata(true_source_path).unwrap(); 
                let obj_meta = fs::metadata(true_obj_path).unwrap(); 
                
                let src_mod_time = FileTime::from_last_modification_time(&src_meta);
                let obj_mod_time = FileTime::from_last_modification_time(&obj_meta);

                if src_mod_time.unix_seconds() > obj_mod_time.unix_seconds() { 
                    result.push(source.clone());
                }

                continue 'outer;
            }

        }
        if Path::new(source).extension().unwrap() == "cpp" {
            result.push(source.clone());
        }

    }

    result

}

pub struct BuildCommand {
    pub exe : String, 
    pub args : String,
}

pub fn generate_build_commands(outdated: &Vec<String>, args: &argument_handling::Args) -> Vec<BuildCommand> {

    let mut result: Vec<BuildCommand> = Vec::new();
    let imt_dir = format!("{}/imt/",args.directory.clone());

    for file in outdated {

        let new_command = BuildCommand {

            exe : args.compiler.clone(),
            args: format!("-c {}/src/{} -o {}{}", 
            args.directory.clone(),
            file.clone(), 
            imt_dir,
            Path::new(file).with_extension("o").display()),

        };
        result.push(new_command);

    }

    result
}

pub fn run_commands(commands: &Vec<BuildCommand>) {

    for command in commands {
        let _ = Command::new(command.exe.clone())
        .args(command.args.split_ascii_whitespace())
        .output()
        .expect("Failure to start build process");
    }

}

pub fn link_command(args: &argument_handling::Args) {

    let directory_reader = fs::read_dir(format!("{}/imt/", args.directory.clone())).unwrap();

    let mut files : Vec<String> = Vec::new();

    files.push("-o".to_string());
    files.push(format!("{}/bld/{}", args.directory, args.name));

    for file_res in directory_reader {

        let file = file_res.unwrap();
        files.push(file.path().display().to_string());

    }

    let _ = Command::new(args.compiler.clone()).args(files).spawn().expect("Failure to issue link command");

}