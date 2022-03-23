use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::model::ParameterType;
use aws_sdk_ssm::output;
use aws_sdk_ssm::Region;

/// Attempts to retrieve the specified parameter from the given region.
pub async fn get_parameter(
    name: &str,
    region: Option<String>,
) -> Result<output::GetParameterOutput, aws_sdk_ssm::Error> {
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ssm::Client::new(&shared_config);
    let req = client.get_parameter().name(name).with_decryption(true);
    return Ok(req.send().await?);
}
/// Attempts to put a new SSM Parameter into the given region.
/// If the parameter exists it will be updated. When creating a
/// parameter, a type_param must be specified. When updating None
/// should be provided for the type.
pub async fn put_parameter(
    name: &str,
    region: String,
    new_value: &str,
    type_param: Option<ParameterType>,
) -> Result<output::PutParameterOutput, aws_sdk_ssm::Error> {
    let region_provider = RegionProviderChain::first_try(Region::new(region));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ssm::Client::new(&shared_config);
    let req = client
        .put_parameter()
        .name(name)
        .value(new_value)
        .set_type(type_param)
        .overwrite(true);
    return Ok(req.send().await?);
}
