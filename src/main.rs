#![forbid(unsafe_code)]

fn main() -> std::io::Result<()> {
    sigma_theme::warp::run_service("Sigma Info", sigma_info::routes())
}
