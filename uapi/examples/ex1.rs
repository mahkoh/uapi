fn main() {
    let mut addr: uapi::c::sockaddr_in = uapi::pod_zeroed();
    uapi::getsockname(1, &mut addr);
}
