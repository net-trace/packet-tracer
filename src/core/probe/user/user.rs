#![allow(dead_code)] // FIXME

use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};

use crate::core::{
    events::{
        bpf::{BpfEventOwner, BpfEvents},
        EventField,
    },
    probe::user::proc::Process,
};
use crate::event_field;

#[derive(Debug, PartialEq)]
pub(crate) struct UsdtProbe {
    /// The provider name.
    pub provider: String,
    /// The probe's name.
    pub name: String,
    /// The probe's symbol.
    pub ksym: u64,

    /// The target's path
    pub path: PathBuf,
    /// The target's pid
    pub pid: i32,
}

impl UsdtProbe {
    /// Return a printable name.
    pub(crate) fn new(proc: &Process, target: &str) -> Result<Self> {
        let note = proc
            .usdt_info()
            .ok_or_else(|| anyhow!("No USDT information available"))?
            .get_note(target)?
            .ok_or_else(|| anyhow!("Target not found"))?;

        Ok(UsdtProbe {
            provider: note.provider.to_owned(),
            name: note.name.to_owned(),
            ksym: note.addr,
            path: proc.path().to_owned(),
            pid: proc.pid(),
        })
    }

    /// Return a printable name.
    pub(crate) fn name(&self) -> String {
        format!("usdt:{}:{}", self.provider, self.name)
    }
}

/// Registers the unmarshaler for the userpsace section of the event.
pub(crate) fn register_unmarshaler(events: &mut BpfEvents) -> Result<()> {
    events.register_unmarshaler(
        BpfEventOwner::Userspace,
        Box::new(|raw_section, fields| {
            if raw_section.data.len() != 17 {
                bail!(
                    "Section data is not the expected size {} != 17",
                    raw_section.data.len()
                );
            }

            let symbol = u64::from_ne_bytes(raw_section.data[0..8].try_into()?);
            let pid_tid = u64::from_ne_bytes(raw_section.data[8..16].try_into()?);
            let r#type = u8::from_ne_bytes(raw_section.data[16..17].try_into()?);

            // Split pid and tid
            let pid = (pid_tid & 0xFFFFFFFF) as i32;
            let tid = (pid_tid >> 32) as i32;

            fields.push(event_field!("pid", pid));
            fields.push(event_field!("tid", tid));

            // FIXME: Retrieving the process information every event is definitely very inefficient.
            let proc = Process::from_pid(pid)?;
            let sym_str = proc.get_symbol(symbol)?;

            fields.push(event_field!("symbol", sym_str));
            fields.push(event_field!("ip", symbol));
            fields.push(event_field!(
                "path",
                proc.path()
                    .to_str()
                    .ok_or_else(|| anyhow!("Wrong binary path"))?
                    .to_string()
            ));

            let type_str = match r#type {
                1 => "usdt",
                _ => "unknown",
            };
            fields.push(event_field!("type", type_str.to_string()));
            Ok(())
        }),
    )?;
    Ok(())
}
