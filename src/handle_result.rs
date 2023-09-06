use datafusion::arrow::{array::*, datatypes::DataType, record_batch::RecordBatch};
use serde_json::Value;

trait AsJsonValue {
    fn as_json_value(&self, index: usize) -> Value;
}

impl AsJsonValue for StringArray {
    fn as_json_value(&self, index: usize) -> Value {
        Value::String(self.value(index).to_string())
    }
}

impl AsJsonValue for BooleanArray {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Bool(self.value(index).into())
    }
}

impl AsJsonValue for UInt8Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for UInt16Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for UInt32Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}
impl AsJsonValue for UInt64Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for Int8Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for Int16Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for Int32Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for Int64Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::Number(self.value(index).into())
    }
}

impl AsJsonValue for Float32Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::String(self.value(index).to_string())
    }
}

impl AsJsonValue for Float64Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::String(self.value(index).to_string())
    }
}

impl AsJsonValue for Date32Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::String(self.value(index).to_string())
    }
}

impl AsJsonValue for Date64Array {
    fn as_json_value(&self, index: usize) -> Value {
        Value::String(self.value(index).to_string())
    }
}

// ... Repeat for other array types ...

#[inline]
fn handle_column(column: &ArrayRef, j: usize) -> Value {
    if column.is_null(j) {
        return Value::String("".to_string());
    }

    match column.data_type() {
        DataType::Utf8 => {
            let array = column.as_any().downcast_ref::<StringArray>().unwrap();
            array.as_json_value(j)
        }
        DataType::Boolean => {
            let array = column.as_any().downcast_ref::<BooleanArray>().unwrap();
            array.as_json_value(j)
        }
        DataType::UInt8 => {
            let array = column.as_any().downcast_ref::<UInt8Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::UInt16 => {
            let array = column.as_any().downcast_ref::<UInt16Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::UInt32 => {
            let array = column.as_any().downcast_ref::<UInt32Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::UInt64 => {
            let array = column.as_any().downcast_ref::<UInt64Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Int8 => {
            let array = column.as_any().downcast_ref::<Int8Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Int16 => {
            let array = column.as_any().downcast_ref::<Int16Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Int32 => {
            let array = column.as_any().downcast_ref::<Int32Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Int64 => {
            let array = column.as_any().downcast_ref::<Int64Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Date32 => {
            let array = column.as_any().downcast_ref::<Date32Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Date64 => {
            let array = column.as_any().downcast_ref::<Date64Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Float32 => {
            let array = column.as_any().downcast_ref::<Float32Array>().unwrap();
            array.as_json_value(j)
        }
        DataType::Float64 => {
            let array = column.as_any().downcast_ref::<Float64Array>().unwrap();
            array.as_json_value(j)
        }
        _ => Value::String("".to_string()),
    }
}

pub fn record_batch_to_vec(batch: RecordBatch) -> Vec<Vec<Value>> {
    let mut result: Vec<Vec<Value>> = Vec::new();

    for i in 0..batch.num_columns() {
        let column = batch.column(i);
        let mut values: Vec<Value> = Vec::new();

        for j in 0..column.len() {
            if column.is_null(j) {
                values.push(Value::String("".to_string()));
            } else {
                values.push(handle_column(column, j));
            }
        }

        result.push(values);
    }

    result
}
