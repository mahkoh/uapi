use crate::*;

#[man("ptrace(2) with request = `PTRACE_TRACEME`")]
#[notest]
pub fn ptrace_traceme() -> Result<()> {
    let res = unsafe { c::ptrace(c::PTRACE_TRACEME) };
    map_err!(res).map(drop)
}
