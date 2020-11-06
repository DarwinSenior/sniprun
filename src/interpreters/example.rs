//Interpreter:| Name                | language    |
//############|_____________________|_____________|________________<- delimiters to help formatting,
//###########| Interpretername      | language    | comment
// Keep (but modify the first line after the :) if you wish to have this interpreter listed via
// SnipList

// Be sure to read the CONTRIBUTING.md file :-)

#[derive(Clone)]
#[allow(non_camel_case_types)]
// For example, Rust_original is a good name for a first rust interpreter
pub struct Language_subname {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,

    ///specific to compiled languages
    language_work_dir: String,
    bin_path: String,
    main_file_path: String,
    // you can and should add field as needed
}

//necessary boilerplate, you don't need to implement that if you want a Bloc support level
//interpreter (the easiest && most common)
impl ReplLikeInterpreter for Language_subname {}

impl Interpreter for Rust_original {
    fn new_with_level(data: DataHolder, support_level: SupportLevel) -> Box<Rust_original> {
        //create a subfolder in the cache folder
        let lwd = data.work_dir.clone() + "/language_subname";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&lwd)
            .expect("Could not create directory for rust-original");

        //pre-create string pointing to main file's and binary's path
        let mfp = lwd.clone() + "/main.extension";
        let bp = lwd.clone() + "/main"; // remove extension so binary is named 'main'
        Box::new(Rust_original {
            data,
            support_level,
            code: String::new(),
            rust_work_dir: lwd,
            bin_path: bp,
            main_file_path: mfp,
        })
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            //':set ft?' in nvim to get the filetype of opened file
            String::from("language_filetype"),
            String::from("extension"), //should not be necessary, but just in case
                                       // another similar name (like python and python3)?
        ]
    }

    fn get_name() -> String {
        // get your interpreter name
        String::from("Language_subname")
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        //define the max level support of the interpreter (see readme for definitions)
        SupportLevel::Bloc
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        //here if you detect conditions that make higher support level impossible,
        //or unecessary, you should set the current level down. Then you will be able to
        //ignore maybe-heavy code that won't be needed anyway

        //add code from data to self.code
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.support_level >= SupportLevel::Bloc
        {
            // if bloc is not pseudo empty and has Bloc current support level
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.support_level >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }

        // now self.code contains the line or bloc of code wanted :-)
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        // an example following Rust's syntax
        self.code = String::from("fn main() {") + &self.code + "}";
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        //write code to file
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for rust-original");
        // IO errors can be ignored, or handled into a proper SniprunError
        // If you panic, it should not be too dangerous for anyone
        write(&self.main_file_path, &self.code).expect("Unable to write to file for rust-original");

        //compile it (to the bin_path that arleady points to the rigth path)
        let output = Command::new("compiler")
            .arg("--optimize") // for short snippets, that may contain a long loop
            .arg("--out-dir")
            .arg(&self.language_work_dir)
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");

        //TODO if relevant, return the error number (parse it from stderr)
        if !output.status.success() {
            return Err(SniprunError::CompilationError("".to_string()));
        } else {
            return Ok(());
        }
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        //run th binary and get the std output (or stderr)
        let output = Command::new(&self.bin_path)
            .output()
            .expect("Unable to start process");

        if output.status.success() {
            //return stdout
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            // return stderr
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}
