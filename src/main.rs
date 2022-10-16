use std::env;
use std::fs;

use building::link_command;
use building::run_commands;

mod argument_handling;
mod building;

fn create_project(dir : &String) {

    let _ = fs::create_dir_all(format!("{}/src", dir)).unwrap();
    let _ = fs::create_dir_all(format!("{}/imt", dir)).unwrap();
    let _ = fs::create_dir_all(format!("{}/bld", dir)).unwrap();

}

fn main() {
    
    let args : Vec<String> = env::args().collect();

    let arguments = argument_handling::Args::new(&args); 

    if arguments.show_help {

        print!("TinyMake Manual
            -d | the project directory [required for all]
            -c | the compiler location [required for compiling]
            -n | the project name [required for compiling]
            -m | create a project directory
            -h | display this menu\n");
        return;
    }

    if arguments.create_project {
        create_project(&arguments.directory);
        return;
    }

    if !arguments.is_valid() {
        return;
    }

    let source_directory = building::gather_source_file_names(&arguments);
    let intermediate_directory = building::gather_compiled_file_names(&arguments);

    let outdated = building::generate_outdated_list(&source_directory, &intermediate_directory, &arguments);

    let build_commands = building::generate_build_commands(&outdated, &arguments);
    
    run_commands(&build_commands);

    link_command(&arguments);

}