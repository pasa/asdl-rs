use std::path::Path;
use std::error::Error;
use std::fmt;

use asdl;

use tera::*;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase, MixedCase};

pub type Result<T> = std::result::Result<T, AsdlTeraError>;

#[derive(Debug)]
pub struct AsdlTeraError {
    details: String,
}

impl AsdlTeraError {
    fn new(msg: &str) -> Self {
        AsdlTeraError { details: msg.to_string() }
    }
}

impl fmt::Display for AsdlTeraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AsdlTeraError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<tera::Error> for AsdlTeraError {
    fn from(err: tera::Error) -> Self {
        AsdlTeraError::new(err.description())
    }
}

impl From<asdl::AsdlError> for AsdlTeraError {
    fn from(err: asdl::AsdlError) -> Self {
        AsdlTeraError::new(err.description())
    }
}

impl From<std::io::Error> for AsdlTeraError {
    fn from(err: std::io::Error) -> Self {
        AsdlTeraError::new(err.description())
    }
}

pub fn generate<P: AsRef<Path>>(asdl: &str, templates: &Vec<P>) -> Result<String> {
    let model = asdl::model(asdl)?;
    let mut tera = Tera::default();
    tera.register_filter("camel", |arg, _| Ok(arg.as_str().unwrap().to_camel_case().into()));
    tera.register_filter("snake", |arg, _| Ok(arg.as_str().unwrap().to_snake_case().into()));
    tera.register_filter("mixed", |arg, _| Ok(arg.as_str().unwrap().to_mixed_case().into()));
    tera.register_filter("SCREAM", |arg, _| {
        Ok(arg.as_str().unwrap().to_shouty_snake_case().into())
    });
    for t in templates {
        tera.add_template_file(t, None)?;
    }

    let main_template = templates.last().unwrap().as_ref().to_str().unwrap();
    Ok(tera.render(main_template, &model)?)
}
