use crate::client;
use crate::ssm_change_error::{CalculateSSMChangeErrorKind, SSMChangeError};
use aws_sdk_ssm::model::ParameterType;
use aws_sdk_ssm::output;
use colored::Colorize;
use std::error::Error;
use std::fmt;

/// Describes the action that needs to be taken for an SSMChange.
#[derive(PartialEq, Debug)]
pub enum Action {
    NoAction,
    Update,
    Create,
}

/// Takes a reference to a ParameterType (&T) and returns T
/// Needed for ssm client as it only accepts T.
fn parameter_type_from_ref(p_type: &ParameterType) -> ParameterType {
    match p_type {
        ParameterType::String => return ParameterType::String,
        ParameterType::SecureString => return ParameterType::SecureString,
        ParameterType::StringList => return ParameterType::StringList,
        ParameterType::Unknown(s) => return ParameterType::Unknown(s.to_string()),
        _ => return ParameterType::SecureString,
    };
}

/// An SSMChange represents the changes that need to be made
/// to an SSM Parameter in a region to match the arguments to
/// the program.
#[derive(PartialEq, Debug)]
pub struct SSMChange {
    pub action: Action,
    pub region: String,
    pub parameter: String,
    pub value: String,
    pub type_param: ParameterType,
}

impl SSMChange {
    /// Creates a new SSM Change based on args
    pub fn new(
        action: Action,
        region: &str,
        parameter: &str,
        value: &str,
        type_param: ParameterType,
    ) -> SSMChange {
        SSMChange {
            action,
            region: region.to_owned(),
            parameter: parameter.to_owned(),
            value: value.to_owned(),
            type_param,
        }
    }
}

impl SSMChange {
    /// Creates a new SSMChange based on the result of a
    /// ssm:GetParameter call.
    /// Calculates the changes that need to be made to sync an SSM
    /// parameter across regions.
    pub fn calculate_change(
        result: Result<output::GetParameterOutput, aws_sdk_ssm::Error>,
        region: &str,
        ssm_parameter: &str,
        new_value: &str,
        new_type: &ParameterType,
    ) -> Result<SSMChange, SSMChangeError> {
        match result {
            Ok(param) => {
                if param.parameter().is_none() {
                    return Err(SSMChangeError::new(
                        CalculateSSMChangeErrorKind::NoParameterInResponse,
                    ));
                }

                if param.parameter().unwrap().value().is_none() {
                    return Err(SSMChangeError::new(
                        CalculateSSMChangeErrorKind::NoValueInParameter,
                    ));
                }

                if param.parameter().unwrap().r#type().is_none() {
                    return Err(SSMChangeError::new(
                        CalculateSSMChangeErrorKind::NoValueInParameter,
                    ));
                }

                let value = param.parameter().unwrap().value().unwrap();
                let type_param = param.parameter().unwrap().r#type().unwrap();

                if !value.eq(new_value) {
                    return Ok(SSMChange::new(
                        Action::Update,
                        &region,
                        &ssm_parameter,
                        &new_value,
                        parameter_type_from_ref(type_param),
                    ));
                }

                return Ok(SSMChange::new(
                    Action::NoAction,
                    &region,
                    &ssm_parameter,
                    &new_value,
                    parameter_type_from_ref(type_param),
                ));
            }
            Err(e) => match e {
                aws_sdk_ssm::Error::ParameterNotFound(_) => {
                    return Ok(SSMChange::new(
                        Action::Create,
                        &region,
                        &ssm_parameter,
                        &new_value,
                        parameter_type_from_ref(new_type),
                    ));
                }
                _ => Err(SSMChangeError::new(
                    CalculateSSMChangeErrorKind::UnexpectedError,
                )),
            },
        }
    }
}

impl fmt::Display for SSMChange {
    /// Set the formatting of SSMChange so the objects look nice when
    /// printed to the console. Printed in a tab-delimited format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_char = match self.action {
            Action::Create => "+".green(),
            Action::Update => "~".yellow(),
            Action::NoAction => "".normal(),
        };
        write!(
            f,
            "{}\t\t{}\t\t{}\t\t{}",
            start_char, self.region, self.parameter, self.value
        )
    }
}
/// Prints the header for the tab delimited table printed to
/// the console. Ugly method of achieving a decent looking table.
/// Will probably break w
pub fn print_change_table_header() {
    println!("Op\t\tRegion   \t\tParam   \t\tNewVal");
}

pub fn print_change_table_keys() {
    let create = "+ = create".green();
    let update = "~ = update".yellow();
    let no_action = " = no change";
    println!("Op Keys:");
    println!("{}", create);
    println!("{}", update);
    println!("{}", no_action);
}

pub fn print_ssm_changes(changes: &Vec<SSMChange>) {
    print_change_table_header();
    for change in changes {
        println!("{}", change);
    }
}

/// Accepts a list of changes to be made in SSM and actions
/// these changes using the AWS SDK.
pub async fn process_ssm_changes(changes: &Vec<SSMChange>) -> Result<(), Box<dyn Error>> {
    for change in changes {
        let change_type = parameter_type_from_ref(&change.type_param);
        match change.action {
            Action::NoAction => (),
            Action::Update => {
                client::ssm::put_parameter(
                    &change.parameter,
                    change.region.to_owned(),
                    &change.value,
                    None,
                )
                .await?;
            }
            Action::Create => {
                client::ssm::put_parameter(
                    &change.parameter,
                    change.region.to_owned(),
                    &change.value,
                    Some(change_type),
                )
                .await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_new_ssm_change() {
        let test = SSMChange {
            action: Action::NoAction,
            region: String::from("ap-southeast-2"),
            parameter: String::from("/test"),
            value: String::from("test"),
            type_param: ParameterType::SecureString,
        };
        assert_eq!(
            SSMChange::new(
                Action::NoAction,
                "ap-southeast-2",
                "/test",
                "test",
                ParameterType::SecureString
            ),
            test
        );
    }
}
