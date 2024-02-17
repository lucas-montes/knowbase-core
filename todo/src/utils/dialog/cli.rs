use clap::{ValueEnum, Args};

use super::Zenity;


#[derive(Debug, Args, Clone)]
pub struct DialogArgs {
    #[arg(
        short,
        long,
        default_value_t = DialogOptions::DialogMessage,
        default_missing_value = "DialogMessage",
        value_enum, 
        required = false
    )]
    work: DialogOptions,
}

impl DialogArgs {
    pub async fn run(&self) -> i16 {
        match &self.work{
            DialogOptions::DialogQuestion => Zenity::new().show_question("e") ,
            DialogOptions::DialogInput => Zenity::new().show_input("e") ,
            DialogOptions::DialogMessage => Zenity::new().show_message("e") ,
            DialogOptions::DialogPassword => Zenity::new().show_password("e"),
    };
    0
    }
    
}






#[derive(ValueEnum, Clone, Debug)]
pub enum DialogOptions {
DialogInput,
DialogMessage,
DialogPassword,
DialogQuestion,
}