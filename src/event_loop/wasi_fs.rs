use crate::event_loop::poll::*;
use core::mem::MaybeUninit;
use core::fmt;

pub type Size = usize;

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Errno(u16);
/// No error occurred. System call completed successfully.
pub const ERRNO_SUCCESS: Errno = Errno(0);
/// Argument list too long.
pub const ERRNO_2BIG: Errno = Errno(1);
/// Permission denied.
pub const ERRNO_ACCES: Errno = Errno(2);
/// Address in use.
pub const ERRNO_ADDRINUSE: Errno = Errno(3);
/// Address not available.
pub const ERRNO_ADDRNOTAVAIL: Errno = Errno(4);
/// Address family not supported.
pub const ERRNO_AFNOSUPPORT: Errno = Errno(5);
/// Resource unavailable, or operation would block.
pub const ERRNO_AGAIN: Errno = Errno(6);
/// Connection already in progress.
pub const ERRNO_ALREADY: Errno = Errno(7);
/// Bad file descriptor.
pub const ERRNO_BADF: Errno = Errno(8);
/// Bad message.
pub const ERRNO_BADMSG: Errno = Errno(9);
/// Device or resource busy.
pub const ERRNO_BUSY: Errno = Errno(10);
/// Operation canceled.
pub const ERRNO_CANCELED: Errno = Errno(11);
/// No child processes.
pub const ERRNO_CHILD: Errno = Errno(12);
/// Connection aborted.
pub const ERRNO_CONNABORTED: Errno = Errno(13);
/// Connection refused.
pub const ERRNO_CONNREFUSED: Errno = Errno(14);
/// Connection reset.
pub const ERRNO_CONNRESET: Errno = Errno(15);
/// Resource deadlock would occur.
pub const ERRNO_DEADLK: Errno = Errno(16);
/// Destination address required.
pub const ERRNO_DESTADDRREQ: Errno = Errno(17);
/// Mathematics argument out of domain of function.
pub const ERRNO_DOM: Errno = Errno(18);
/// Reserved.
pub const ERRNO_DQUOT: Errno = Errno(19);
/// File exists.
pub const ERRNO_EXIST: Errno = Errno(20);
/// Bad address.
pub const ERRNO_FAULT: Errno = Errno(21);
/// File too large.
pub const ERRNO_FBIG: Errno = Errno(22);
/// Host is unreachable.
pub const ERRNO_HOSTUNREACH: Errno = Errno(23);
/// Identifier removed.
pub const ERRNO_IDRM: Errno = Errno(24);
/// Illegal byte sequence.
pub const ERRNO_ILSEQ: Errno = Errno(25);
/// Operation in progress.
pub const ERRNO_INPROGRESS: Errno = Errno(26);
/// Interrupted function.
pub const ERRNO_INTR: Errno = Errno(27);
/// Invalid argument.
pub const ERRNO_INVAL: Errno = Errno(28);
/// I/O error.
pub const ERRNO_IO: Errno = Errno(29);
/// Socket is connected.
pub const ERRNO_ISCONN: Errno = Errno(30);
/// Is a directory.
pub const ERRNO_ISDIR: Errno = Errno(31);
/// Too many levels of symbolic links.
pub const ERRNO_LOOP: Errno = Errno(32);
/// File descriptor value too large.
pub const ERRNO_MFILE: Errno = Errno(33);
/// Too many links.
pub const ERRNO_MLINK: Errno = Errno(34);
/// Message too large.
pub const ERRNO_MSGSIZE: Errno = Errno(35);
/// Reserved.
pub const ERRNO_MULTIHOP: Errno = Errno(36);
/// Filename too long.
pub const ERRNO_NAMETOOLONG: Errno = Errno(37);
/// Network is down.
pub const ERRNO_NETDOWN: Errno = Errno(38);
/// Connection aborted by network.
pub const ERRNO_NETRESET: Errno = Errno(39);
/// Network unreachable.
pub const ERRNO_NETUNREACH: Errno = Errno(40);
/// Too many files open in system.
pub const ERRNO_NFILE: Errno = Errno(41);
/// No buffer space available.
pub const ERRNO_NOBUFS: Errno = Errno(42);
/// No such device.
pub const ERRNO_NODEV: Errno = Errno(43);
/// No such file or directory.
pub const ERRNO_NOENT: Errno = Errno(44);
/// Executable file format error.
pub const ERRNO_NOEXEC: Errno = Errno(45);
/// No locks available.
pub const ERRNO_NOLCK: Errno = Errno(46);
/// Reserved.
pub const ERRNO_NOLINK: Errno = Errno(47);
/// Not enough space.
pub const ERRNO_NOMEM: Errno = Errno(48);
/// No message of the desired type.
pub const ERRNO_NOMSG: Errno = Errno(49);
/// Protocol not available.
pub const ERRNO_NOPROTOOPT: Errno = Errno(50);
/// No space left on device.
pub const ERRNO_NOSPC: Errno = Errno(51);
/// Function not supported.
pub const ERRNO_NOSYS: Errno = Errno(52);
/// The socket is not connected.
pub const ERRNO_NOTCONN: Errno = Errno(53);
/// Not a directory or a symbolic link to a directory.
pub const ERRNO_NOTDIR: Errno = Errno(54);
/// Directory not empty.
pub const ERRNO_NOTEMPTY: Errno = Errno(55);
/// State not recoverable.
pub const ERRNO_NOTRECOVERABLE: Errno = Errno(56);
/// Not a socket.
pub const ERRNO_NOTSOCK: Errno = Errno(57);
/// Not supported, or operation not supported on socket.
pub const ERRNO_NOTSUP: Errno = Errno(58);
/// Inappropriate I/O control operation.
pub const ERRNO_NOTTY: Errno = Errno(59);
/// No such device or address.
pub const ERRNO_NXIO: Errno = Errno(60);
/// Value too large to be stored in data type.
pub const ERRNO_OVERFLOW: Errno = Errno(61);
/// Previous owner died.
pub const ERRNO_OWNERDEAD: Errno = Errno(62);
/// Operation not permitted.
pub const ERRNO_PERM: Errno = Errno(63);
/// Broken pipe.
pub const ERRNO_PIPE: Errno = Errno(64);
/// Protocol error.
pub const ERRNO_PROTO: Errno = Errno(65);
/// Protocol not supported.
pub const ERRNO_PROTONOSUPPORT: Errno = Errno(66);
/// Protocol wrong type for socket.
pub const ERRNO_PROTOTYPE: Errno = Errno(67);
/// Result too large.
pub const ERRNO_RANGE: Errno = Errno(68);
/// Read-only file system.
pub const ERRNO_ROFS: Errno = Errno(69);
/// Invalid seek.
pub const ERRNO_SPIPE: Errno = Errno(70);
/// No such process.
pub const ERRNO_SRCH: Errno = Errno(71);
/// Reserved.
pub const ERRNO_STALE: Errno = Errno(72);
/// Connection timed out.
pub const ERRNO_TIMEDOUT: Errno = Errno(73);
/// Text file busy.
pub const ERRNO_TXTBSY: Errno = Errno(74);
/// Cross-device link.
pub const ERRNO_XDEV: Errno = Errno(75);
/// Extension: Capabilities insufficient.
pub const ERRNO_NOTCAPABLE: Errno = Errno(76);
impl Errno {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SUCCESS",
            1 => "2BIG",
            2 => "ACCES",
            3 => "ADDRINUSE",
            4 => "ADDRNOTAVAIL",
            5 => "AFNOSUPPORT",
            6 => "AGAIN",
            7 => "ALREADY",
            8 => "BADF",
            9 => "BADMSG",
            10 => "BUSY",
            11 => "CANCELED",
            12 => "CHILD",
            13 => "CONNABORTED",
            14 => "CONNREFUSED",
            15 => "CONNRESET",
            16 => "DEADLK",
            17 => "DESTADDRREQ",
            18 => "DOM",
            19 => "DQUOT",
            20 => "EXIST",
            21 => "FAULT",
            22 => "FBIG",
            23 => "HOSTUNREACH",
            24 => "IDRM",
            25 => "ILSEQ",
            26 => "INPROGRESS",
            27 => "INTR",
            28 => "INVAL",
            29 => "IO",
            30 => "ISCONN",
            31 => "ISDIR",
            32 => "LOOP",
            33 => "MFILE",
            34 => "MLINK",
            35 => "MSGSIZE",
            36 => "MULTIHOP",
            37 => "NAMETOOLONG",
            38 => "NETDOWN",
            39 => "NETRESET",
            40 => "NETUNREACH",
            41 => "NFILE",
            42 => "NOBUFS",
            43 => "NODEV",
            44 => "NOENT",
            45 => "NOEXEC",
            46 => "NOLCK",
            47 => "NOLINK",
            48 => "NOMEM",
            49 => "NOMSG",
            50 => "NOPROTOOPT",
            51 => "NOSPC",
            52 => "NOSYS",
            53 => "NOTCONN",
            54 => "NOTDIR",
            55 => "NOTEMPTY",
            56 => "NOTRECOVERABLE",
            57 => "NOTSOCK",
            58 => "NOTSUP",
            59 => "NOTTY",
            60 => "NXIO",
            61 => "OVERFLOW",
            62 => "OWNERDEAD",
            63 => "PERM",
            64 => "PIPE",
            65 => "PROTO",
            66 => "PROTONOSUPPORT",
            67 => "PROTOTYPE",
            68 => "RANGE",
            69 => "ROFS",
            70 => "SPIPE",
            71 => "SRCH",
            72 => "STALE",
            73 => "TIMEDOUT",
            74 => "TXTBSY",
            75 => "XDEV",
            76 => "NOTCAPABLE",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "No error occurred. System call completed successfully.",
            1 => "Argument list too long.",
            2 => "Permission denied.",
            3 => "Address in use.",
            4 => "Address not available.",
            5 => "Address family not supported.",
            6 => "Resource unavailable, or operation would block.",
            7 => "Connection already in progress.",
            8 => "Bad file descriptor.",
            9 => "Bad message.",
            10 => "Device or resource busy.",
            11 => "Operation canceled.",
            12 => "No child processes.",
            13 => "Connection aborted.",
            14 => "Connection refused.",
            15 => "Connection reset.",
            16 => "Resource deadlock would occur.",
            17 => "Destination address required.",
            18 => "Mathematics argument out of domain of function.",
            19 => "Reserved.",
            20 => "File exists.",
            21 => "Bad address.",
            22 => "File too large.",
            23 => "Host is unreachable.",
            24 => "Identifier removed.",
            25 => "Illegal byte sequence.",
            26 => "Operation in progress.",
            27 => "Interrupted function.",
            28 => "Invalid argument.",
            29 => "I/O error.",
            30 => "Socket is connected.",
            31 => "Is a directory.",
            32 => "Too many levels of symbolic links.",
            33 => "File descriptor value too large.",
            34 => "Too many links.",
            35 => "Message too large.",
            36 => "Reserved.",
            37 => "Filename too long.",
            38 => "Network is down.",
            39 => "Connection aborted by network.",
            40 => "Network unreachable.",
            41 => "Too many files open in system.",
            42 => "No buffer space available.",
            43 => "No such device.",
            44 => "No such file or directory.",
            45 => "Executable file format error.",
            46 => "No locks available.",
            47 => "Reserved.",
            48 => "Not enough space.",
            49 => "No message of the desired type.",
            50 => "Protocol not available.",
            51 => "No space left on device.",
            52 => "Function not supported.",
            53 => "The socket is not connected.",
            54 => "Not a directory or a symbolic link to a directory.",
            55 => "Directory not empty.",
            56 => "State not recoverable.",
            57 => "Not a socket.",
            58 => "Not supported, or operation not supported on socket.",
            59 => "Inappropriate I/O control operation.",
            60 => "No such device or address.",
            61 => "Value too large to be stored in data type.",
            62 => "Previous owner died.",
            63 => "Operation not permitted.",
            64 => "Broken pipe.",
            65 => "Protocol error.",
            66 => "Protocol not supported.",
            67 => "Protocol wrong type for socket.",
            68 => "Result too large.",
            69 => "Read-only file system.",
            70 => "Invalid seek.",
            71 => "No such process.",
            72 => "Reserved.",
            73 => "Connection timed out.",
            74 => "Text file busy.",
            75 => "Cross-device link.",
            76 => "Extension: Capabilities insufficient.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Errno")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}
impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (error {})", self.name(), self.0)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Iovec {
    /// The address of the buffer to be filled.
    pub buf: *mut u8,
    /// The length of the buffer to be filled.
    pub buf_len: Size,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ciovec {
    /// The address of the buffer to be written.
    pub buf: *const u8,
    /// The length of the buffer to be written.
    pub buf_len: Size,
}
pub type IovecArray<'a> = &'a [Iovec];
pub type CiovecArray<'a> = &'a [Ciovec];
pub type Filedelta = i64;

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Whence(u8);
/// Seek relative to start-of-file.
pub const WHENCE_SET: Whence = Whence(0);
/// Seek relative to current position.
pub const WHENCE_CUR: Whence = Whence(1);
/// Seek relative to end-of-file.
pub const WHENCE_END: Whence = Whence(2);
impl Whence {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SET",
            1 => "CUR",
            2 => "END",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "Seek relative to start-of-file.",
            1 => "Seek relative to current position.",
            2 => "Seek relative to end-of-file.",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for Whence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Whence")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

pub type Dircookie = u64;
pub type Dirnamlen = u32;
pub type Inode = u64;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Filetype(u8);
/// The type of the file descriptor or file is unknown or is different from any of the other types specified.
pub const FILETYPE_UNKNOWN: Filetype = Filetype(0);
/// The file descriptor or file refers to a block device inode.
pub const FILETYPE_BLOCK_DEVICE: Filetype = Filetype(1);
/// The file descriptor or file refers to a character device inode.
pub const FILETYPE_CHARACTER_DEVICE: Filetype = Filetype(2);
/// The file descriptor or file refers to a directory inode.
pub const FILETYPE_DIRECTORY: Filetype = Filetype(3);
/// The file descriptor or file refers to a regular file inode.
pub const FILETYPE_REGULAR_FILE: Filetype = Filetype(4);
/// The file descriptor or file refers to a datagram socket.
pub const FILETYPE_SOCKET_DGRAM: Filetype = Filetype(5);
/// The file descriptor or file refers to a byte-stream socket.
pub const FILETYPE_SOCKET_STREAM: Filetype = Filetype(6);
/// The file refers to a symbolic link inode.
pub const FILETYPE_SYMBOLIC_LINK: Filetype = Filetype(7);
impl Filetype {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "UNKNOWN",
            1 => "BLOCK_DEVICE",
            2 => "CHARACTER_DEVICE",
            3 => "DIRECTORY",
            4 => "REGULAR_FILE",
            5 => "SOCKET_DGRAM",
            6 => "SOCKET_STREAM",
            7 => "SYMBOLIC_LINK",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {0 => "The type of the file descriptor or file is unknown or is different from any of the other types specified.",1 => "The file descriptor or file refers to a block device inode.",2 => "The file descriptor or file refers to a character device inode.",3 => "The file descriptor or file refers to a directory inode.",4 => "The file descriptor or file refers to a regular file inode.",5 => "The file descriptor or file refers to a datagram socket.",6 => "The file descriptor or file refers to a byte-stream socket.",7 => "The file refers to a symbolic link inode.",_ => unsafe { core::hint::unreachable_unchecked() },}
    }
}
impl fmt::Debug for Filetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filetype")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Dirent {
    /// The offset of the next directory entry stored in this directory.
    pub d_next: Dircookie,
    /// The serial number of the file referred to by this directory entry.
    pub d_ino: Inode,
    /// The length of the name of the directory entry.
    pub d_namlen: Dirnamlen,
    /// The type of the file referred to by this directory entry.
    pub d_type: Filetype,
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Advice(u8);
/// The application has no advice to give on its behavior with respect to the specified data.
pub const ADVICE_NORMAL: Advice = Advice(0);
/// The application expects to access the specified data sequentially from lower offsets to higher offsets.
pub const ADVICE_SEQUENTIAL: Advice = Advice(1);
/// The application expects to access the specified data in a random order.
pub const ADVICE_RANDOM: Advice = Advice(2);
/// The application expects to access the specified data in the near future.
pub const ADVICE_WILLNEED: Advice = Advice(3);
/// The application expects that it will not access the specified data in the near future.
pub const ADVICE_DONTNEED: Advice = Advice(4);
/// The application expects to access the specified data once and then not reuse it thereafter.
pub const ADVICE_NOREUSE: Advice = Advice(5);
impl Advice {
    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "NORMAL",
            1 => "SEQUENTIAL",
            2 => "RANDOM",
            3 => "WILLNEED",
            4 => "DONTNEED",
            5 => "NOREUSE",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {0 => "The application has no advice to give on its behavior with respect to the specified data.",1 => "The application expects to access the specified data sequentially from lower offsets to higher offsets.",2 => "The application expects to access the specified data in a random order.",3 => "The application expects to access the specified data in the near future.",4 => "The application expects that it will not access the specified data in the near future.",5 => "The application expects to access the specified data once and then not reuse it thereafter.",_ => unsafe { core::hint::unreachable_unchecked() },}
    }
}
impl fmt::Debug for Advice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Advice")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Fdstat {
    /// File type.
    pub fs_filetype: Filetype,
    /// File descriptor flags.
    pub fs_flags: Fdflags,
    /// Rights that apply to this file descriptor.
    pub fs_rights_base: Rights,
    /// Maximum set of rights that may be installed on new file descriptors that
    /// are created through this file descriptor, e.g., through `path_open`.
    pub fs_rights_inheriting: Rights,
}

pub type Fdflags = u16;
/// Append mode: Data written to the file is always appended to the file's end.
pub const FDFLAGS_APPEND: Fdflags = 1 << 0;
/// Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.
pub const FDFLAGS_DSYNC: Fdflags = 1 << 1;
/// Non-blocking mode.
pub const FDFLAGS_NONBLOCK: Fdflags = 1 << 2;
/// Synchronized read I/O operations.
pub const FDFLAGS_RSYNC: Fdflags = 1 << 3;
/// Write according to synchronized I/O file integrity completion. In
/// addition to synchronizing the data stored in the file, the implementation
/// may also synchronously update the file's metadata.
pub const FDFLAGS_SYNC: Fdflags = 1 << 4;

pub type Rights = u64;
/// The right to invoke `fd_datasync`.
/// If `path_open` is set, includes the right to invoke
/// `path_open` with `fdflags::dsync`.
pub const RIGHTS_FD_DATASYNC: Rights = 1 << 0;
/// The right to invoke `fd_read` and `sock_recv`.
/// If `rights::fd_seek` is set, includes the right to invoke `fd_pread`.
pub const RIGHTS_FD_READ: Rights = 1 << 1;
/// The right to invoke `fd_seek`. This flag implies `rights::fd_tell`.
pub const RIGHTS_FD_SEEK: Rights = 1 << 2;
/// The right to invoke `fd_fdstat_set_flags`.
pub const RIGHTS_FD_FDSTAT_SET_FLAGS: Rights = 1 << 3;
/// The right to invoke `fd_sync`.
/// If `path_open` is set, includes the right to invoke
/// `path_open` with `fdflags::rsync` and `fdflags::dsync`.
pub const RIGHTS_FD_SYNC: Rights = 1 << 4;
/// The right to invoke `fd_seek` in such a way that the file offset
/// remains unaltered (i.e., `whence::cur` with offset zero), or to
/// invoke `fd_tell`.
pub const RIGHTS_FD_TELL: Rights = 1 << 5;
/// The right to invoke `fd_write` and `sock_send`.
/// If `rights::fd_seek` is set, includes the right to invoke `fd_pwrite`.
pub const RIGHTS_FD_WRITE: Rights = 1 << 6;
/// The right to invoke `fd_advise`.
pub const RIGHTS_FD_ADVISE: Rights = 1 << 7;
/// The right to invoke `fd_allocate`.
pub const RIGHTS_FD_ALLOCATE: Rights = 1 << 8;
/// The right to invoke `path_create_directory`.
pub const RIGHTS_PATH_CREATE_DIRECTORY: Rights = 1 << 9;
/// If `path_open` is set, the right to invoke `path_open` with `oflags::creat`.
pub const RIGHTS_PATH_CREATE_FILE: Rights = 1 << 10;
/// The right to invoke `path_link` with the file descriptor as the
/// source directory.
pub const RIGHTS_PATH_LINK_SOURCE: Rights = 1 << 11;
/// The right to invoke `path_link` with the file descriptor as the
/// target directory.
pub const RIGHTS_PATH_LINK_TARGET: Rights = 1 << 12;
/// The right to invoke `path_open`.
pub const RIGHTS_PATH_OPEN: Rights = 1 << 13;
/// The right to invoke `fd_readdir`.
pub const RIGHTS_FD_READDIR: Rights = 1 << 14;
/// The right to invoke `path_readlink`.
pub const RIGHTS_PATH_READLINK: Rights = 1 << 15;
/// The right to invoke `path_rename` with the file descriptor as the source directory.
pub const RIGHTS_PATH_RENAME_SOURCE: Rights = 1 << 16;
/// The right to invoke `path_rename` with the file descriptor as the target directory.
pub const RIGHTS_PATH_RENAME_TARGET: Rights = 1 << 17;
/// The right to invoke `path_filestat_get`.
pub const RIGHTS_PATH_FILESTAT_GET: Rights = 1 << 18;
/// The right to change a file's size (there is no `path_filestat_set_size`).
/// If `path_open` is set, includes the right to invoke `path_open` with `oflags::trunc`.
pub const RIGHTS_PATH_FILESTAT_SET_SIZE: Rights = 1 << 19;
/// The right to invoke `path_filestat_set_times`.
pub const RIGHTS_PATH_FILESTAT_SET_TIMES: Rights = 1 << 20;
/// The right to invoke `fd_filestat_get`.
pub const RIGHTS_FD_FILESTAT_GET: Rights = 1 << 21;
/// The right to invoke `fd_filestat_set_size`.
pub const RIGHTS_FD_FILESTAT_SET_SIZE: Rights = 1 << 22;
/// The right to invoke `fd_filestat_set_times`.
pub const RIGHTS_FD_FILESTAT_SET_TIMES: Rights = 1 << 23;
/// The right to invoke `path_symlink`.
pub const RIGHTS_PATH_SYMLINK: Rights = 1 << 24;
/// The right to invoke `path_remove_directory`.
pub const RIGHTS_PATH_REMOVE_DIRECTORY: Rights = 1 << 25;
/// The right to invoke `path_unlink_file`.
pub const RIGHTS_PATH_UNLINK_FILE: Rights = 1 << 26;
/// If `rights::fd_read` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_read`.
/// If `rights::fd_write` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_write`.
pub const RIGHTS_POLL_FD_READWRITE: Rights = 1 << 27;
/// The right to invoke `sock_shutdown`.
pub const RIGHTS_SOCK_SHUTDOWN: Rights = 1 << 28;
/// The right to invoke `sock_accept`.
pub const RIGHTS_SOCK_ACCEPT: Rights = 1 << 29;

pub type Device = u64;
pub type Fstflags = u16;
/// Adjust the last data access timestamp to the value stored in `filestat::atim`.
pub const FSTFLAGS_ATIM: Fstflags = 1 << 0;
/// Adjust the last data access timestamp to the time of clock `clockid::realtime`.
pub const FSTFLAGS_ATIM_NOW: Fstflags = 1 << 1;
/// Adjust the last data modification timestamp to the value stored in `filestat::mtim`.
pub const FSTFLAGS_MTIM: Fstflags = 1 << 2;
/// Adjust the last data modification timestamp to the time of clock `clockid::realtime`.
pub const FSTFLAGS_MTIM_NOW: Fstflags = 1 << 3;

pub type Lookupflags = u32;
/// As long as the resolved path corresponds to a symbolic link, it is expanded.
pub const LOOKUPFLAGS_SYMLINK_FOLLOW: Lookupflags = 1 << 0;

pub type Oflags = u16;
/// Create file if it does not exist.
pub const OFLAGS_CREAT: Oflags = 1 << 0;
/// Fail if not a directory.
pub const OFLAGS_DIRECTORY: Oflags = 1 << 1;
/// Fail if file already exists.
pub const OFLAGS_EXCL: Oflags = 1 << 2;
/// Truncate file to size 0.
pub const OFLAGS_TRUNC: Oflags = 1 << 3;

pub type Linkcount = u64;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Filestat {
    /// Device ID of device containing the file.
    pub dev: Device,
    /// File serial number.
    pub ino: Inode,
    /// File type.
    pub filetype: Filetype,
    /// Number of hard links to the file.
    pub nlink: Linkcount,
    /// For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.
    pub size: Filesize,
    /// Last data access timestamp.
    pub atim: Timestamp,
    /// Last data modification timestamp.
    pub mtim: Timestamp,
    /// Last file status change timestamp.
    pub ctim: Timestamp,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PrestatDir {
    /// The length of the directory name for use with `fd_prestat_dir_name`.
    pub pr_name_len: Size,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PrestatU {
    pub dir: PrestatDir,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Prestat {
    pub tag: u8,
    pub u: PrestatU,
}

mod wasi {
    #[link(wasm_import_module = "wasi_snapshot_preview1")]
    extern "C" {
        /// Provide file advisory information on a file descriptor.
        /// Note: This is similar to `posix_fadvise` in POSIX.
        pub fn fd_advise(arg0: i32, arg1: i64, arg2: i64, arg3: i32) -> i32;
        /// Force the allocation of space in a file.
        /// Note: This is similar to `posix_fallocate` in POSIX.
        pub fn fd_allocate(arg0: i32, arg1: i64, arg2: i64) -> i32;
        /// Close a file descriptor.
        /// Note: This is similar to `close` in POSIX.
        pub fn fd_close(arg0: i32) -> i32;
        /// Synchronize the data of a file to disk.
        /// Note: This is similar to `fdatasync` in POSIX.
        pub fn fd_datasync(arg0: i32) -> i32;
        /// Get the attributes of a file descriptor.
        /// Note: This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields.
        pub fn fd_fdstat_get(arg0: i32, arg1: i32) -> i32;
        /// Adjust the flags associated with a file descriptor.
        /// Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
        pub fn fd_fdstat_set_flags(arg0: i32, arg1: i32) -> i32;
        /// Adjust the rights associated with a file descriptor.
        /// This can only be used to remove rights, and returns `errno::notcapable` if called in a way that would attempt to add rights
        pub fn fd_fdstat_set_rights(arg0: i32, arg1: i64, arg2: i64) -> i32;
        /// Return the attributes of an open file.
        pub fn fd_filestat_get(arg0: i32, arg1: i32) -> i32;
        /// Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros.
        /// Note: This is similar to `ftruncate` in POSIX.
        pub fn fd_filestat_set_size(arg0: i32, arg1: i64) -> i32;
        /// Adjust the timestamps of an open file or directory.
        /// Note: This is similar to `futimens` in POSIX.
        pub fn fd_filestat_set_times(arg0: i32, arg1: i64, arg2: i64, arg3: i32) -> i32;
        /// Read from a file descriptor, without using and updating the file descriptor's offset.
        /// Note: This is similar to `preadv` in POSIX.
        pub fn fd_pread(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i32) -> i32;
        /// Return a description of the given preopened file descriptor.
        pub fn fd_prestat_get(arg0: i32, arg1: i32) -> i32;
        /// Return a description of the given preopened file descriptor.
        pub fn fd_prestat_dir_name(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Write to a file descriptor, without using and updating the file descriptor's offset.
        /// Note: This is similar to `pwritev` in POSIX.
        pub fn fd_pwrite(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i32) -> i32;
        /// Read from a file descriptor.
        /// Note: This is similar to `readv` in POSIX.
        pub fn fd_read(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Read directory entries from a directory.
        /// When successful, the contents of the output buffer consist of a sequence of
        /// directory entries. Each directory entry consists of a `dirent` object,
        /// followed by `dirent::d_namlen` bytes holding the name of the directory
        /// entry.
        /// This function fills the output buffer as much as possible, potentially
        /// truncating the last directory entry. This allows the caller to grow its
        /// read buffer size in case it's too small to fit a single large directory
        /// entry, or skip the oversized directory entry.
        pub fn fd_readdir(arg0: i32, arg1: i32, arg2: i32, arg3: i64, arg4: i32) -> i32;
        /// Atomically replace a file descriptor by renumbering another file descriptor.
        /// Due to the strong focus on thread safety, this environment does not provide
        /// a mechanism to duplicate or renumber a file descriptor to an arbitrary
        /// number, like `dup2()`. This would be prone to race conditions, as an actual
        /// file descriptor with the same number could be allocated by a different
        /// thread at the same time.
        /// This function provides a way to atomically renumber file descriptors, which
        /// would disappear if `dup2()` were to be removed entirely.
        pub fn fd_renumber(arg0: i32, arg1: i32) -> i32;
        /// Move the offset of a file descriptor.
        /// Note: This is similar to `lseek` in POSIX.
        pub fn fd_seek(arg0: i32, arg1: i64, arg2: i32, arg3: i32) -> i32;
        /// Synchronize the data and metadata of a file to disk.
        /// Note: This is similar to `fsync` in POSIX.
        pub fn fd_sync(arg0: i32) -> i32;
        /// Return the current offset of a file descriptor.
        /// Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
        pub fn fd_tell(arg0: i32, arg1: i32) -> i32;
        /// Write to a file descriptor.
        /// Note: This is similar to `writev` in POSIX.
        pub fn fd_write(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        /// Create a directory.
        /// Note: This is similar to `mkdirat` in POSIX.
        pub fn path_create_directory(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Return the attributes of a file or directory.
        /// Note: This is similar to `stat` in POSIX.
        pub fn path_filestat_get(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Adjust the timestamps of a file or directory.
        /// Note: This is similar to `utimensat` in POSIX.
        pub fn path_filestat_set_times(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i64,
            arg5: i64,
            arg6: i32,
        ) -> i32;
        /// Create a hard link.
        /// Note: This is similar to `linkat` in POSIX.
        pub fn path_link(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
        ) -> i32;
        /// Open a file or directory.
        /// The returned file descriptor is not guaranteed to be the lowest-numbered
        /// file descriptor not currently open; it is randomized to prevent
        /// applications from depending on making assumptions about indexes, since this
        /// is error-prone in multi-threaded contexts. The returned file descriptor is
        /// guaranteed to be less than 2**31.
        /// Note: This is similar to `openat` in POSIX.
        pub fn path_open(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i64,
            arg6: i64,
            arg7: i32,
            arg8: i32,
        ) -> i32;
        /// Read the contents of a symbolic link.
        /// Note: This is similar to `readlinkat` in POSIX.
        pub fn path_readlink(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
        ) -> i32;
        /// Remove a directory.
        /// Return `errno::notempty` if the directory is not empty.
        /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
        pub fn path_remove_directory(arg0: i32, arg1: i32, arg2: i32) -> i32;
        /// Rename a file or directory.
        /// Note: This is similar to `renameat` in POSIX.
        pub fn path_rename(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32)
            -> i32;
        /// Create a symbolic link.
        /// Note: This is similar to `symlinkat` in POSIX.
        pub fn path_symlink(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        /// Unlink a file.
        /// Return `errno::isdir` if the path refers to a directory.
        /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
        pub fn path_unlink_file(arg0: i32, arg1: i32, arg2: i32) -> i32;
    }
}

/// Provide file advisory information on a file descriptor.
/// Note: This is similar to `posix_fadvise` in POSIX.
///
/// ## Parameters
///
/// * `offset` - The offset within the file to which the advisory applies.
/// * `len` - The length of the region to which the advisory applies.
/// * `advice` - The advice.
pub unsafe fn fd_advise(
    fd: Fd,
    offset: Filesize,
    len: Filesize,
    advice: Advice,
) -> Result<(), Errno> {
    let ret =
        wasi::fd_advise(fd as i32, offset as i64, len as i64, advice.0 as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Force the allocation of space in a file.
/// Note: This is similar to `posix_fallocate` in POSIX.
///
/// ## Parameters
///
/// * `offset` - The offset at which to start the allocation.
/// * `len` - The length of the area that is allocated.
pub unsafe fn fd_allocate(fd: Fd, offset: Filesize, len: Filesize) -> Result<(), Errno> {
    let ret = wasi::fd_allocate(fd as i32, offset as i64, len as i64);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Close a file descriptor.
/// Note: This is similar to `close` in POSIX.
pub unsafe fn fd_close(fd: Fd) -> Result<(), Errno> {
    let ret = wasi::fd_close(fd as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Synchronize the data of a file to disk.
/// Note: This is similar to `fdatasync` in POSIX.
pub unsafe fn fd_datasync(fd: Fd) -> Result<(), Errno> {
    let ret = wasi::fd_datasync(fd as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Get the attributes of a file descriptor.
/// Note: This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields.
///
/// ## Return
///
/// The buffer where the file descriptor's attributes are stored.
pub unsafe fn fd_fdstat_get(fd: Fd) -> Result<Fdstat, Errno> {
    let mut rp0 = MaybeUninit::<Fdstat>::uninit();
    let ret = wasi::fd_fdstat_get(fd as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Fdstat)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Adjust the flags associated with a file descriptor.
/// Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
///
/// ## Parameters
///
/// * `flags` - The desired values of the file descriptor flags.
pub unsafe fn fd_fdstat_set_flags(fd: Fd, flags: Fdflags) -> Result<(), Errno> {
    let ret = wasi::fd_fdstat_set_flags(fd as i32, flags as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Adjust the rights associated with a file descriptor.
/// This can only be used to remove rights, and returns `errno::notcapable` if called in a way that would attempt to add rights
///
/// ## Parameters
///
/// * `fs_rights_base` - The desired rights of the file descriptor.
pub unsafe fn fd_fdstat_set_rights(
    fd: Fd,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
) -> Result<(), Errno> {
    let ret = wasi::fd_fdstat_set_rights(
        fd as i32,
        fs_rights_base as i64,
        fs_rights_inheriting as i64,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Return the attributes of an open file.
///
/// ## Return
///
/// The buffer where the file's attributes are stored.
pub unsafe fn fd_filestat_get(fd: Fd) -> Result<Filestat, Errno> {
    let mut rp0 = MaybeUninit::<Filestat>::uninit();
    let ret = wasi::fd_filestat_get(fd as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Filestat)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros.
/// Note: This is similar to `ftruncate` in POSIX.
///
/// ## Parameters
///
/// * `size` - The desired file size.
pub unsafe fn fd_filestat_set_size(fd: Fd, size: Filesize) -> Result<(), Errno> {
    let ret = wasi::fd_filestat_set_size(fd as i32, size as i64);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Adjust the timestamps of an open file or directory.
/// Note: This is similar to `futimens` in POSIX.
///
/// ## Parameters
///
/// * `atim` - The desired values of the data access timestamp.
/// * `mtim` - The desired values of the data modification timestamp.
/// * `fst_flags` - A bitmask indicating which timestamps to adjust.
pub unsafe fn fd_filestat_set_times(
    fd: Fd,
    atim: Timestamp,
    mtim: Timestamp,
    fst_flags: Fstflags,
) -> Result<(), Errno> {
    let ret = wasi::fd_filestat_set_times(
        fd as i32,
        atim as i64,
        mtim as i64,
        fst_flags as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Read from a file descriptor, without using and updating the file descriptor's offset.
/// Note: This is similar to `preadv` in POSIX.
///
/// ## Parameters
///
/// * `iovs` - List of scatter/gather vectors in which to store data.
/// * `offset` - The offset within the file at which to read.
///
/// ## Return
///
/// The number of bytes read.
pub unsafe fn fd_pread(fd: Fd, iovs: IovecArray<'_>, offset: Filesize) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::fd_pread(
        fd as i32,
        iovs.as_ptr() as i32,
        iovs.len() as i32,
        offset as i64,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Return a description of the given preopened file descriptor.
///
/// ## Return
///
/// The buffer where the description is stored.
pub unsafe fn fd_prestat_get(fd: Fd) -> Result<Prestat, Errno> {
    let mut rp0 = MaybeUninit::<Prestat>::uninit();
    let ret = wasi::fd_prestat_get(fd as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Prestat)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Return a description of the given preopened file descriptor.
///
/// ## Parameters
///
/// * `path` - A buffer into which to write the preopened directory name.
pub unsafe fn fd_prestat_dir_name(fd: Fd, path: *mut u8, path_len: Size) -> Result<(), Errno> {
    let ret = wasi::fd_prestat_dir_name(fd as i32, path as i32, path_len as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Write to a file descriptor, without using and updating the file descriptor's offset.
/// Note: This is similar to `pwritev` in POSIX.
///
/// ## Parameters
///
/// * `iovs` - List of scatter/gather vectors from which to retrieve data.
/// * `offset` - The offset within the file at which to write.
///
/// ## Return
///
/// The number of bytes written.
pub unsafe fn fd_pwrite(fd: Fd, iovs: CiovecArray<'_>, offset: Filesize) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::fd_pwrite(
        fd as i32,
        iovs.as_ptr() as i32,
        iovs.len() as i32,
        offset as i64,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Read from a file descriptor.
/// Note: This is similar to `readv` in POSIX.
///
/// ## Parameters
///
/// * `iovs` - List of scatter/gather vectors to which to store data.
///
/// ## Return
///
/// The number of bytes read.
pub unsafe fn fd_read(fd: Fd, iovs: IovecArray<'_>) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::fd_read(
        fd as i32,
        iovs.as_ptr() as i32,
        iovs.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Read directory entries from a directory.
/// When successful, the contents of the output buffer consist of a sequence of
/// directory entries. Each directory entry consists of a `dirent` object,
/// followed by `dirent::d_namlen` bytes holding the name of the directory
/// entry.
/// This function fills the output buffer as much as possible, potentially
/// truncating the last directory entry. This allows the caller to grow its
/// read buffer size in case it's too small to fit a single large directory
/// entry, or skip the oversized directory entry.
///
/// ## Parameters
///
/// * `buf` - The buffer where directory entries are stored
/// * `cookie` - The location within the directory to start reading
///
/// ## Return
///
/// The number of bytes stored in the read buffer. If less than the size of the read buffer, the end of the directory has been reached.
pub unsafe fn fd_readdir(
    fd: Fd,
    buf: *mut u8,
    buf_len: Size,
    cookie: Dircookie,
) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::fd_readdir(
        fd as i32,
        buf as i32,
        buf_len as i32,
        cookie as i64,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Atomically replace a file descriptor by renumbering another file descriptor.
/// Due to the strong focus on thread safety, this environment does not provide
/// a mechanism to duplicate or renumber a file descriptor to an arbitrary
/// number, like `dup2()`. This would be prone to race conditions, as an actual
/// file descriptor with the same number could be allocated by a different
/// thread at the same time.
/// This function provides a way to atomically renumber file descriptors, which
/// would disappear if `dup2()` were to be removed entirely.
///
/// ## Parameters
///
/// * `to` - The file descriptor to overwrite.
pub unsafe fn fd_renumber(fd: Fd, to: Fd) -> Result<(), Errno> {
    let ret = wasi::fd_renumber(fd as i32, to as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Move the offset of a file descriptor.
/// Note: This is similar to `lseek` in POSIX.
///
/// ## Parameters
///
/// * `offset` - The number of bytes to move.
/// * `whence` - The base from which the offset is relative.
///
/// ## Return
///
/// The new offset of the file descriptor, relative to the start of the file.
pub unsafe fn fd_seek(fd: Fd, offset: Filedelta, whence: Whence) -> Result<Filesize, Errno> {
    let mut rp0 = MaybeUninit::<Filesize>::uninit();
    let ret = wasi::fd_seek(
        fd as i32,
        offset,
        whence.0 as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Filesize)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Synchronize the data and metadata of a file to disk.
/// Note: This is similar to `fsync` in POSIX.
pub unsafe fn fd_sync(fd: Fd) -> Result<(), Errno> {
    let ret = wasi::fd_sync(fd as i32);
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Return the current offset of a file descriptor.
/// Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
///
/// ## Return
///
/// The current offset of the file descriptor, relative to the start of the file.
pub unsafe fn fd_tell(fd: Fd) -> Result<Filesize, Errno> {
    let mut rp0 = MaybeUninit::<Filesize>::uninit();
    let ret = wasi::fd_tell(fd as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Filesize)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Write to a file descriptor.
/// Note: This is similar to `writev` in POSIX.
///
/// ## Parameters
///
/// * `iovs` - List of scatter/gather vectors from which to retrieve data.
pub unsafe fn fd_write(fd: Fd, iovs: CiovecArray<'_>) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::fd_write(
        fd as i32,
        iovs.as_ptr() as i32,
        iovs.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Create a directory.
/// Note: This is similar to `mkdirat` in POSIX.
///
/// ## Parameters
///
/// * `path` - The path at which to create the directory.
pub unsafe fn path_create_directory(fd: Fd, path: &str) -> Result<(), Errno> {
    let ret = wasi::path_create_directory(
        fd as i32,
        path.as_ptr() as i32,
        path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Return the attributes of a file or directory.
/// Note: This is similar to `stat` in POSIX.
///
/// ## Parameters
///
/// * `flags` - Flags determining the method of how the path is resolved.
/// * `path` - The path of the file or directory to inspect.
///
/// ## Return
///
/// The buffer where the file's attributes are stored.
pub unsafe fn path_filestat_get(fd: Fd, flags: Lookupflags, path: &str) -> Result<Filestat, Errno> {
    let mut rp0 = MaybeUninit::<Filestat>::uninit();
    let ret = wasi::path_filestat_get(
        fd as i32,
        flags as i32,
        path.as_ptr() as i32,
        path.len() as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Filestat)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Adjust the timestamps of a file or directory.
/// Note: This is similar to `utimensat` in POSIX.
///
/// ## Parameters
///
/// * `flags` - Flags determining the method of how the path is resolved.
/// * `path` - The path of the file or directory to operate on.
/// * `atim` - The desired values of the data access timestamp.
/// * `mtim` - The desired values of the data modification timestamp.
/// * `fst_flags` - A bitmask indicating which timestamps to adjust.
pub unsafe fn path_filestat_set_times(
    fd: Fd,
    flags: Lookupflags,
    path: &str,
    atim: Timestamp,
    mtim: Timestamp,
    fst_flags: Fstflags,
) -> Result<(), Errno> {
    let ret = wasi::path_filestat_set_times(
        fd as i32,
        flags as i32,
        path.as_ptr() as i32,
        path.len() as i32,
        atim as i64,
        mtim as i64,
        fst_flags as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Create a hard link.
/// Note: This is similar to `linkat` in POSIX.
///
/// ## Parameters
///
/// * `old_flags` - Flags determining the method of how the path is resolved.
/// * `old_path` - The source path from which to link.
/// * `new_fd` - The working directory at which the resolution of the new path starts.
/// * `new_path` - The destination path at which to create the hard link.
pub unsafe fn path_link(
    old_fd: Fd,
    old_flags: Lookupflags,
    old_path: &str,
    new_fd: Fd,
    new_path: &str,
) -> Result<(), Errno> {
    let ret = wasi::path_link(
        old_fd as i32,
        old_flags as i32,
        old_path.as_ptr() as i32,
        old_path.len() as i32,
        new_fd as i32,
        new_path.as_ptr() as i32,
        new_path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Open a file or directory.
/// The returned file descriptor is not guaranteed to be the lowest-numbered
/// file descriptor not currently open; it is randomized to prevent
/// applications from depending on making assumptions about indexes, since this
/// is error-prone in multi-threaded contexts. The returned file descriptor is
/// guaranteed to be less than 2**31.
/// Note: This is similar to `openat` in POSIX.
///
/// ## Parameters
///
/// * `dirflags` - Flags determining the method of how the path is resolved.
/// * `path` - The relative path of the file or directory to open, relative to the
///   `path_open::fd` directory.
/// * `oflags` - The method by which to open the file.
/// * `fs_rights_base` - The initial rights of the newly created file descriptor. The
///   implementation is allowed to return a file descriptor with fewer rights
///   than specified, if and only if those rights do not apply to the type of
///   file being opened.
///   The *base* rights are rights that will apply to operations using the file
///   descriptor itself, while the *inheriting* rights are rights that apply to
///   file descriptors derived from it.
///
/// ## Return
///
/// The file descriptor of the file that has been opened.
pub unsafe fn path_open(
    fd: Fd,
    dirflags: Lookupflags,
    path: &str,
    oflags: Oflags,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fdflags: Fdflags,
) -> Result<Fd, Errno> {
    let mut rp0 = MaybeUninit::<Fd>::uninit();
    let ret = wasi::path_open(
        fd as i32,
        dirflags as i32,
        path.as_ptr() as i32,
        path.len() as i32,
        oflags as i32,
        fs_rights_base as i64,
        fs_rights_inheriting as i64,
        fdflags as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Fd)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Read the contents of a symbolic link.
/// Note: This is similar to `readlinkat` in POSIX.
///
/// ## Parameters
///
/// * `path` - The path of the symbolic link from which to read.
/// * `buf` - The buffer to which to write the contents of the symbolic link.
///
/// ## Return
///
/// The number of bytes placed in the buffer.
pub unsafe fn path_readlink(
    fd: Fd,
    path: &str,
    buf: *mut u8,
    buf_len: Size,
) -> Result<Size, Errno> {
    let mut rp0 = MaybeUninit::<Size>::uninit();
    let ret = wasi::path_readlink(
        fd as i32,
        path.as_ptr() as i32,
        path.len() as i32,
        buf as i32,
        buf_len as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Size)),
        _ => Err(Errno(ret as u16)),
    }
}

/// Remove a directory.
/// Return `errno::notempty` if the directory is not empty.
/// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
///
/// ## Parameters
///
/// * `path` - The path to a directory to remove.
pub unsafe fn path_remove_directory(fd: Fd, path: &str) -> Result<(), Errno> {
    let ret = wasi::path_remove_directory(
        fd as i32,
        path.as_ptr() as i32,
        path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Rename a file or directory.
/// Note: This is similar to `renameat` in POSIX.
///
/// ## Parameters
///
/// * `old_path` - The source path of the file or directory to rename.
/// * `new_fd` - The working directory at which the resolution of the new path starts.
/// * `new_path` - The destination path to which to rename the file or directory.
pub unsafe fn path_rename(fd: Fd, old_path: &str, new_fd: Fd, new_path: &str) -> Result<(), Errno> {
    let ret = wasi::path_rename(
        fd as i32,
        old_path.as_ptr() as i32,
        old_path.len() as i32,
        new_fd as i32,
        new_path.as_ptr() as i32,
        new_path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Create a symbolic link.
/// Note: This is similar to `symlinkat` in POSIX.
///
/// ## Parameters
///
/// * `old_path` - The contents of the symbolic link.
/// * `new_path` - The destination path at which to create the symbolic link.
pub unsafe fn path_symlink(old_path: &str, fd: Fd, new_path: &str) -> Result<(), Errno> {
    let ret = wasi::path_symlink(
        old_path.as_ptr() as i32,
        old_path.len() as i32,
        fd as i32,
        new_path.as_ptr() as i32,
        new_path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}

/// Unlink a file.
/// Return `errno::isdir` if the path refers to a directory.
/// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
///
/// ## Parameters
///
/// * `path` - The path to a file to unlink.
pub unsafe fn path_unlink_file(fd: Fd, path: &str) -> Result<(), Errno> {
    let ret = wasi::path_unlink_file(
        fd as i32,
        path.as_ptr() as i32,
        path.len() as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(Errno(ret as u16)),
    }
}
