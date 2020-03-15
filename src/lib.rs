use libc::c_int;

#[no_mangle]
pub extern "C" fn double(n: c_int) -> c_int {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(double(2), 4);
    }
}
