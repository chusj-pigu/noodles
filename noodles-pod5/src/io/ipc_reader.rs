// standard
use std::{
    fmt::{
        self,
        Formatter,
        Display,
    },
    error::Error,
    collections::HashMap,
};
// third party
use arrow::datatypes::ArrowNativeType;
use arrow::array::{RecordBatch};
use arrow::error::{ArrowError};
use arrow::buffer::Buffer;
use arrow::ipc::convert::fb_to_schema;
use arrow::ipc::reader::{FileDecoder, read_footer_length};
use arrow::ipc::{Block, root_as_footer};
use std::sync::Arc;
// local
use crate::record::batch::{BatchIndex, BatchRange};


#[derive(Debug)]
/// Error raised when parsing an IPC file to create a [`IPCReader`].
pub(crate) enum IPCReaderError {
    /// Error during parsing from a string.
    ParseError(String),

    /// Error during IPC operations in `arrow-ipc` or `arrow-flight`.
    IpcError(String),

    /// Unhandled or unknown errors.
    Unknown(ArrowError),
}

impl Display for IPCReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IPCReaderError::ParseError(message) => write!(f, "parsing error: {}", message),
            IPCReaderError::IpcError(message) => write!(f, "IPC error: {}", message),
            IPCReaderError::Unknown(error) => write!(f, "unexpected arrow error: {}", error),
        }
    }
}

impl Error for IPCReaderError {}

/// Incrementally decodes [`RecordBatches`](RecordBatch) from a memory-mapped IPC file stored in a Arrow
/// [`Buffer`] using the [`FileDecoder`] API.
pub(crate) struct IPCReader {
    /// Memory mapped Buffer with the data
    buffer: Buffer,

    /// The decoder
    decoder: FileDecoder,

    /// The blocks in the file
    ///
    /// A block indicates the regions in the file to read to get data
    blocks: Vec<Block>,

    /// The total number of blocks, which may contain record batches and other types
    total_blocks: usize,

    /// User defined metadata
    custom_metadata: HashMap<String, String>,
}

impl IPCReader {
    pub(crate) fn new (buf: Buffer) -> Result<Self, IPCReaderError> {
        Self::new_with_projection(buf, None)
    }
    pub(crate) fn new_with_projection (buffer: Buffer, projection: Option<Vec<usize>>) -> Result<Self, IPCReaderError> {
        // Space for ARROW_MAGIC (6 bytes) and length (4 bytes)
        let trailer_start = match buffer.len().checked_sub(10) {
            Some(start) => start,
            None => return Err(IPCReaderError::ParseError(String::from("buffer is too small"))),
        };
        let trailer_buf: [u8; 10] = buffer[trailer_start..].try_into().unwrap();

        let footer_len = match read_footer_length(trailer_buf) {
            Ok(len) => len,
            Err(ArrowError::ParseError(message)) => return Err(IPCReaderError::ParseError(message)),
            Err(error) => return Err(IPCReaderError::Unknown(error)),
        };
        let footer_start = match trailer_start.checked_sub(footer_len) {
            Some(start) => start,
            None => return Err(IPCReaderError::ParseError(String::from("buffer is too small"))),
        };

        // read footer
        let footer_buf: &[u8] = &buffer[footer_start..trailer_start];
        let footer = root_as_footer(footer_buf).map_err(
            |err| IPCReaderError::ParseError(format!("unable to get root as footer: {err:?}")),
        )?;

        let blocks = footer.recordBatches().ok_or_else(|| {
            IPCReaderError::ParseError(String::from("unable to get record batches from IPC Footer"))
        })?;

        let total_blocks = blocks.len();

        let blocks = blocks.iter().copied().collect();

        let ipc_schema = footer.schema().ok_or_else(|| {
            IPCReaderError::ParseError(String::from("unable to get schema from IPC Footer"))
        })?;
        if !ipc_schema.endianness().equals_to_target_endianness() {
            return Err(IPCReaderError::IpcError(
                String::from("the endianness of the source system does not match the endianness of the target system.")
            ));
        }

        let schema = fb_to_schema(ipc_schema);

        let mut custom_metadata = HashMap::new();
        if let Some(fb_custom_metadata) = footer.custom_metadata() {
            for kv in fb_custom_metadata.into_iter() {
                custom_metadata.insert(
                    kv.key().unwrap().to_string(),
                    kv.value().unwrap().to_string(),
                );
            }
        }

        let mut decoder = FileDecoder::new(Arc::new(schema), footer.version());
        if let Some(projection) = projection {
            decoder = decoder.with_projection(projection)
        }

        // Create an array of optional dictionary value arrays, one per field.
        if let Some(dictionaries) = footer.dictionaries() {
            for block in dictionaries {
                let body_len = block.bodyLength().to_usize().unwrap();
                let metadata_len = block.metaDataLength().to_usize().unwrap();
                let total_len = body_len.checked_add(metadata_len).unwrap();

                let block_buf = buffer.slice_with_length(
                    block.offset() as usize,
                    total_len,
                );

                match decoder.read_dictionary(block, &block_buf) {
                    Ok(()) => {},
                    Err(ArrowError::ParseError(message)) => return Err(IPCReaderError::ParseError(message)),
                    Err(ArrowError::IpcError(message)) => return Err(IPCReaderError::IpcError(message)),
                    Err(error) => return Err(IPCReaderError::Unknown(error)),
                }
            }
        }

        Ok(IPCReader {
            buffer,
            blocks,
            total_blocks,
            decoder,
            custom_metadata,
        })
    }

    /// Return the number of [`RecordBatch`]es in this buffer
    pub(crate) fn num_batches(&self) -> usize {
        self.total_blocks
    }

    /// Return the [`RecordBatch`] at `batch_index`.
    ///
    /// This may return `None` if the IPC message was None
    pub(crate) fn get_batch_range(&self, batch_index: BatchIndex) -> BatchRange {
        let block = &self.blocks[Into::<usize>::into(batch_index)];
        let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
        let start = block.offset() as usize;
        BatchRange::new(start, block_len)
    }

    /// Return the [`RecordBatch`] at `batch_index`.
    ///
    /// This may return `None` if the IPC message was None
    pub(crate) fn get_batch(&self, batch_index: BatchIndex) -> Result<Option<RecordBatch>, ArrowError> {
        let block = &self.blocks[Into::<usize>::into(batch_index)];
        let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
        let data = self
            .buffer
            .slice_with_length(block.offset() as _, block_len);
        self.decoder.read_record_batch(block, &data)
    }
}