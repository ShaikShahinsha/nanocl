use nanocl_error::io::IoResult;
use nanocld_client::stubs::resource::Resource;

use crate::{
  config::CliConfig,
  models::{
    GenericDefaultOpts, ResourceArg, ResourceCommand, ResourceHistoryOpts,
    ResourceRevertOpts, ResourceRow,
  },
  utils,
};

use super::{
  GenericCommand, GenericCommandInspect, GenericCommandLs, GenericCommandRm,
};

impl GenericCommand for ResourceArg {
  fn object_name() -> &'static str {
    "resources"
  }
}

impl GenericCommandLs for ResourceArg {
  type Item = ResourceRow;
  type Args = ResourceArg;
  type ApiItem = nanocld_client::stubs::resource::Resource;

  fn get_key(item: &Self::Item) -> String {
    item.name.clone()
  }
}

impl GenericCommandRm<GenericDefaultOpts, String> for ResourceArg {}

impl GenericCommandInspect for ResourceArg {
  type ApiItem = Resource;
}

/// Function that execute when running `nanocl resource history`
async fn exec_resource_history(
  cli_conf: &CliConfig,
  opts: &ResourceHistoryOpts,
) -> IoResult<()> {
  let client = &cli_conf.client;
  let history = client.list_history_resource(&opts.name).await?;
  utils::print::print_yml(history)?;
  Ok(())
}

/// Function that execute when running `nanocl resource revert`
async fn exec_resource_revert(
  cli_conf: &CliConfig,
  opts: &ResourceRevertOpts,
) -> IoResult<()> {
  let client = &cli_conf.client;
  let resource = client.revert_resource(&opts.name, &opts.key).await?;
  utils::print::print_yml(resource)?;
  Ok(())
}

/// Function that execute when running `nanocl resource`
pub async fn exec_resource(
  cli_conf: &CliConfig,
  args: &ResourceArg,
) -> IoResult<()> {
  match &args.command {
    ResourceCommand::List(opts) => {
      ResourceArg::exec_ls(&cli_conf.client, args, opts).await
    }
    ResourceCommand::Remove(opts) => {
      ResourceArg::exec_rm(&cli_conf.client, opts, None).await
    }
    ResourceCommand::Inspect(opts) => {
      ResourceArg::exec_inspect(cli_conf, opts, None).await
    }
    ResourceCommand::History(opts) => {
      exec_resource_history(cli_conf, opts).await
    }
    ResourceCommand::Revert(opts) => exec_resource_revert(cli_conf, opts).await,
  }
}
