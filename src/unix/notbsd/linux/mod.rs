//! Linux-specific definitions for linux-like values

pub type useconds_t = u32;
pub type dev_t = u64;
pub type socklen_t = u32;
pub type pthread_t = c_ulong;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;
pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;

pub enum fpos64_t {} // TODO: fill this out with a struct

s! {
    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct dirent64 {
        pub d_ino: ::ino64_t,
        pub d_off: ::off64_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct rlimit64 {
        pub rlim_cur: rlim64_t,
        pub rlim_max: rlim64_t,
    }

    pub struct glob_t {
        pub gl_pathc: ::size_t,
        pub gl_pathv: *mut *mut c_char,
        pub gl_offs: ::size_t,
        pub gl_flags: ::c_int,

        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
    }

    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut c_char,
        pub ifa_flags: ::c_uint,
        pub ifa_addr: *mut ::sockaddr,
        pub ifa_netmask: *mut ::sockaddr,
        pub ifa_ifu: *mut ::sockaddr, // FIXME This should be a union
        pub ifa_data: *mut ::c_void
    }

    pub struct pthread_mutex_t {
        #[cfg(any(target_arch = "mips", target_arch = "mipsel",
                  target_arch = "arm"))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips", target_arch = "mipsel",
                      target_arch = "arm")))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEX_T],
    }

    pub struct pthread_rwlock_t {
        #[cfg(any(target_arch = "mips", target_arch = "mipsel",
                  target_arch = "arm"))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips", target_arch = "mipsel",
                      target_arch = "arm")))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_RWLOCK_T],
    }

    pub struct pthread_mutexattr_t {
        #[cfg(target_arch = "x86_64")]
        __align: [::c_int; 0],
        #[cfg(not(target_arch = "x86_64"))]
        __align: [::c_long; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEXATTR_T],
    }

    pub struct pthread_cond_t {
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_COND_T],
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        #[cfg(target_pointer_width = "32")]
        pub __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct sockaddr_nl {
        pub nl_family: ::sa_family_t,
        nl_pad: ::c_ushort,
        pub nl_pid: u32,
        pub nl_groups: u32
    }
}

pub const FILENAME_MAX: ::c_uint = 4096;
pub const L_tmpnam: ::c_uint = 20;
pub const _PC_NAME_MAX: ::c_int = 3;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_STREAM_MAX: ::c_int = 5;
pub const _SC_TZNAME_MAX: ::c_int = 6;
pub const _SC_JOB_CONTROL: ::c_int = 7;
pub const _SC_SAVED_IDS: ::c_int = 8;
pub const _SC_REALTIME_SIGNALS: ::c_int = 9;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 10;
pub const _SC_TIMERS: ::c_int = 11;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 12;
pub const _SC_PRIORITIZED_IO: ::c_int = 13;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 14;
pub const _SC_FSYNC: ::c_int = 15;
pub const _SC_MAPPED_FILES: ::c_int = 16;
pub const _SC_MEMLOCK: ::c_int = 17;
pub const _SC_MEMLOCK_RANGE: ::c_int = 18;
pub const _SC_MEMORY_PROTECTION: ::c_int = 19;
pub const _SC_MESSAGE_PASSING: ::c_int = 20;
pub const _SC_SEMAPHORES: ::c_int = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 22;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 23;
pub const _SC_AIO_MAX: ::c_int = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 25;
pub const _SC_DELAYTIMER_MAX: ::c_int = 26;
pub const _SC_MQ_OPEN_MAX: ::c_int = 27;
pub const _SC_MQ_PRIO_MAX: ::c_int = 28;
pub const _SC_VERSION: ::c_int = 29;
pub const _SC_PAGESIZE: ::c_int = 30;
pub const _SC_RTSIG_MAX: ::c_int = 31;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 32;
pub const _SC_SEM_VALUE_MAX: ::c_int = 33;
pub const _SC_SIGQUEUE_MAX: ::c_int = 34;
pub const _SC_TIMER_MAX: ::c_int = 35;
pub const _SC_BC_BASE_MAX: ::c_int = 36;
pub const _SC_BC_DIM_MAX: ::c_int = 37;
pub const _SC_BC_SCALE_MAX: ::c_int = 38;
pub const _SC_BC_STRING_MAX: ::c_int = 39;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 40;
pub const _SC_EXPR_NEST_MAX: ::c_int = 42;
pub const _SC_LINE_MAX: ::c_int = 43;
pub const _SC_RE_DUP_MAX: ::c_int = 44;
pub const _SC_2_VERSION: ::c_int = 46;
pub const _SC_2_C_BIND: ::c_int = 47;
pub const _SC_2_C_DEV: ::c_int = 48;
pub const _SC_2_FORT_DEV: ::c_int = 49;
pub const _SC_2_FORT_RUN: ::c_int = 50;
pub const _SC_2_SW_DEV: ::c_int = 51;
pub const _SC_2_LOCALEDEF: ::c_int = 52;
pub const _SC_IOV_MAX: ::c_int = 60;
pub const _SC_THREADS: ::c_int = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 68;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 69;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 70;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 71;
pub const _SC_TTY_NAME_MAX: ::c_int = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 73;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 74;
pub const _SC_THREAD_STACK_MIN: ::c_int = 75;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 76;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 79;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 80;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 81;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 84;
pub const _SC_ATEXIT_MAX: ::c_int = 87;
pub const _SC_XOPEN_VERSION: ::c_int = 89;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 90;
pub const _SC_XOPEN_UNIX: ::c_int = 91;
pub const _SC_XOPEN_CRYPT: ::c_int = 92;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 93;
pub const _SC_XOPEN_SHM: ::c_int = 94;
pub const _SC_2_CHAR_TERM: ::c_int = 95;
pub const _SC_2_UPE: ::c_int = 97;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 125;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 126;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 128;
pub const _SC_XOPEN_LEGACY: ::c_int = 129;
pub const _SC_XOPEN_REALTIME: ::c_int = 130;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 131;

pub const RLIM_SAVED_MAX: ::rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_CUR: ::rlim_t = RLIM_INFINITY;

pub const GLOB_ERR: ::c_int = 1 << 0;
pub const GLOB_MARK: ::c_int = 1 << 1;
pub const GLOB_NOSORT: ::c_int = 1 << 2;
pub const GLOB_DOOFFS: ::c_int = 1 << 3;
pub const GLOB_NOCHECK: ::c_int = 1 << 4;
pub const GLOB_APPEND: ::c_int = 1 << 5;
pub const GLOB_NOESCAPE: ::c_int = 1 << 6;

pub const GLOB_NOSPACE: ::c_int = 1;
pub const GLOB_ABORTED: ::c_int = 2;
pub const GLOB_NOMATCH: ::c_int = 3;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_RANDOM: ::c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;

pub const S_IEXEC: mode_t = 64;
pub const S_IWRITE: mode_t = 128;
pub const S_IREAD: mode_t = 256;

pub const F_LOCK: ::c_int = 1;
pub const F_TEST: ::c_int = 3;
pub const F_TLOCK: ::c_int = 2;
pub const F_ULOCK: ::c_int = 0;

pub const ST_RDONLY: ::c_ulong = 1;
pub const ST_NOSUID: ::c_ulong = 2;
pub const ST_NODEV: ::c_ulong = 4;
pub const ST_NOEXEC: ::c_ulong = 8;
pub const ST_SYNCHRONOUS: ::c_ulong = 16;
pub const ST_MANDLOCK: ::c_ulong = 64;
pub const ST_WRITE: ::c_ulong = 128;
pub const ST_APPEND: ::c_ulong = 256;
pub const ST_IMMUTABLE: ::c_ulong = 512;
pub const ST_NOATIME: ::c_ulong = 1024;
pub const ST_NODIRATIME: ::c_ulong = 2048;

pub const RTLD_NEXT: *mut ::c_void = -1i64 as *mut ::c_void;
pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const MAP_32BIT: ::c_int = 0x0040;

pub const TCP_MD5SIG: ::c_int = 14;

pub const F_DUPFD_CLOEXEC: ::c_int = 1030;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_MUTEX_T],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_COND_T],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
};
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;

pub const SCHED_OTHER: ::c_int = 0;
pub const SCHED_FIFO: ::c_int = 1;
pub const SCHED_RR: ::c_int = 2;
pub const SCHED_BATCH: ::c_int = 3;
pub const SCHED_IDLE: ::c_int = 5;

extern {
    pub fn shm_open(name: *const c_char, oflag: ::c_int,
                    mode: mode_t) -> ::c_int;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn __errno_location() -> *mut ::c_int;

    pub fn fopen64(filename: *const c_char,
                   mode: *const c_char) -> *mut ::FILE;
    pub fn freopen64(filename: *const c_char, mode: *const c_char,
                     file: *mut ::FILE) -> *mut ::FILE;
    pub fn tmpfile64() -> *mut ::FILE;
    pub fn fgetpos64(stream: *mut ::FILE, ptr: *mut fpos64_t) -> ::c_int;
    pub fn fsetpos64(stream: *mut ::FILE, ptr: *const fpos64_t) -> ::c_int;
    pub fn fstat64(fildes: ::c_int, buf: *mut stat64) -> ::c_int;
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn open64(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn creat64(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn lseek64(fd: ::c_int, offset: off64_t, whence: ::c_int) -> off64_t;
    pub fn pread64(fd: ::c_int, buf: *mut ::c_void, count: ::size_t,
                   offset: off64_t) -> ::ssize_t;
    pub fn pwrite64(fd: ::c_int, buf: *const ::c_void, count: ::size_t,
                    offset: off64_t) -> ::ssize_t;
    pub fn mmap64(addr: *mut ::c_void,
                  len: ::size_t,
                  prot: ::c_int,
                  flags: ::c_int,
                  fd: ::c_int,
                  offset: off64_t)
                  -> *mut ::c_void;
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn ftruncate64(fd: ::c_int, length: off64_t) -> ::c_int;
    pub fn readdir64_r(dirp: *mut ::DIR, entry: *mut ::dirent64,
                       result: *mut *mut ::dirent64) -> ::c_int;

    pub fn getrlimit64(resource: ::c_int, rlim: *mut rlimit64) -> ::c_int;
    pub fn setrlimit64(resource: ::c_int, rlim: *const rlimit64) -> ::c_int;
    pub fn fseeko64(stream: *mut ::FILE,
                    offset: ::off64_t,
                    whence: ::c_int) -> ::c_int;
    pub fn ftello64(stream: *mut ::FILE) -> ::off64_t;
    pub fn fallocate(fd: ::c_int, mode: ::c_int,
                     offset: ::off_t, len: ::off_t) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn readahead(fd: ::c_int, offset: ::off64_t,
                     count: ::size_t) -> ::ssize_t;
    pub fn getxattr(path: *const c_char, name: *const c_char,
                    value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn lgetxattr(path: *const c_char, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn fgetxattr(filedes: ::c_int, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn setxattr(path: *const c_char, name: *const c_char,
                    value: *const ::c_void, size: ::size_t,
                    flags: ::c_int) -> ::c_int;
    pub fn lsetxattr(path: *const c_char, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn fsetxattr(filedes: ::c_int, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn listxattr(path: *const c_char, list: *mut c_char,
                     size: ::size_t) -> ::ssize_t;
    pub fn llistxattr(path: *const c_char, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn flistxattr(filedes: ::c_int, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn removexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn lremovexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn fremovexattr(filedes: ::c_int, name: *const c_char) -> ::c_int;
}

cfg_if! {
    if #[cfg(target_env = "musl")] {
        mod musl;
        pub use self::musl::*;
    } else if #[cfg(any(target_arch = "mips", target_arch = "mipsel"))] {
        mod mips;
        pub use self::mips::*;
    } else {
        mod other;
        pub use self::other::*;
    }
}

pub const AF_NETLINK: ::c_int = 16;
