use anyhow::Result;
use structopt::StructOpt;

#[derive(serde::Deserialize)]
pub struct EnvVars {
    pub redis_port: u16,
}

fn main() -> Result<()> {
    env_logger::init();

    if let Ok(dotenv_path) = dotenv::dotenv() {
        log::debug!("Using dotenv config from {}", dotenv_path.display());
    }

    let env_vars: EnvVars = envy::from_env()?;

    let params = console_ui::Params::from_args();
    log::debug!("Using params {:?}", params);

    let redis_url = format!("127.0.0.1:{}", env_vars.redis_port);
    console_ui::run_ui(params, redis_chat::Chat::new(&redis_url)?)
}
