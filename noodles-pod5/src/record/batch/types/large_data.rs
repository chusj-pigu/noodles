/// Abstracts over the actual form of the signal data, be it compressed or not.
pub enum LargeData<'a> {
    /// The raw form of the data, directly readable.
    Raw(&'a [i16]),

    /// The compressed, unreadable form of the data.
    VBZ(&'a [u8]),
}