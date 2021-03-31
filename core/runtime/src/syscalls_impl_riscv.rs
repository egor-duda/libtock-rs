use libtock_platform::RawSyscalls;

unsafe impl RawSyscalls for crate::TockSyscalls {
    // This yield implementation is currently limited to RISC-V versions without
    // floating-point registers, as it does not mark them clobbered.
    #[cfg(not(any(target_feature = "d", target_feature = "f")))]
    unsafe fn yield1(r0: *mut ()) {
        // Safety: This matches the invariants required by the documentation on
        // RawSyscalls::yield1
        unsafe {
            asm!("ecall",
                 // x0 is the zero register.
                 lateout("x1") _, // Return address
                 // x2-x4 are stack, global, and thread pointers. sp is
                 // callee-saved.
                 lateout("x5") _, // t0
                 lateout("x6") _, // t1
                 lateout("x7") _, // t2
                 // x8 and x9 are s0 and s1 and are callee-saved.
                 inlateout("x10") r0 => _, // a0
                 lateout("x11") _,         // a1
                 lateout("x12") _,         // a2
                 lateout("x13") _,         // a3
                 inlateout("x14") 0 => _,  // a4
                 lateout("x15") _,         // a5
                 lateout("x16") _,         // a6
                 lateout("x17") _,         // a7
                 // x18-27 are s2-s11 and are callee-saved
                 lateout("x28") _, // t3
                 lateout("x29") _, // t4
                 lateout("x30") _, // t5
                 lateout("x31") _, // t6
            );
        }
    }

    // This yield implementation is currently limited to RISC-V versions without
    // floating-point registers, as it does not mark them clobbered.
    #[cfg(not(any(target_feature = "d", target_feature = "f")))]
    unsafe fn yield2(r0: *mut (), r1: *mut ()) {
        // Safety: This matches the invariants required by the documentation on
        // RawSyscalls::yield2
        unsafe {
            asm!("ecall",
                 // x0 is the zero register.
                 lateout("x1") _, // Return address
                 // x2-x4 are stack, global, and thread pointers. sp is
                 // callee-saved.
                 lateout("x5") _, // t0
                 lateout("x6") _, // t1
                 lateout("x7") _, // t2
                 // x8 and x9 are s0 and s1 and are callee-saved.
                 inlateout("x10") r0 => _, // a0
                 inlateout("x11") r1 => _, // a1
                 lateout("x12") _,         // a2
                 lateout("x13") _,         // a3
                 inlateout("x14") 0 => _,  // a4
                 lateout("x15") _,         // a5
                 lateout("x16") _,         // a6
                 lateout("x17") _,         // a7
                 // x18-27 are s2-s11 and are callee-saved
                 lateout("x28") _, // t3
                 lateout("x29") _, // t4
                 lateout("x30") _, // t5
                 lateout("x31") _, // t6
            );
        }
    }

    unsafe fn syscall1<const CLASS: usize>(mut r0: *mut ()) -> (*mut (), *mut ()) {
        let r1;
        // Safety: This matches the invariants required by the documentation on
        // RawSyscalls::syscall1
        unsafe {
            asm!("ecall",
                 inlateout("a0") r0,
                 lateout("a1") r1,
                 in("a4") CLASS,
                 options(preserves_flags, nostack, nomem),
            );
        }
        (r0, r1)
    }

    unsafe fn syscall2<const CLASS: usize>(mut r0: *mut (), mut r1: *mut ()) -> (*mut (), *mut ()) {
        // Safety: This matches the invariants required by the documentation on
        // RawSyscalls::syscall2
        unsafe {
            asm!("ecall",
                 inlateout("a0") r0,
                 inlateout("a1") r1,
                 in("a4") CLASS,
                 options(preserves_flags, nostack, nomem)
            );
        }
        (r0, r1)
    }

    unsafe fn syscall4<const CLASS: usize>(
        mut r0: *mut (),
        mut r1: *mut (),
        mut r2: *mut (),
        mut r3: *mut (),
    ) -> (*mut (), *mut (), *mut (), *mut ()) {
        // Safety: This matches the invariants required by the documentation on
        // RawSyscalls::syscall4
        unsafe {
            asm!("ecall",
                 inlateout("a0") r0,
                 inlateout("a1") r1,
                 inlateout("a2") r2,
                 inlateout("a3") r3,
                 in("a4") CLASS,
                 options(preserves_flags, nostack),
            );
        }
        (r0, r1, r2, r3)
    }
}
