use std::error::Error;
use std::io::{self, Write};
mod client;
mod config;
mod ssm_change;
pub mod ssm_change_error;
pub use config::CommandLineArgs;
use ssm_change::SSMChange;

fn print_warning(message: &str) {
    println!("WARN: {}", message);
}

fn get_value_from_stdin() -> Result<String, io::Error> {
    let mut input = String::new();
    print!("Enter parameter value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)?;
    let value = input.trim_end().to_owned();
    return Ok(value);
}

/// Requests the user to provide confirmation of the given
/// message. Useful when the user should read and acknowledge what
/// the program is about to do.
fn get_user_confirmation(message: &str) -> Result<bool, io::Error> {
    let mut input = String::new();
    print!("{message}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)?;
    if input.trim().eq_ignore_ascii_case("Y") {
        return Ok(true);
    }
    return Ok(false);
}
/// The main running function of the library.
/// Errors that occur in this function are bubbled up to main.
pub async fn run(cli_args: CommandLineArgs) -> Result<(), Box<dyn Error>> {
    // Get's the list of regions to operate on.
    // If none are provided on the command line, retrieves the list of regions
    // from AWS.
    let regions = match cli_args.regions {
        Some(rs) => rs,
        None => client::ec2::list_regions().await?,
    };
    let value = match cli_args.value {
        Some(val) => {
            print_warning("Value provided via CLI. If this is a secret, please ensure you remove this command from your shells history, or ensure a ' ' space was prepended to the command");
            val
        }
        None => get_value_from_stdin()?,
    };
    // Stores the changes that will be made to the SSM Parameter in each region.
    let mut changes: Vec<SSMChange> = Vec::with_capacity(regions.len());
    // For each specified region, attempt to get that parameter from SSM. The result of the API
    // call is sent to calculate_change which will handle appropriate errors from the SSM API call
    // or bubble up any other errors that should be returned to the user.
    for region in regions {
        let result = client::ssm::get_parameter(&cli_args.parameter, Some(region.to_owned())).await;
        let ssm_change = SSMChange::calculate_change(
            result,
            &region,
            &cli_args.parameter,
            &value,
            &cli_args.type_param,
        )?;
        changes.push(ssm_change);
    }

    ssm_change::print_ssm_changes(&changes);
    ssm_change::print_change_table_keys();
    if !cli_args.dry_run {
        let user_confirmed =
            get_user_confirmation("\nPlease confirm you want to make these changes (y/N): ")?;
        if user_confirmed {
            ssm_change::process_ssm_changes(&changes).await?;
            println!("Successfully synced parameters across regions");
        }
    }
    Ok(())
}
