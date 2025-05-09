use std::collections::HashMap;

use crate::{
    config::{self, Config},
    controller::middleware,
    logger, scheduler,
};

#[must_use]
pub fn test_config() -> Config {
    Config {
        logger: config::Logger {
            enable: false,
            pretty_backtrace: true,
            level: logger::LogLevel::Off,
            format: logger::Format::Json,
            override_filter: None,
            file_appender: None,
        },
        server: config::Server {
            binding: "localhost".to_string(),
            port: 5555,
            host: "localhost".to_string(),
            ident: None,
            middlewares: middleware::Config::default(),
        },
        #[cfg(feature = "with-db")]
        database: config::Database {
            uri: "sqlite::memory:".to_string(),
            enable_logging: false,
            min_connections: 1,
            max_connections: 1,
            connect_timeout: 1,
            idle_timeout: 1,
            acquire_timeout: None,
            auto_migrate: false,
            dangerously_truncate: false,
            dangerously_recreate: false,
        },
        queue: None,
        auth: None,
        workers: config::Workers {
            mode: config::WorkerMode::ForegroundBlocking,
        },
        mailer: None,
        initializers: None,
        settings: None,
        scheduler: Some(scheduler::Config {
            jobs: HashMap::from([(
                "job 1".to_string(),
                scheduler::Job {
                    run: "echo loco".to_string(),
                    shell: true,
                    run_on_start: false,
                    cron: "*/5 * * * * *".to_string(),
                    tags: Some(vec!["base".to_string()]),
                    output: None,
                },
            )]),

            output: scheduler::Output::STDOUT,
        }),
        // Always use in-memory cache for tests if available
        #[cfg(feature = "cache_inmem")]
        cache: config::CacheConfig::InMem(config::InMemCacheConfig {
            max_capacity: 32 * 1024 * 1024, // Use explicit value instead of default
        }),
        // If cache_inmem is not enabled, use null cache
        #[cfg(not(feature = "cache_inmem"))]
        cache: config::CacheConfig::Null,
    }
}
