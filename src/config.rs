use aws_sdk_ssm::model::ParameterType;
use clap::Parser;

/// Program to sync SSM Parameter Store entries accross regions. New values for parameters can be
/// entered once the program is ran to prevent secrets ending up in shell history.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Parameter you want to sync
    #[clap(short, long)]
    pub parameter: String,
    /// Value to set parameter to
    #[clap(short, long)]
    pub value: Option<String>,
    /// Regions to work on
    #[clap(short, long)]
    pub regions: Option<Vec<String>>,

    /// Type of parameter. Used when a new param need to be created.
    #[clap(short, long)]
    pub type_param: ParameterType,

    /// Show what changes would be made but don't actually change anything.
    #[clap(short, long)]
    pub dry_run: bool,
}
