use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
    process,
    str::FromStr,
    time::Instant,
};

const RCC_VERSION: &str = "0.1.7";

pub fn exe(args: Vec<&str>) {
    let quiet_mode = args.iter().any(|arg| arg == &"/Quiet");

    if !quiet_mode {
        println!("RawCCopy v{RCC_VERSION}\n");
    }

    // We use `std::time::Instant` for a robust, monotonic clock.
    // This is safer and more idiomatic than calling GetTickCount via FFI.
    let start_time = Instant::now();

    // --- Argument Marshalling ---
    // This section prepares the Rust command-line arguments to be passed to C.

    // 1. Collect all command-line arguments as CStrings.
    let c_args: Result<Vec<CString>, std::ffi::NulError> = std::iter::once("rawccopy.exe")
        .chain(args)
        .map(|arg| CString::from_str(arg))
        .collect();
    let c_args = c_args.unwrap();

    // 2. Create a C-style `argv` array: a vector of raw pointers.
    //    The `c_args` vector must outlive this `argv` vector to prevent dangling pointers.
    let mut argv: Vec<*mut c_char> = c_args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();

    // --- C Function Calls ---

    // C equivalent: execution_context cont = SetupContext(argc, argv);
    let context = unsafe { rawccopy_sys::SetupContext(argv.len() as c_int, argv.as_mut_ptr()) };

    // C equivalent: if (!cont) exit(-1);
    // Check if the context pointer is null, which indicates a setup failure.
    if context.is_null() {
        process::exit(-1);
    }

    // C equivalent: if (!PerformOperation(cont)) { ... }
    let operation_successful = unsafe { rawccopy_sys::PerformOperation(context) };

    if !operation_successful {
        // If the operation failed, clean up and exit with code -2.
        // We must cast the specific pointer to the generic `*mut c_void` that CleanUp expects.
        unsafe { rawccopy_sys::CleanUp(context as *mut c_void) };
        process::exit(-2);
    }

    // C equivalent: CleanUp(cont);
    // If the operation was successful, clean up normally.
    // We must cast the specific pointer to the generic `*mut c_void` that CleanUp expects.
    unsafe { rawccopy_sys::CleanUp(context as *mut c_void) };

    if !quiet_mode {
        // C equivalent: uint64_t duration = ElapsedTime(start);
        //              printf("Job took %.2f seconds.\n", ((double)duration)/1000.0);
        // The `elapsed()` method on `Instant` handles the duration calculation safely,
        // avoiding the wraparound issues present in the C implementation.
        let duration_secs = start_time.elapsed().as_secs_f64();
        println!("Job took {duration_secs:.2} seconds.");
    }
}
