use crate::config::BuildConfig;

pub trait Repository<T> {
    type Error;
    fn find_by_id(&self, id: &str) -> Result<Option<T>, Self::Error>;
    fn save(&self, entity: &T) -> Result<(), Self::Error>;
    fn delete(&self, id: &str) -> Result<bool, Self::Error>;
}

pub trait BundlerPlugin {
    fn name(&self) -> &str;
    fn on_build_start(&self, config: &BuildConfig);
    fn on_build_end(&self, config: &BuildConfig, duration_ms: u64);
}
