mod flatbuffer {
    include!(concat!(env!("OUT_DIR"), "/footer_generated.rs"));
}

pub(crate) mod Footer {
    pub(crate) use self::super::flatbuffer::minknow::reads_format::{
        Footer,
        FooterArgs,
        FooterBuilder,
        FooterOffset
    };
}
pub(crate) mod EmbeddedFile {
    pub(crate) use self::super::flatbuffer::minknow::reads_format::{
        EmbeddedFile,
        EmbeddedFileArgs,
        EmbeddedFileOffset,
        EmbeddedFileBuilder,
    };
}
pub(crate) use self::flatbuffer::minknow::reads_format::{
    ContentType,
    Format,
};
