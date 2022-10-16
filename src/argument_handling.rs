
pub struct Args {
    pub directory : String,
    pub compiler : String,
    pub name : String,
    pub show_help : bool,
    pub create_project : bool,
    valid : bool
}

impl Args {

    /**
     * If the -h argument is passed then the directory, compiler, and name strings will not be
     * collected and you should NOT operate on them. If the -m argument is passed then the 
     * directory argument will be passed but the compiler and name won't be passed.
    */
    pub fn new(argument_list: &Vec<String>) -> Args{

        let mut new_args = Args  { 
            directory: "".to_string(),
            compiler: "".to_string(),
            name : "".to_string(),
            show_help : false,
            create_project : false,
            valid : true
        };


        if has_argument("-h", argument_list) {
        
            new_args.show_help = true;
            new_args.valid = false;
            return new_args; // -h cancels out everything else.
            
        }

        match find_argument("-d", argument_list) {

            Some(c) => new_args.directory = c,
            None => println!("No directory given!")

        }

        if has_argument("-m", argument_list) {
            new_args.create_project = true;
            new_args.valid = false;
            return new_args; // -m cancels the compilation process and just creates a project file.
        }

        match find_argument("-c", argument_list) {
            Some(c) => new_args.compiler = c,
            None => println!("No compiler given!")
        }

        match find_argument("-n", argument_list) {

            Some(c) => new_args.name = c,
            None => println!("No name given!")

        }

        new_args
    }

    pub fn is_valid(&self) -> bool {

        if !self.valid { 
            return false;
        }

        if !std::path::Path::new(self.directory.as_str()).exists()  {
            return false;
        }

        let compiler_path = std::path::Path::new(self.compiler.as_str());
        if !compiler_path.exists() || !compiler_path.is_file() {
            return false;
        }

        if self.name.is_empty() || self.name.contains(' ') {
            return false;
        }

        true
    }

}

fn find_argument(arg: &str, argument_list: &Vec<String>) -> Option<String>{


    let mut next = false;
    for i in argument_list {
        
        if next {
            return Some(i.clone());
        }

        if *i == arg.to_string() {
            next = true;
        }
        
    }

    return None;

}

fn has_argument(arg: &str, argument_list: &Vec<String>) -> bool{

    for i in argument_list {
        if *i == arg.to_string() {
            return true;
        }
    }

    false

}
