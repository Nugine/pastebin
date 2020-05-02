use crate::config::Config;
use crate::limiter::TokenBucket;
use crate::record_endpoint::{find_record, save_record};
use nuclear::{DynEndpoint, Module};
use std::time::Duration;

pub fn record_module(config: &Config) -> Module {
    let mut module = Module::new();

    let mut find = DynEndpoint::new(find_record);
    let mut save = DynEndpoint::new(save_record);

    if let Some(ref limiter) = config.limiter {
        let find_qps = limiter.find_qps as u64;
        let one_second = Duration::from_secs(1);

        let find_limiter = TokenBucket::new(one_second, find_qps, find_qps);

        let save_qps = limiter.save_qps as u64;
        let save_limiter = TokenBucket::new(one_second, save_qps, save_qps);

        find = find.middleware(find_limiter);
        save = save.middleware(save_limiter);
    }

    module.get("/records/:key", find);
    module.post("/records", save);

    module
}
