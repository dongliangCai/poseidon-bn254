#[cfg(all(
    not(target_os = "zkvm"),
    not(target_vendor = "succinct"),
))]
mod write {
    pub(super) static mut HOOK: &dyn Fn([u8; 32]) = &|_| {};
}

#[cfg(all(target_os = "zkvm",target_vendor = "succinct"))]
mod read {
    pub(super) static mut HOOK: &dyn Fn() -> [u8; 32] = &|| sp1_lib::io::read_vec().try_into().unwrap();
}

#[cfg(all(target_os = "zkvm",target_vendor = "succinct"))]
pub unsafe fn set_zkvm_hint_read_hook<F>(make_callback: F)
where F: FnOnce() -> &'static dyn Fn() -> [u8; 32],
{
    read::HOOK = make_callback();
}

#[cfg(all(
    not(target_os = "zkvm"),
    not(target_vendor = "succinct"),
))]
pub unsafe fn set_zkvm_hint_write_hook<F>(make_callback: F)
where
    F: FnOnce() -> &'static dyn Fn([u8; 32]),
{
    write::HOOK = make_callback();
}

#[cfg(all(
    not(target_os = "zkvm"),
    not(target_vendor = "succinct"),
    feature = "zkvm-hint"
))]
#[inline]
pub fn write(result: [u8; 32]) {
    unsafe { write::HOOK(result) }
}


#[cfg(all(
    target_os = "zkvm",
    target_vendor = "succinct",
    feature = "zkvm-hint"
))]
#[inline]
pub fn read_hint() -> [u8; 32] {
    unsafe { read::HOOK() }
}