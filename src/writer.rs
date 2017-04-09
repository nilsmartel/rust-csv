use std::fs::File;
use std::io;
use std::path::Path;

use csv_core::{
    Writer as CoreWriter, WriterBuilder as CoreWriterBuilder,
    QuoteStyle, Terminator,
};

use byte_record::Position;
use error::Result;

/// Builds a CSV writer with various configuration knobs.
///
/// This builder can be used to tweak the field delimiter, record terminator
/// and more for writing CSV. Once a CSV `Writer` is built, its configuration
/// cannot be changed.
#[derive(Debug)]
pub struct WriterBuilder {
    builder: CoreWriterBuilder,
    capacity: usize,
    flexible: bool,
    has_headers: bool,
}

impl Default for WriterBuilder {
    fn default() -> WriterBuilder {
        WriterBuilder {
            builder: CoreWriterBuilder::default(),
            capacity: 8 * (1<<10),
            flexible: false,
            has_headers: true,
        }
    }
}

impl WriterBuilder {
    /// Create a new builder for configuring CSV writing.
    ///
    /// To convert a builder into a writer, call one of the methods starting
    /// with `from_`.
    pub fn new() -> WriterBuilder {
        WriterBuilder::default()
    }

    /// Build a CSV writer from this configuration that writer data to the
    /// given file path.
    ///
    /// If there was a problem opening the file at the given path, then this
    /// returns the corresponding error.
    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<Writer<File>> {
        Ok(Writer::new(self, File::create(path)?))
    }

    /// Build a CSV writer from this configuration that writes data to `wtr`.
    ///
    /// Note that the CSV writer is buffered automatically, so you should not
    /// wrap `wtr` in a buffered writer like `io::BufWriter`.
    pub fn from_writer<W: io::Write>(&self, wtr: W) -> Writer<W> {
        Writer::new(self, wtr)
    }

    /// The field delimiter to use when writing CSV.
    ///
    /// The default is `b','`.
    pub fn delimiter(&mut self, delimiter: u8) -> &mut WriterBuilder {
        self.builder.delimiter(delimiter);
        self
    }

    /// Whether to write a header row before writing any other row.
    ///
    /// When this is enabled and the `serialize` method is used to write data
    /// with something that contains field names (like a struct or a map), then
    /// a header row is written containing the field names before any other
    /// row is written.
    ///
    /// This option has no effect when using other methods to write rows. That
    /// is, if you don't use `serialize`, then you must write your header row
    /// explicitly if you want it.
    pub fn has_headers(&mut self, yes: bool) -> &mut WriterBuilder {
        self.has_headers = yes;
        self
    }

    /// Whether the number of fields in records is allowed to change or not.
    ///
    /// When disabled (which is the default), writing CSV data will return an
    /// error if a record is written with a number of fields different from the
    /// number of fields written in a previous record.
    ///
    /// When enabled, this error checking is turned off.
    pub fn flexible(&mut self, yes: bool) -> &mut WriterBuilder {
        self.flexible = yes;
        self
    }

    /// The record terminator to use when writing CSV.
    ///
    /// A record terminator can be any single byte. The default is a special
    /// value, `Terminator::CRLF`, which treats any occurrence of `\r`, `\n`
    /// or `\r\n` as a single record terminator.
    pub fn terminator(
        &mut self,
        term: Terminator,
    ) -> &mut WriterBuilder {
        self.builder.terminator(term);
        self
    }

    /// The quoting style to use when writing CSV.
    ///
    /// By default, this is set to `QuoteStyle::Necessary`, which will only
    /// use quotes when they are necessary to preserve the integrity of data.
    ///
    /// Note that regardless of this setting, an empty field is quoted if it is
    /// the only field in a record.
    pub fn quote_style(&mut self, style: QuoteStyle) -> &mut WriterBuilder {
        self.builder.quote_style(style);
        self
    }

    /// The quote character to use when writing CSV.
    ///
    /// The default is `b'"'`.
    pub fn quote(&mut self, quote: u8) -> &mut WriterBuilder {
        self.builder.quote(quote);
        self
    }

    /// The escape character to use when writing CSV.
    ///
    /// In some variants of CSV, quotes are escaped using a special escape
    /// character like `\` (instead of escaping quotes by doubling them).
    ///
    /// By default, writing these idiosyncratic escapes is disabled, and is
    /// only used when `double_quote` is disabled.
    pub fn escape(&mut self, escape: u8) -> &mut WriterBuilder {
        self.builder.escape(escape);
        self
    }

    /// Enable double quote escapes.
    ///
    /// This is enabled by default, but it may be disabled. When disabled,
    /// quotes in field data are escaped instead of doubled.
    pub fn double_quote(&mut self, yes: bool) -> &mut WriterBuilder {
        self.builder.double_quote(yes);
        self
    }

    /// Set the capacity (in bytes) of the buffer used in the CSV writer.
    pub fn buffer_capacity(&mut self, capacity: usize) -> &mut WriterBuilder {
        self.capacity = capacity;
        self
    }
}

#[derive(Debug)]
pub struct Writer<W: io::Write> {
    core: CoreWriter,
    wtr: io::BufWriter<W>,
    state: WriterState,
}

#[derive(Debug)]
struct WriterState {
    flexible: bool,
    has_headers: bool,
    fields_written: u64,
}

impl<W: io::Write> Writer<W> {
    fn new(builder: &WriterBuilder, wtr: W) -> Writer<W> {
        Writer {
            core: builder.builder.build(),
            wtr: io::BufWriter::with_capacity(builder.capacity, wtr),
            state: WriterState {
                flexible: builder.flexible,
                has_headers: builder.has_headers,
                fields_written: 0,
            },
        }
    }

    /// Write a single field.
    ///
    /// One should prefer using `write_record` over this method. It is provided
    /// for cases where writing a field at a time is more convenient than
    /// writing a record at a time.
    ///
    /// Note that if this API is used, `write_record` should be called with an
    /// empty iterator to write a record terminator.
    pub fn write_field<T: AsRef<[u8]>>(field: T) -> Result<()> {
        // if self.state.fields_written > 0 {
        // }
        unimplemented!()
    }
}
