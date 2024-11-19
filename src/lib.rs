use 
  agave_geyser_plugin_interface::geyser_plugin_interface::{
      GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
      ReplicaEntryInfoVersions, ReplicaTransactionInfoVersions, Result as PluginResult,
      SlotStatus,
  };


#[derive(Debug, Default)]
pub struct Plugin {
}

impl GeyserPlugin for Plugin {
  fn name(&self) -> &'static str {
    "dummy-geyser"
  }

  fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
      Ok(())
  }

  fn on_unload(&mut self) {
  }

  fn update_account(
      &self,
      _account: ReplicaAccountInfoVersions,
      _slot: u64,
      _is_startup: bool,
  ) -> PluginResult<()> {
      Ok(())
  }

  fn notify_end_of_startup(&self) -> PluginResult<()> {
      Ok(())
  }

  fn update_slot_status(
      &self,
      _slot: u64,
      _parent: Option<u64>,
      _status: SlotStatus,
  ) -> PluginResult<()> {
      Ok(())
  }

  fn notify_transaction(
      &self,
      _transaction: ReplicaTransactionInfoVersions<'_>,
      _slot: u64,
  ) -> PluginResult<()> {
      Ok(())
  }

  fn notify_entry(&self, _entry: ReplicaEntryInfoVersions) -> PluginResult<()> {
      Ok(())
  }

  fn notify_block_metadata(&self, _blockinfo: ReplicaBlockInfoVersions<'_>) -> PluginResult<()> {
      Ok(())
  }

  fn account_data_notifications_enabled(&self) -> bool {
    false
  }

  fn transaction_notifications_enabled(&self) -> bool {
    false
  }

  fn entry_notifications_enabled(&self) -> bool {
    false
  }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the Plugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
  let plugin = Plugin::default();
  let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
  Box::into_raw(plugin)
}