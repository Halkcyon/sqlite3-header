// https://sqlite.org/fileformat2.html

pub mod error;

use std::convert::TryInto;

/// The C string "SQLite format 3\000"
const MAGIC_HEADER_BYTES: [u8; 16] = [
    0x53, 0x51, 0x4c, 0x69,
    0x74, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x20, 0x33, 0x00,
];

fn two_byte_slice_to_u16(slice: &[u8]) -> u16 {
    u16::from_be_bytes(slice.try_into().unwrap())
}

fn four_byte_slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes(slice.try_into().unwrap())
}

/// The file format write version and file format read version at offsets 18 and 19
/// are intended to allow for enhancements of the file format in future versions of
/// SQLite. In current versions of SQLite, both of these values are 1 for rollback
/// journalling modes and 2 for WAL journalling mode. If a version of SQLite coded
/// to the current file format specification encounters a database file where the
/// read version is 1 or 2 but the write version is greater than 2, then the
/// database file must be treated as read-only. If a database file with a read
/// version greater than 2 is encountered, then that database cannot be read or
/// written.
#[derive(Debug)]
pub enum FileFormat {
    Inaccessible,
    Legacy,
    WriteAheadLogging,
}

/// The maximum and minimum embedded payload fractions and the leaf payload
/// fraction values must be 64, 32, and 32. These values were originally intended
/// to be tunable parameters that could be used to modify the storage format of the
/// b-tree algorithm. However, that functionality is not supported and there are no
/// current plans to add support in the future. Hence, these three bytes are fixed
/// at the values specified.
#[derive(Debug)]
pub struct Payload {
    pub leaf_fraction: u8,
    pub maximum_embedded_fraction: u8,
    pub minimum_embedded_fraction: u8,
}

/// Unused pages in the database file are stored on a freelist. The 4-byte
/// big-endian integer at offset 32 stores the page number of the first page of the
/// freelist, or zero if the freelist is empty. The 4-byte big-endian integer at
/// offset 36 stores stores the total number of pages on the freelist.
#[derive(Debug)]
pub struct Freelist {
    pub page_index: u32,
    pub count: u32,
}

/// The schema format number is a 4-byte big-endian integer at offset 44. The
/// schema format number is similar to the file format read and write version
/// numbers at offsets 18 and 19 except that the schema format number refers to the
/// high-level SQL formatting rather than the low-level b-tree formatting. Four
/// schema format numbers are currently defined:
/// 1. Format 1 is understood by all versions of SQLite back to version 3.0.0 (2004-06-18).
/// 2. Format 2 adds the ability of rows within the same table to have a varying number of columns, in order to support the ALTER TABLE ... ADD COLUMN functionality. Support for reading and writing format 2 was added in SQLite version 3.1.3 on 2005-02-20.
/// 3. Format 3 adds the ability of extra columns added by ALTER TABLE ... ADD COLUMN to have non-NULL default values. This capability was added in SQLite version 3.1.4 on 2005-03-11.
/// 4. Format 4 causes SQLite to respect the DESC keyword on index declarations. (The DESC keyword is ignored in indexes for formats 1, 2, and 3.) Format 4 also adds two new boolean record type values (serial types 8 and 9). Support for format 4 was added in SQLite 3.3.0 on 2006-01-10.
/// New database files created by SQLite use format 4 by default. The
/// legacy_file_format pragma can be used to cause SQLite to create new database
/// files using format 1. The format version number can be made to default to 1
/// instead of 4 by setting SQLITE_DEFAULT_FILE_FORMAT=1 at compile-time.
#[derive(Debug)]
pub enum SchemaFormat {
    Format1,
    Format2,
    Format3,
    Format4,
}

#[derive(Debug)]
pub struct Schema {
    /// The schema cookie is a 4-byte big-endian integer at offset 40 that is
    /// incremented whenever the database schema changes. A prepared statement is
    /// compiled against a specific version of the database schema. When the database
    /// schema changes, the statement must be reprepared. When a prepared statement
    /// runs, it first checks the schema cookie to ensure the value is the same as when
    /// the statement was prepared and if the schema cookie has changed, the statement
    /// either automatically reprepares and reruns or it aborts with an SQLITE_SCHEMA
    /// error.
    pub cookie: u32,
    pub format: SchemaFormat,
}

/// The 4-byte big-endian integer at offset 56 determines the encoding used for all text strings
/// stored in the database. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means
/// UTF-16be. No other values are allowed. The sqlite3.h header file defines C-preprocessor macros
/// SQLITE_UTF8 as 1, SQLITE_UTF16LE as 2, and SQLITE_UTF16BE as 3, to use in place of the numeric
/// codes for the text encoding.
#[derive(Debug)]
pub enum DatabaseTextEncoding {
    Utf8,
    Utf16le,
    Utf16be,
}

#[derive(Debug)]
pub enum VacuumMode {
    Auto,
    Incremental,
}

/// The two 4-byte big-endian integers at offsets 52 and 64 are used to manage the
/// auto_vacuum and incremental_vacuum modes. If the integer at offset 52 is zero
/// then pointer-map (ptrmap) pages are omitted from the database file and neither
/// auto_vacuum nor incremental_vacuum are supported. If the integer at offset 52 is
/// non-zero then it is the page number of the largest root page in the database
/// file, the database file will contain ptrmap pages, and the mode must be either
/// auto_vacuum or incremental_vacuum. In this latter case, the integer at offset 64
/// is true for incremental_vacuum and false for auto_vacuum. If the integer at
/// offset 52 is zero then the integer at offset 64 must also be zero.
#[derive(Debug)]
pub struct Vacuum {
    pub largest_root_btree_page: u32,
    pub mode: VacuumMode,
}

/// The 4-byte big-endian integer at offset 96 stores the SQLITE_VERSION_NUMBER
/// value for the SQLite library that most recently modified the database file. The
/// 4-byte big-endian integer at offset 92 is the value of the change counter when
/// the version number was stored. The integer at offset 92 indicates which
/// transaction the version number is valid for and is sometimes called the
/// "version-valid-for number".
#[derive(Debug)]
pub struct LastUpdate {
    pub sqlite_version_number: u32,
    pub version_valid_for: u32,
}

#[derive(Debug)]
pub struct SQLite3Header {
    page_size: u16,

    file_format_write_version: FileFormat,
    file_format_read_version: FileFormat,

    reserved_bytes_per_page: u8,

    payload_fraction: Payload,

    file_change_counter: u32,

    in_header_database_size: u32,

    freelist: Freelist,

    schema: Schema,

    default_page_cache_size: u32,

    database_text_encoding: DatabaseTextEncoding,

    user_version: u32,

    vacuum: Option<Vacuum>,

    application_id: u32,

    last_update: LastUpdate,
}

impl SQLite3Header {
    /// All other bytes of the database file header are reserved for future expansion
    /// and must be set to zero.
    #[allow(non_upper_case_globals)]
    pub const reserved: [u8; 20] = [0; 20];

    /// Every valid SQLite database file begins with the following 16 bytes (in hex):
    /// 53 51 4c 69 74 65 20 66 6f 72 6d 61 74 20 33 00. This byte sequence corresponds
    /// to the UTF-8 string "SQLite format 3" including the nul terminator character at
    /// the end.
    pub fn magic_header_string(&self) -> &str {
        std::str::from_utf8(&MAGIC_HEADER_BYTES).unwrap()
    }

    /// The two-byte value beginning at offset 16 determines the page size of the
    /// database. For SQLite versions 3.7.0.1 (2010-08-04) and earlier, this value is
    /// interpreted as a big-endian integer and must be a power of two between 512 and
    /// 32768, inclusive. Beginning with SQLite version 3.7.1 (2010-08-23), a page size
    /// of 65536 bytes is supported. The value 65536 will not fit in a two-byte
    /// integer, so to specify a 65536-byte page size, the value at offset 16 is 0x00
    /// 0x01. This value can be interpreted as a big-endian 1 and thought of as a magic
    /// number to represent the 65536 page size. Or one can view the two-byte field as
    /// a little endian number and say that it represents the page size divided by 256.
    /// These two interpretations of the page-size field are equivalent.
    pub fn page_size(&self) -> u16 {
        self.page_size
    }

    pub fn file_format_read_version(&self) -> &FileFormat {
        &self.file_format_read_version
    }

    pub fn file_format_write_version(&self) -> &FileFormat {
        &self.file_format_write_version
    }

    /// SQLite has the ability to set aside a small number of extra bytes at the end of
    /// every page for use by extensions. These extra bytes are used, for example, by
    /// the SQLite Encryption Extension to store a nonce and/or cryptographic checksum
    /// associated with each page. The "reserved space" size in the 1-byte integer at
    /// offset 20 is the number of bytes of space at the end of each page to reserve
    /// for extensions. This value is usually 0. The value can be odd.

    /// The "usable size" of a database page is the page size specified by the 2-byte
    /// integer at offset 16 in the header less the "reserved" space size recorded in
    /// the 1-byte integer at offset 20 in the header. The usable size of a page might
    /// be an odd number. However, the usable size is not allowed to be less than 480.
    /// In other words, if the page size is 512, then the reserved space size cannot
    /// exceed 32.
    pub fn reserved_bytes_per_page(&self) -> u8 {
        self.reserved_bytes_per_page
    }

    pub fn payload_fraction(&self) -> &Payload {
        &self.payload_fraction
    }

    /// The file change counter is a 4-byte big-endian integer at offset 24 that is
    /// incremented whenever the database file is unlocked after having been modified.
    /// When two or more processes are reading the same database file, each process can
    /// detect database changes from other processes by monitoring the change counter.
    /// A process will normally want to flush its database page cache when another
    /// process modified the database, since the cache has become stale. The file
    /// change counter facilitates this.

    /// In WAL mode, changes to the database are detected using the wal-index and so
    /// the change counter is not needed. Hence, the change counter might not be
    /// incremented on each transaction in WAL mode.
    pub fn file_change_counter(&self) -> u32 {
        self.file_change_counter
    }

    /// The 4-byte big-endian integer at offset 28 into the header stores the size of
    /// the database file in pages. If this in-header datasize size is not valid (see
    /// the next paragraph), then the database size is computed by looking at the
    /// actual size of the database file. Older versions of SQLite ignored the
    /// in-header database size and used the actual file size exclusively. Newer
    /// versions of SQLite use the in-header database size if it is available but fall
    /// back to the actual file size if the in-header database size is not valid.

    /// The in-header database size is only considered to be valid if it is non-zero
    /// and if the 4-byte change counter at offset 24 exactly matches the 4-byte
    /// version-valid-for number at offset 92. The in-header database size is always
    /// valid when the database is only modified using recent versions of SQLite,
    /// versions 3.7.0 (2010-07-21) and later. If a legacy version of SQLite writes to
    /// the database, it will not know to update the in-header database size and so the
    /// in-header database size could be incorrect. But legacy versions of SQLite will
    /// also leave the version-valid-for number at offset 92 unchanged so it will not
    /// match the change-counter. Hence, invalid in-header database sizes can be
    /// detected (and ignored) by observing when the change-counter does not match the
    /// version-valid-for number.
    pub fn in_header_database_size(&self) -> u32 {
        self.in_header_database_size
    }

    pub fn freelist(&self) -> &Freelist {
        &self.freelist
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// The 4-byte big-endian signed integer at offset 48 is the suggested cache size
    /// in pages for the database file. The value is a suggestion only and SQLite is
    /// under no obligation to honor it. The absolute value of the integer is used as
    /// the suggested size. The suggested cache size can be set using the
    /// default_cache_size pragma.
    pub fn default_page_cache_size(&self) -> u32 {
        self.default_page_cache_size
    }

    pub fn database_text_encoding(&self) -> &DatabaseTextEncoding {
        &self.database_text_encoding
    }

    /// The 4-byte big-endian integer at offset 60 is the user version which is set and
    /// queried by the user_version pragma. The user version is not used by SQLite.
    pub fn user_version(&self) -> u32 {
        self.user_version
    }

    pub fn vacuum(&self) -> Option<&Vacuum> {
        if let Some(vac) = &self.vacuum {
            Some(vac)
        } else {
            None
        }
    }

    /// The 4-byte big-endian integer at offset 68 is an "Application ID" that can be
    /// set by the PRAGMA application_id command in order to identify the database as
    /// belonging to or associated with a particular application. The application ID is
    /// intended for database files used as an application file-format. The application
    /// ID can be used by utilities such as file(1) to determine the specific file type
    /// rather than just reporting "SQLite3 Database". A list of assigned application
    /// IDs can be seen by consulting the magic.txt file in the SQLite source repository.
    pub fn application_id(&self) -> u32 {
        self.application_id
    }

    pub fn last_update(&self) -> &LastUpdate {
        &self.last_update
    }
}
