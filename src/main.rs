#![forbid(unsafe_code)]

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sigma_info::config::validate()?;
    sigma_theme::warp::run_service("Sigma Info", sigma_info::routes())?;
    Ok(())
}
