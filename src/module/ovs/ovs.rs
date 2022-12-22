use anyhow::{bail, Result};

use super::main_hook;
use crate::{
    cli::{dynamic::DynamicCommand, CliConfig},
    collect::Collector,
    core::{
        events::bpf::BpfEvents,
        probe::{
            user::{proc::Process, UsdtProbe},
            Hook, Probe, ProbeManager,
        },
    },
};

const OVS_COLLECTOR: &str = "ovs";

pub(crate) struct OvsCollector {}

impl Collector for OvsCollector {
    fn new() -> Result<OvsCollector> {
        Ok(OvsCollector {})
    }

    fn name(&self) -> &'static str {
        OVS_COLLECTOR
    }

    fn known_kernel_types(&self) -> Option<Vec<&'static str>> {
        None
    }

    fn register_cli(&self, cmd: &mut DynamicCommand) -> Result<()> {
        cmd.register_module_noargs(OVS_COLLECTOR)
    }

    fn init(
        &mut self,
        _: &CliConfig,
        probes: &mut ProbeManager,
        _events: &mut BpfEvents,
    ) -> Result<()> {
        let ovs = Process::from_cmd("ovs-vswitchd")?;

        match ovs.usdt_info() {
            None => bail!("USDTs not enabled on OVS"),
            Some(info) => {
                if !info.is_usdt("main::run_start")? {
                    bail!("main loop USDT not found");
                }
            }
        }

        let main_probe = Probe::Usdt(UsdtProbe::new(&ovs, "main::run_start")?);
        probes.register_hook_to(Hook::from(main_hook::DATA), main_probe)?;

        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        Ok(())
    }
}
