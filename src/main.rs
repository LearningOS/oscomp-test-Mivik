#![no_std]
#![no_main]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate log;
extern crate alloc;

mod ctypes;

mod mm;
mod ptr;
mod signal;
mod syscall_imp;
mod task;

use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use axerrno::AxResult;
use axhal::{arch::UspaceContext, mem::virt_to_phys, paging::MappingFlags};
use axmm::{AddrSpace, kernel_aspace};
use axsync::Mutex;
use memory_addr::{PAGE_SIZE_4K, VirtAddr};

fn new_user_aspace_empty() -> AxResult<AddrSpace> {
    AddrSpace::new_empty(
        VirtAddr::from_usize(axconfig::plat::USER_SPACE_BASE),
        axconfig::plat::USER_SPACE_SIZE,
    )
}

unsafe extern "C" {
    fn start_signal_trampoline();
}

/// If the target architecture requires it, the kernel portion of the address
/// space will be copied to the user address space.
fn copy_from_kernel(aspace: &mut AddrSpace) -> AxResult {
    if !cfg!(target_arch = "aarch64") && !cfg!(target_arch = "loongarch64") {
        // ARMv8 (aarch64) and LoongArch64 use separate page tables for user space
        // (aarch64: TTBR0_EL1, LoongArch64: PGDL), so there is no need to copy the
        // kernel portion to the user page table.
        aspace.copy_mappings_from(&kernel_aspace().lock())?;
    }
    Ok(())
}

fn run_user_app(args: &[String], envs: &[String]) -> Option<i32> {
    let mut uspace = new_user_aspace_empty()
        .and_then(|mut it| {
            copy_from_kernel(&mut it)?;
            let signal_trampoline_paddr = virt_to_phys((start_signal_trampoline as usize).into());
            it.map_linear(
                axconfig::plat::SIGNAL_TRAMPOLINE.into(),
                signal_trampoline_paddr,
                PAGE_SIZE_4K,
                MappingFlags::READ | MappingFlags::EXECUTE | MappingFlags::USER,
            )?;
            Ok(it)
        })
        .expect("Failed to create user address space");

    let path = arceos_posix_api::FilePath::new(&args[0]).expect("Invalid file path");
    axfs::api::set_current_dir(path.parent().unwrap()).expect("Failed to set current dir");

    let args: Vec<String> = if args[0].ends_with(".sh") {
        ["/musl/busybox".to_owned(), "sh".to_owned()]
            .into_iter()
            .chain(args.iter().map(|it| it.clone()))
            .collect()
    } else {
        args.iter().map(|it| it.clone()).collect()
    };

    let (entry_vaddr, ustack_top) = mm::load_user_app(&mut uspace, &args, envs)
        .unwrap_or_else(|e| panic!("Failed to load user app: {}", e));
    let user_task = task::spawn_user_task(
        Arc::new(Mutex::new(uspace)),
        UspaceContext::new(entry_vaddr.into(), ustack_top, 2333),
        axconfig::plat::USER_HEAP_BASE as _,
    );
    user_task.join()
}

#[unsafe(no_mangle)]
fn main() {
    let testcases = option_env!("AX_TESTCASES_LIST")
        .unwrap_or_else(|| "Please specify the testcases list by making user_apps")
        .split(',')
        .filter(|&x| !x.is_empty());

    for testcase in testcases {
        let args = testcase
            .split_ascii_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let exit_code = run_user_app(&args, &[]);
        info!("User task {} exited with code: {:?}", testcase, exit_code);
    }
}
