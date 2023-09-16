use crate::response::{QueryJobResult, Schema};
use datafusion::arrow::{array::*, datatypes::DataType, record_batch::RecordBatch};
use serde_json::Value;

trait AsJsonValue {
    fn as_json_value(&self, index: usize) -> Value;
}

macro_rules! impl_as_json_value_for_number {
    ($array_type:ty) => {
        impl AsJsonValue for $array_type {
            fn as_json_value(&self, index: usize) -> Value {
                Value::Number(self.value(index).into())
            }
        }
    };
}

macro_rules! impl_as_json_value_for_string {
    ($array_type:ty) => {
        impl AsJsonValue for $array_type {
            fn as_json_value(&self, index: usize) -> Value {
                Value::String(self.value(index).to_string())
            }
        }
    };
}

impl_as_json_value_for_string!(StringArray);
impl_as_json_value_for_string!(Float32Array);
impl_as_json_value_for_string!(Float64Array);
impl_as_json_value_for_string!(Date32Array);
impl_as_json_value_for_string!(Date64Array);

impl AsJsonValue for BooleanArray {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Bool(self.value(index))
    }
}

impl_as_json_value_for_number!(UInt8Array);
impl_as_json_value_for_number!(UInt16Array);
impl_as_json_value_for_number!(UInt32Array);
impl_as_json_value_for_number!(UInt64Array);
impl_as_json_value_for_number!(Int8Array);
impl_as_json_value_for_number!(Int16Array);
impl_as_json_value_for_number!(Int32Array);
impl_as_json_value_for_number!(Int64Array);

#[inline]
fn handle_column(column: &ArrayRef, j: usize) -> Value {
    if column.is_null(j) {
        return Value::String("".to_string());
    }

    macro_rules! downcast_and_invoke {
        ($array_type:ty) => {
            column
                .as_any()
                .downcast_ref::<$array_type>()
                .unwrap()
                .as_json_value(j)
        };
    }

    match column.data_type() {
        DataType::Utf8 => downcast_and_invoke!(StringArray),
        DataType::Boolean => downcast_and_invoke!(BooleanArray),
        DataType::UInt8 => downcast_and_invoke!(UInt8Array),
        DataType::UInt16 => downcast_and_invoke!(UInt16Array),
        DataType::UInt32 => downcast_and_invoke!(UInt32Array),
        DataType::UInt64 => downcast_and_invoke!(UInt64Array),
        DataType::Int8 => downcast_and_invoke!(Int8Array),
        DataType::Int16 => downcast_and_invoke!(Int16Array),
        DataType::Int32 => downcast_and_invoke!(Int32Array),
        DataType::Int64 => downcast_and_invoke!(Int64Array),
        DataType::Float32 => downcast_and_invoke!(Float32Array),
        DataType::Float64 => downcast_and_invoke!(Float64Array),
        DataType::Date32 => downcast_and_invoke!(Date32Array),
        DataType::Date64 => downcast_and_invoke!(Date64Array),
        _ => Value::String("".to_string()),
    }
}

pub fn record_batch_to_vec(batch: RecordBatch) -> QueryJobResult {
    let num_rows = batch.num_rows() as u32;

    let schema_fields: Vec<String> = batch
        .schema()
        .fields()
        .iter()
        .map(|field| field.name().clone())
        .collect();

    let schema_types: Vec<String> = batch
        .schema()
        .fields()
        .iter()
        .map(|field| format!("{:?}", field.data_type()))
        .collect();

    let columns = (0..batch.num_columns())
        .map(|i| {
            (0..batch.column(i).len())
                .map(|j| {
                    if batch.column(i).is_null(j) {
                        Value::String("".to_string())
                    } else {
                        handle_column(batch.column(i), j)
                    }
                })
                .collect()
        })
        .collect();

    QueryJobResult {
        total_rows: num_rows,
        schema: Schema {
            fields: schema_fields,
            types: schema_types,
        },
        columns,
    }
}
