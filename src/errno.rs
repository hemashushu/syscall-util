// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

// ## about the return value of syscall:
//
// when a syscall complete, the return value is stored in the 'rax' register,
// if the operation failed, the value is a negative value (rax < 0).

// ## about the `errno`
//
// ### std library
//
// in the C std lib, this negative value will be transmuted to a positive number (-rax)
// and stores in a private thread-local variable 'errno' in the section '.tbss',
// the address of the variable can be obtained by the `libc` function:
//
// - '__errno_location' on linux (redox, fuchsia)
// - '__error' on freebas (ios, macos)
//
// finally the std lib returns value '-1'.
//
// in C, the error number can be obtained by external variable 'errno' in '<errno.h>',
// in Rust, it can be obtained by 'std::io::Error::last_os_error().raw_os_error()'
//
// NOTE THAT THERE IS NO 'errno' if invoke syscall by assembly directly,
// neither the 'std::io::Error::last_os_error()'.
//
// ### obtain the value of 'errno'
//
// to obtains the std lib errno, on linux (includes redox, fuchsia etc.), import
// the external function '__errno_location', e.g.
//
// ```rust
// extern "C" {
//    fn __errno_location() -> *mut u32;
// }
// ```
//
// the read the errno as the following:
//
// ```rust
// pub fn get_errno() -> u32 {
//     unsafe { *__errno_location() }
// }
// ```
//
// it is the same as:
//
// ```rust
// let e = std::io::Error::last_os_error();
// if let Some(n) = e.raw_os_error() ...
// ```
//
// to write the errno:
//
// ```rust
// pub fn set_errno(errno: u32) {
//     unsafe {
//         *__errno_location() = errno;
//     }
// }
// ```
//
// note that on the platform freebsd (and ios, macos), the
// external function name is '__error'
//
// the following error number list comes from Linux (kernel 6.3.3) source files:
// 'include/uapi/asm-generic/errno.h'
// 'include/uapi/asm-generic/errno-base.h'

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Errno {
    // 'include/uapi/asm-generic/errno-base.h'
    EPERM = 1,    /* Operation not permitted */
    ENOENT = 2,   /* No such file or directory */
    ESRCH = 3,    /* No such process */
    EINTR = 4,    /* Interrupted system call */
    EIO = 5,      /* I/O error */
    ENXIO = 6,    /* No such device or address */
    E2BIG = 7,    /* Argument list too long */
    ENOEXEC = 8,  /* Exec format error */
    EBADF = 9,    /* Bad file number */
    ECHILD = 10,  /* No child processes */
    EAGAIN = 11,  /* Try again */
    ENOMEM = 12,  /* Out of memory */
    EACCES = 13,  /* Permission denied */
    EFAULT = 14,  /* Bad address */
    ENOTBLK = 15, /* Block device required */
    EBUSY = 16,   /* Device or resource busy */
    EEXIST = 17,  /* File exists */
    EXDEV = 18,   /* Cross-device link */
    ENODEV = 19,  /* No such device */
    ENOTDIR = 20, /* Not a directory */
    EISDIR = 21,  /* Is a directory */
    EINVAL = 22,  /* Invalid argument */
    ENFILE = 23,  /* File table overflow */
    EMFILE = 24,  /* Too many open files */
    ENOTTY = 25,  /* Not a typewriter */
    ETXTBSY = 26, /* Text file busy */
    EFBIG = 27,   /* File too large */
    ENOSPC = 28,  /* No space left on device */
    ESPIPE = 29,  /* Illegal seek */
    EROFS = 30,   /* Read-only file system */
    EMLINK = 31,  /* Too many links */
    EPIPE = 32,   /* Broken pipe */
    EDOM = 33,    /* Math argument out of domain of func */
    ERANGE = 34,  /* Math result not representable */

    // 'include/uapi/asm-generic/errno.h'
    EDEADLK = 35,      /* Resource deadlock would occur */
    ENAMETOOLONG = 36, /* File name too long */
    ENOLCK = 37,       /* No record locks available */

    /*
     * This error code is special: arch syscall entry code will return
     * -ENOSYS if users try to call a syscall that doesn't exist.  To keep
     * failures of syscalls that really do exist distinguishable from
     * failures due to attempts to use a nonexistent syscall, syscall
     * implementations should refrain from returning -ENOSYS.
     */
    ENOSYS = 38, /* Invalid system call number */

    ENOTEMPTY = 39, /* Directory not empty */
    ELOOP = 40,     /* Too many symbolic links encountered */
    // EWOULDBLOCK = EAGAIN, /* Operation would block */
    ENOMSG = 42,   /* No message of desired type */
    EIDRM = 43,    /* Identifier removed */
    ECHRNG = 44,   /* Channel number out of range */
    EL2NSYNC = 45, /* Level 2 not synchronized */
    EL3HLT = 46,   /* Level 3 halted */
    EL3RST = 47,   /* Level 3 reset */
    ELNRNG = 48,   /* Link number out of range */
    EUNATCH = 49,  /* Protocol driver not attached */
    ENOCSI = 50,   /* No CSI structure available */
    EL2HLT = 51,   /* Level 2 halted */
    EBADE = 52,    /* Invalid exchange */
    EBADR = 53,    /* Invalid request descriptor */
    EXFULL = 54,   /* Exchange full */
    ENOANO = 55,   /* No anode */
    EBADRQC = 56,  /* Invalid request code */
    EBADSLT = 57,  /* Invalid slot */

    // EDEADLOCK = EDEADLK,
    EBFONT = 59,          /* Bad font file format */
    ENOSTR = 60,          /* Device not a stream */
    ENODATA = 61,         /* No data available */
    ETIME = 62,           /* Timer expired */
    ENOSR = 63,           /* Out of streams resources */
    ENONET = 64,          /* Machine is not on the network */
    ENOPKG = 65,          /* Package not installed */
    EREMOTE = 66,         /* Object is remote */
    ENOLINK = 67,         /* Link has been severed */
    EADV = 68,            /* Advertise error */
    ESRMNT = 69,          /* Srmount error */
    ECOMM = 70,           /* Communication error on send */
    EPROTO = 71,          /* Protocol error */
    EMULTIHOP = 72,       /* Multihop attempted */
    EDOTDOT = 73,         /* RFS specific error */
    EBADMSG = 74,         /* Not a data message */
    EOVERFLOW = 75,       /* Value too large for defined data type */
    ENOTUNIQ = 76,        /* Name not unique on network */
    EBADFD = 77,          /* File descriptor in bad state */
    EREMCHG = 78,         /* Remote address changed */
    ELIBACC = 79,         /* Can not access a needed shared library */
    ELIBBAD = 80,         /* Accessing a corrupted shared library */
    ELIBSCN = 81,         /* .lib section in a.out corrupted */
    ELIBMAX = 82,         /* Attempting to link in too many shared libraries */
    ELIBEXEC = 83,        /* Cannot exec a shared library directly */
    EILSEQ = 84,          /* Illegal byte sequence */
    ERESTART = 85,        /* Interrupted system call should be restarted */
    ESTRPIPE = 86,        /* Streams pipe error */
    EUSERS = 87,          /* Too many users */
    ENOTSOCK = 88,        /* Socket operation on non-socket */
    EDESTADDRREQ = 89,    /* Destination address required */
    EMSGSIZE = 90,        /* Message too long */
    EPROTOTYPE = 91,      /* Protocol wrong type for socket */
    ENOPROTOOPT = 92,     /* Protocol not available */
    EPROTONOSUPPORT = 93, /* Protocol not supported */
    ESOCKTNOSUPPORT = 94, /* Socket type not supported */
    EOPNOTSUPP = 95,      /* Operation not supported on transport endpoint */
    EPFNOSUPPORT = 96,    /* Protocol family not supported */
    EAFNOSUPPORT = 97,    /* Address family not supported by protocol */
    EADDRINUSE = 98,      /* Address already in use */
    EADDRNOTAVAIL = 99,   /* Cannot assign requested address */
    ENETDOWN = 100,       /* Network is down */
    ENETUNREACH = 101,    /* Network is unreachable */
    ENETRESET = 102,      /* Network dropped connection because of reset */
    ECONNABORTED = 103,   /* Software caused connection abort */
    ECONNRESET = 104,     /* Connection reset by peer */
    ENOBUFS = 105,        /* No buffer space available */
    EISCONN = 106,        /* Transport endpoint is already connected */
    ENOTCONN = 107,       /* Transport endpoint is not connected */
    ESHUTDOWN = 108,      /* Cannot send after transport endpoint shutdown */
    ETOOMANYREFS = 109,   /* Too many references: cannot splice */
    ETIMEDOUT = 110,      /* Connection timed out */
    ECONNREFUSED = 111,   /* Connection refused */
    EHOSTDOWN = 112,      /* Host is down */
    EHOSTUNREACH = 113,   /* No route to host */
    EALREADY = 114,       /* Operation already in progress */
    EINPROGRESS = 115,    /* Operation now in progress */
    ESTALE = 116,         /* Stale file handle */
    EUCLEAN = 117,        /* Structure needs cleaning */
    ENOTNAM = 118,        /* Not a XENIX named type file */
    ENAVAIL = 119,        /* No XENIX semaphores available */
    EISNAM = 120,         /* Is a named type file */
    EREMOTEIO = 121,      /* Remote I/O error */
    EDQUOT = 122,         /* Quota exceeded */

    ENOMEDIUM = 123,    /* No medium found */
    EMEDIUMTYPE = 124,  /* Wrong medium type */
    ECANCELED = 125,    /* Operation Canceled */
    ENOKEY = 126,       /* Required key not available */
    EKEYEXPIRED = 127,  /* Key has expired */
    EKEYREVOKED = 128,  /* Key has been revoked */
    EKEYREJECTED = 129, /* Key was rejected by service */

    /* for robust mutexes */
    EOWNERDEAD = 130,      /* Owner died */
    ENOTRECOVERABLE = 131, /* State not recoverable */
    ERFKILL = 132,         /* Operation not possible due to RF-kill */
    EHWPOISON = 133,       /* Memory page has hardware error */
}

// alias
pub const EWOULDBLOCK: Errno = Errno::EAGAIN; /* Operation would block */
pub const EDEADLOCK: Errno = Errno::EDEADLK;
