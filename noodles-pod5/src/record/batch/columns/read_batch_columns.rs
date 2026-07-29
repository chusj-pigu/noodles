use arrow::array::ArrayRef;
use crate::record::batch::arrays::{
    BooleanArray,
    DownCastFailure,
    Float32Array,
    IndexListArray,
    StringDictionary,
    UInt16Array,
    UInt32Array,
    UInt64Array,
    UInt8Array,
    UuidArray,
};

pub struct ReadBatchColumns {
    read_id: Result<UuidArray, DownCastFailure>,
    signal: Result<IndexListArray, DownCastFailure>,
    channel: Result<UInt16Array, DownCastFailure>,
    well: Result<UInt8Array, DownCastFailure>,
    pore_type: Result<StringDictionary, DownCastFailure>,
    calibration_offset: Result<Float32Array, DownCastFailure>,
    calibration_scale: Result<Float32Array, DownCastFailure>,
    read_number: Result<UInt32Array, DownCastFailure>,
    start: Result<UInt64Array, DownCastFailure>,
    median_before: Result<Float32Array, DownCastFailure>,
    tracked_scaling_scale: Result<Float32Array, DownCastFailure>,
    tracked_scaling_shift: Result<Float32Array, DownCastFailure>,
    predicted_scaling_scale: Result<Float32Array, DownCastFailure>,
    predicted_scaling_shift: Result<Float32Array, DownCastFailure>,
    num_reads_since_mux_change: Result<UInt32Array, DownCastFailure>,
    time_since_mux_change: Result<Float32Array, DownCastFailure>,
    num_minknow_events: Result<UInt64Array, DownCastFailure>,
    end_reason: Result<StringDictionary, DownCastFailure>,
    end_reason_forced: Result<BooleanArray, DownCastFailure>,
    run_info: Result<StringDictionary, DownCastFailure>,
    num_samples: Result<UInt64Array, DownCastFailure>,
    open_pore_level: Result<Float32Array, DownCastFailure>,
}

impl ReadBatchColumns {
    pub fn new(columns: &[ArrayRef]) -> Option<Self> {
        if columns.len() != 22 {
            return None;
        }

        let read_id = UuidArray::try_from_array_ref(&columns[0]);
        let signal = IndexListArray::try_from_array_ref(&columns[1]);
        let channel = UInt16Array::try_from_array_ref(&columns[2]);
        let well = UInt8Array::try_from_array_ref(&columns[3]);
        let pore_type = StringDictionary::try_from_array_ref(&columns[4]);
        let calibration_offset = Float32Array::try_from_array_ref(&columns[5]);
        let calibration_scale = Float32Array::try_from_array_ref(&columns[6]);
        let read_number = UInt32Array::try_from_array_ref(&columns[7]);
        let start = UInt64Array::try_from_array_ref(&columns[8]);
        let median_before = Float32Array::try_from_array_ref(&columns[9]);
        let tracked_scaling_scale = Float32Array::try_from_array_ref(&columns[10]);
        let tracked_scaling_shift = Float32Array::try_from_array_ref(&columns[11]);
        let predicted_scaling_scale = Float32Array::try_from_array_ref(&columns[12]);
        let predicted_scaling_shift = Float32Array::try_from_array_ref(&columns[13]);
        let num_reads_since_mux_change = UInt32Array::try_from_array_ref(&columns[14]);
        let time_since_mux_change = Float32Array::try_from_array_ref(&columns[15]);
        let num_minknow_events = UInt64Array::try_from_array_ref(&columns[16]);
        let end_reason = StringDictionary::try_from_array_ref(&columns[17]);
        let end_reason_forced = BooleanArray::try_from_array_ref(&columns[18]);
        let run_info = StringDictionary::try_from_array_ref(&columns[19]);
        let num_samples = UInt64Array::try_from_array_ref(&columns[20]);
        let open_pore_level = Float32Array::try_from_array_ref(&columns[21]);

        return Some(Self{
            read_id,
            signal,
            channel,
            well,
            pore_type,
            calibration_offset,
            calibration_scale,
            read_number,
            start,
            median_before,
            tracked_scaling_scale,
            tracked_scaling_shift,
            predicted_scaling_scale,
            predicted_scaling_shift,
            num_reads_since_mux_change,
            time_since_mux_change,
            num_minknow_events,
            end_reason,
            end_reason_forced,
            run_info,
            num_samples,
            open_pore_level,
        })
    }

    /// Returns the stored down-casted `read_id` column.
    pub fn read_id_column(&self) -> Result<&UuidArray, DownCastFailure> {
        match &self.read_id {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column (Recoverable) ---
    /// Returns the stored down-casted signal [`FileRowIndexes`](FileRowIndex) column.
    pub fn signal_column(&self) -> Result<&IndexListArray, DownCastFailure> {
        match &self.signal {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted sequencing channel identifier column.
    pub fn channel_column(&self) -> Result<&UInt16Array, DownCastFailure> {
        match &self.channel {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted pore well identifier column.
    pub fn well_column(&self) -> Result<&UInt8Array, DownCastFailure> {
        match &self.well {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted pore type classification column.
    pub fn pore_type_column(&self) -> Result<&StringDictionary, DownCastFailure> {
        match &self.pore_type {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted calibration offset (applied to signal values) column.
    pub fn calibration_offset_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.calibration_offset {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted calibration scale (applied to signal values) column.
    pub fn calibration_scale_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.calibration_scale {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted sequential read number column.
    pub fn read_number_column(&self) -> Result<&UInt32Array, DownCastFailure> {
        match &self.read_number {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted start position (in samples) column.
    pub fn start_column(&self) -> Result<&UInt64Array, DownCastFailure> {
        match &self.start {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted median signal value (before each read begins) column.
    pub fn median_before_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.median_before {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted tracked scaling factor (applied during basecalling) column.
    pub fn tracked_scaling_scale_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.tracked_scaling_scale {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted tracked scaling shift (applied during basecalling) column.
    pub fn tracked_scaling_shift_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.tracked_scaling_shift {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted predicted scaling factor column.
    pub fn predicted_scaling_scale_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.predicted_scaling_scale {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted predicted scaling shift column.
    pub fn predicted_scaling_shift_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.predicted_scaling_shift {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted column of the number of reads since the last mux change.
    pub fn num_reads_since_mux_change_column(&self) -> Result<&UInt32Array, DownCastFailure> {
        match &self.num_reads_since_mux_change {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted column of the time elapsed since the last mux change.
    pub fn time_since_mux_change_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.time_since_mux_change {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted column of the number of MinKNOW events associated with each read.
    pub fn num_minknow_events_column(&self) -> Result<&UInt64Array, DownCastFailure> {
        match &self.num_minknow_events {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted column of the end reason classification for each read.
    pub fn end_reason_column(&self) -> Result<&StringDictionary, DownCastFailure> {
        match &self.end_reason {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted column of whether the end reason was forced.
    pub fn end_reason_forced_column(&self) -> Result<&BooleanArray, DownCastFailure> {
        match &self.end_reason_forced {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column ---
    /// Returns the stored down-casted run information identifier column.
    pub fn run_info_column(&self) -> Result<&StringDictionary, DownCastFailure> {
        match &self.run_info {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    // --- Core Column (Recoverable) ---
    /// Returns the stored down-casted column of the number of samples in each read.
    pub fn num_samples_column(&self) -> Result<&UInt64Array, DownCastFailure> {
        match &self.num_samples {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }

    /// Returns the stored down-casted open pore level measurement column.
    pub fn open_pore_level_column(&self) -> Result<&Float32Array, DownCastFailure> {
        match &self.open_pore_level {
            Ok(column) => Ok(column),
            Err(error) => Err(error.clone())
        }
    }
}