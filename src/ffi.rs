#![allow(non_camel_case_types, dead_code)]

use libc::{c_char, c_int, c_uint, c_ulong, c_void};

pub const CL_INIT_DEFAULT: u32 = 0x0;

pub type cl_engine = c_void;

// :libclamav.so.7 as opposed to clamav as libclamav.so may not exist
#[link(name = ":libclamav.so.7")]
extern "C" {
    pub fn cl_init(initOptions: c_uint) -> cl_error;
    pub fn cl_strerror(clerror: c_int) -> *const c_char;

    // engine
    pub fn cl_engine_new() -> *mut cl_engine;
    pub fn cl_engine_free(engine: *mut cl_engine) -> cl_error;
    pub fn cl_engine_compile(engine: *mut cl_engine) -> cl_error;

    // database
    pub fn cl_load(
        path: *const c_char,
        engine: *mut cl_engine,
        signo: *mut c_uint,
        dboptions: c_uint,
    ) -> cl_error;
    pub fn cl_retdbdir() -> *const c_char;

    // scanning
    pub fn cl_scanfile(
        filename: *const c_char,
        virname: *mut *const c_char,
        scanned: *mut c_ulong,
        engine: *const cl_engine,
        scanoptions: c_uint,
    ) -> cl_error;

    // version
    pub fn cl_retflevel() -> c_uint;
    pub fn cl_retver() -> *const c_char;
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum cl_error {
    /* libclamav specific */
    /* CL_CLEAN = 0, */
    CL_SUCCESS = 0,
    CL_VIRUS,
    CL_ENULLARG,
    CL_EARG,
    CL_EMALFDB,
    CL_ECVD,
    CL_EVERIFY,
    CL_EUNPACK,

    /* I/O and memory errors */
    CL_EOPEN,
    CL_ECREAT,
    CL_EUNLINK,
    CL_ESTAT,
    CL_EREAD,
    CL_ESEEK,
    CL_EWRITE,
    CL_EDUP,
    CL_EACCES,
    CL_ETMPFILE,
    CL_ETMPDIR,
    CL_EMAP,
    CL_EMEM,
    CL_ETIMEOUT,

    /* internal (not reported outside libclamav) */
    CL_BREAK,
    CL_EMAXREC,
    CL_EMAXSIZE,
    CL_EMAXFILES,
    CL_EFORMAT,
    CL_EPARSE,
    CL_EBYTECODE,          /* may be reported in testmode */
    CL_EBYTECODE_TESTFAIL, /* may be reported in testmode */

    /* c4w error codes */
    CL_ELOCK,
    CL_EBUSY,
    CL_ESTATE,

    /* no error codes below this line please */
    CL_ELAST_ERROR,
}

impl cl_error {
    // CL_CLEAN has the same value as CL_SUCCESS, which Rust does not yet support
    pub const CL_CLEAN: cl_error = cl_error::CL_SUCCESS;
}

/* db settings */
pub const CL_DB_PHISHING: c_uint = 0x2;
pub const CL_DB_PHISHING_URLS: c_uint = 0x8;
pub const CL_DB_PUA: c_uint = 0x10;
pub const CL_DB_CVDNOTMP: c_uint = 0x20;
pub const CL_DB_OFFICIAL: c_uint = 0x40;
pub const CL_DB_PUA_MODE: c_uint = 0x80;
pub const CL_DB_PUA_INCLUDE: c_uint = 0x100;
pub const CL_DB_PUA_EXCLUDE: c_uint = 0x200;
pub const CL_DB_COMPILED: c_uint = 0x400;
pub const CL_DB_DIRECTORY: c_uint = 0x800;
pub const CL_DB_OFFICIAL_ONLY: c_uint = 0x1000;
pub const CL_DB_BYTECODE: c_uint = 0x2000;
pub const CL_DB_SIGNED: c_uint = 0x4000;
pub const CL_DB_BYTECODE_UNSIGNED: c_uint = 0x8000;
pub const CL_DB_UNSIGNED: c_uint = 0x10000;
pub const CL_DB_BYTECODE_STATS: c_uint = 0x20000;
pub const CL_DB_ENHANCED: c_uint = 0x40000;
pub const CL_DB_PCRE_STATS: c_uint = 0x80000;
pub const CL_DB_YARA_EXCLUDE: c_uint = 0x100000;
pub const CL_DB_YARA_ONLY: c_uint = 0x200000;

/* recommended db settings */
pub const CL_DB_STDOPT: c_uint = CL_DB_PHISHING | CL_DB_PHISHING_URLS | CL_DB_BYTECODE;

/* scan options */
pub const CL_SCAN_RAW: c_uint = 0x0;
pub const CL_SCAN_ARCHIVE: c_uint = 0x1;
pub const CL_SCAN_MAIL: c_uint = 0x2;
pub const CL_SCAN_OLE2: c_uint = 0x4;
pub const CL_SCAN_BLOCKENCRYPTED: c_uint = 0x8;
pub const CL_SCAN_HTML: c_uint = 0x10;
pub const CL_SCAN_PE: c_uint = 0x20;
pub const CL_SCAN_BLOCKBROKEN: c_uint = 0x40;
pub const CL_SCAN_MAILURL: c_uint = 0x80;
pub const CL_SCAN_BLOCKMAX: c_uint = 0x100;
pub const CL_SCAN_ALGORITHMIC: c_uint = 0x200;
pub const CL_SCAN_PHISHING_BLOCKSSL: c_uint = 0x800;
pub const CL_SCAN_PHISHING_BLOCKCLOAK: c_uint = 0x1000;
pub const CL_SCAN_ELF: c_uint = 0x2000;
pub const CL_SCAN_PDF: c_uint = 0x4000;
pub const CL_SCAN_STRUCTURED: c_uint = 0x8000;
pub const CL_SCAN_STRUCTURED_SSN_NORMAL: c_uint = 0x10000;
pub const CL_SCAN_STRUCTURED_SSN_STRIPPED: c_uint = 0x20000;
pub const CL_SCAN_PARTIAL_MESSAGE: c_uint = 0x40000;
pub const CL_SCAN_HEURISTIC_PRECEDENCE: c_uint = 0x80000;
pub const CL_SCAN_BLOCKMACROS: c_uint = 0x100000;
pub const CL_SCAN_ALLMATCHES: c_uint = 0x200000;
pub const CL_SCAN_SWF: c_uint = 0x400000;
pub const CL_SCAN_PARTITION_INTXN: c_uint = 0x800000;
pub const CL_SCAN_XMLDOCS: c_uint = 0x1000000;
pub const CL_SCAN_HWP3: c_uint = 0x2000000;
pub const CL_SCAN_FILE_PROPERTIES: c_uint = 0x10000000;
pub const CL_SCAN_PERFORMANCE_INFO: c_uint = 0x40000000;
pub const CL_SCAN_INTERNAL_COLLECT_SHA: c_uint = 0x80000000;

/* recommended scan settings */
pub const CL_SCAN_STDOPT: c_uint = CL_SCAN_ARCHIVE | CL_SCAN_MAIL | CL_SCAN_OLE2 | CL_SCAN_PDF
    | CL_SCAN_HTML | CL_SCAN_PE | CL_SCAN_ALGORITHMIC
    | CL_SCAN_ELF | CL_SCAN_SWF | CL_SCAN_XMLDOCS | CL_SCAN_HWP3;
