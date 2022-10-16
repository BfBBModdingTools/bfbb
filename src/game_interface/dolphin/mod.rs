//! Dolphin backend for [`GameInterface`](super::GameInterface)
use log::{debug, error, trace};
use process_memory::TryIntoProcessHandle;
use sysinfo::{ProcessExt, System, SystemExt};
use tap::TapFallible;
use thiserror::Error;

use crate::game_interface::{GameInterface, InterfaceError};

use self::dolphin_var::DolphinBackend;

use super::{InterfaceProvider, InterfaceResult};

mod data_member;
pub mod dolphin_var;

const REGION_SIZE: usize = 0x200_0000;

#[cfg(target_os = "linux")]
const PROCESS_NAME: &str = "dolphin-emu";
#[cfg(target_os = "windows")]
const PROCESS_NAME: &str = "Dolphin";

/// Error type for failures to interact with Dolphin's memory.
#[derive(Debug, Error)]
pub enum Error {
    /// The BfBB memory-region could not be found.
    #[error("The emulated memory region could not be found. Make sure Dolphin is running and the game is open.")]
    RegionNotFound,
    /// An error occurred while reading/writing to Dolphin's memory
    #[error("Dolphin memory could not be accessed.")]
    IO,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

/// Provider for [`GameInterface<DolphinBackend>`]
pub struct DolphinInterface {
    system: System,
    state: DolphinState,
}

// This is really no different from an Option, but allows us to more easily extend `Dolphin` with
// different states in the future.
#[allow(clippy::large_enum_variant)]
enum DolphinState {
    Unhooked,
    Hooked(GameInterface<DolphinBackend>),
}

impl Default for DolphinInterface {
    fn default() -> Self {
        Self {
            system: System::default(),
            state: DolphinState::Unhooked,
        }
    }
}

impl InterfaceProvider for DolphinInterface {
    type Backend = DolphinBackend;

    fn do_with_interface<T>(
        &mut self,
        fun: impl FnOnce(&mut GameInterface<Self::Backend>) -> InterfaceResult<T>,
    ) -> InterfaceResult<T> {
        let interface = self.get_interface_or_hook();

        // Run the user's provided code, catching any `Unhooked` error that may occur and setting our state accordingly
        fun(interface?).tap_err(|e| {
            if let InterfaceError::Unhooked = e {
                trace!("Unhooked from Dolphin");
                self.state = DolphinState::Unhooked;
            }
        })
    }

    fn is_available(&mut self) -> bool {
        self.get_interface_or_hook().is_ok()
    }
}

impl DolphinInterface {
    fn get_interface_or_hook(&mut self) -> InterfaceResult<&mut GameInterface<DolphinBackend>> {
        let interface = match self.state {
            DolphinState::Unhooked => {
                // TODO: Add information about why.
                let interface = self.hook()?;
                self.state = DolphinState::Hooked(interface);
                match self.state {
                    DolphinState::Unhooked => unreachable!(),
                    DolphinState::Hooked(ref mut interface) => interface,
                }
            }
            DolphinState::Hooked(ref mut interface) => interface,
        };
        Ok(interface)
    }

    /// Attempt to hook Dolphin
    ///
    /// Dolphin is considered "hooked" when it's process is found and the region of memory used
    /// for emulating the GameCube's memory is located. This method will always attempt to hook
    /// Dolphin when called, even if already hooked.
    fn hook(&mut self) -> InterfaceResult<GameInterface<DolphinBackend>> {
        // TODO: Differentiate errors between Dolphin not being found and Dolphin being found, but the game not being open.
        self.system.refresh_processes();

        let procs = self.system.processes_by_name(PROCESS_NAME);
        if let Some((pid, addr)) = procs
            .into_iter()
            .map(|p| {
                let pid = p.pid();
                trace!("{} found with pid {pid}", p.name());
                (pid, get_emulated_base_address(pid))
            })
            .find_map(|(pid, addr)| addr.map(|addr| (pid, addr)))
        {
            debug!("Found emulated memory region at {addr:#X}");
            let base_address = addr;

            // Convert sysinfo Pid (wrapper type) to process_memory Pid (platform specific alias)
            #[cfg(target_os = "windows")]
            // Work around for sysinfo crate using usize on Windows for Pids
            let pid = <sysinfo::Pid as Into<usize>>::into(pid) as u32;

            // This isn't uselss on *nix
            #[allow(clippy::useless_conversion)]
            let pid: process_memory::Pid = pid.into();
            let handle = pid.try_into_process_handle()?;
            return Ok(GameInterface::<DolphinBackend>::new(base_address, handle));
        }

        Err(InterfaceError::HookingFailed)
    }
}

#[cfg(target_os = "linux")]
fn get_emulated_base_address(pid: sysinfo::Pid) -> Option<usize> {
    use proc_maps::get_process_maps;
    let maps = match get_process_maps(pid.into()) {
        Err(e) => {
            error!("Could not get dolphin process maps\n{e:?}");
            return None;
        }
        Ok(maps) => maps,
    };

    // Multiple maps exist that fit our criteria who only differ by address.
    // Perhaps by chance, the last match appears to always have the correct address.
    let map = maps.iter().rev().find(|m| {
        m.size() == REGION_SIZE
            && m.filename()
                .map(|f| f.to_string_lossy().contains("dolphin-emu"))
                .unwrap_or(false)
    });
    map.map(|m| m.start())
}

#[cfg(target_os = "windows")]
fn get_emulated_base_address(pid: sysinfo::Pid) -> Option<usize> {
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::memoryapi::VirtualQueryEx;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::{QueryWorkingSetEx, PSAPI_WORKING_SET_EX_INFORMATION};
    use winapi::um::winnt::{
        MEMORY_BASIC_INFORMATION, MEM_MAPPED, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
        PROCESS_VM_READ, PROCESS_VM_WRITE,
    };

    unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
            0,
            <sysinfo::Pid as Into<usize>>::into(pid) as u32,
        );
        if handle.is_null() {
            // TODO use GetLastError for error feedback
            error!("Failed to open process handle for dolphin (pid {pid})");
            return None;
        }

        // Begin memory scan for Dolphin's emulated memory region
        // We are looking for a MEM_MAPPED region of size 0x2000000
        let mut mem_info = MEMORY_BASIC_INFORMATION::default();
        let mut mem = std::ptr::null();
        while VirtualQueryEx(
            handle,
            mem,
            &mut mem_info,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        ) == std::mem::size_of::<MEMORY_BASIC_INFORMATION>()
        {
            if mem_info.RegionSize == REGION_SIZE && mem_info.Type == MEM_MAPPED {
                let mut ws_info = PSAPI_WORKING_SET_EX_INFORMATION {
                    VirtualAddress: mem_info.BaseAddress,
                    ..Default::default()
                };
                if QueryWorkingSetEx(
                    handle,
                    std::ptr::addr_of_mut!(ws_info).cast::<std::ffi::c_void>(),
                    std::mem::size_of::<PSAPI_WORKING_SET_EX_INFORMATION>()
                        .try_into()
                        .unwrap(),
                ) != 0
                    && ws_info.VirtualAttributes.Valid() != 0
                {
                    return Some(mem_info.BaseAddress as usize);
                }
            }

            mem = mem.add(mem_info.RegionSize);
        }

        CloseHandle(handle);
    }

    None
}
