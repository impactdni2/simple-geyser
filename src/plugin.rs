use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, Result as PluginResult,
};

#[derive(Debug, Default)]
pub struct SimplePlugin {}

impl GeyserPlugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "dummy-geyser"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        false // process account changes
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false // dont process new txs
    }
}
