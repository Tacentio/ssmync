use std::error::Error;

/// Retrieves the list of available AWS regions. Any errors are returned
/// to the caller to be handled.
pub async fn list_regions() -> Result<Vec<String>, Box<dyn Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&shared_config);
    let req = client.describe_regions();
    let resp = req.send().await?;
    let mut regions: Vec<String> = Vec::new();

    if let Some(rs) = resp.regions {
        for r in rs {
            if let Some(region_name) = r.region_name {
                regions.push(region_name);
            }
        }
    }

    return Ok(regions);
}
