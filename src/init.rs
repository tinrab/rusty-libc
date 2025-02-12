#[cfg(feature = "panic_handler")]
const _: () = {
    use atomic_dbg::dbg;
    use core::panic::PanicInfo;

    use crate::process::{exit, EXIT_FAILURE};

    #[panic_handler]
    unsafe fn panic(info: &PanicInfo<'_>) -> ! {
        dbg!(info);
        exit(EXIT_FAILURE)
    }
};
