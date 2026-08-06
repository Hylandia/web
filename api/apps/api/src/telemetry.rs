use crate::config::Config;

pub fn init(config: &Config) -> Option<sentry::ClientInitGuard> {
    let dsn = config.sentry_dsn.as_deref()?;
    if dsn.is_empty() {
        return None;
    }

    let environment = match std::env::var("RUN_ENV").as_deref() {
        Ok("production") | Ok("prod") => "production",
        _ => "development",
    };

    let guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(environment.into()),
            traces_sample_rate: 0.2,
            ..Default::default()
        },
    ));

    tracing::info!(environment, "Sentry initialized");
    Some(guard)
}
