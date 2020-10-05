use crate::*;
use error::SniprunError;
use interpreter::{Interpreter, SupportLevel};

pub struct Launcher {
    pub data: DataHolder,
}

#[allow(unused_assignments)]
impl Launcher {
    pub fn new(data: DataHolder) -> Self {
        Launcher { data }
    }

    pub fn select_and_run<'a>(&self) -> Result<String, SniprunError> {
        let mut max_level_support = SupportLevel::Unsupported;
        let mut name_best_interpreter = String::from("Generic");
        //select the best interpreter for the language
        let mut skip_all = false;
        iter_types! {
            if !skip_all && Current::get_supported_languages().contains(&self.data.filetype){
                if Current::get_max_support_level() > max_level_support {
                    max_level_support = Current::get_max_support_level();
                    name_best_interpreter = Current::get_name();
                }

                if self.data.selected_interpreters.contains(&Current::get_name()){
                    max_level_support = SupportLevel::Selected;
                    name_best_interpreter = Current::get_name();
                    skip_all = true;
                }
            }
        }
        info!(
            "[LAUNCHER] Selected interpreter : {} ; with support level {:?}",
            name_best_interpreter, max_level_support
        );

        //launch !
        iter_types! {
            if Current::get_name() == name_best_interpreter {
                let mut inter = Current::new_with_level(self.data.clone(), max_level_support);
                return inter.run();
            }
        }
        //technically unreachable but generic interpreter may be obsolete at some point
        return Err(SniprunError::NoInterpreterFound);
    }
}
