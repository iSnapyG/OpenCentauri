use nix::errno::Errno;

pub(crate) fn wrap_ioctl_negative_invalid(result: Result<i32, Errno>) -> Result<i32, Errno> {
    match result {
        Ok(num) => match num {
            ..=-1 => Err(Errno::UnknownErrno),
            _ => Ok(num),
        },
        Err(e) => Err(e),
    }
}
