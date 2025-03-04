use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::model::{FloatType, IntType, List, RuntimeError, Value};

// 🦀 Poor thing, you went unused too?
// pub struct ArgumentError {
//   msg: String,
//   index: usize,
// }

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`
/// and err if there isn't one.
pub fn require_parameter<'a>(
    func_name: &str,
    args: &'a [Value],
    index: usize,
) -> Result<&'a Value, RuntimeError> {
    match args.get(index) {
        Some(val) => Ok(val),
        None => Err(RuntimeError {
            msg: format!(
                "Function \"{}\" requires an argument {}",
                func_name,
                index + 1
            ),
        }),
    }
}

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`,
/// assumed to be a `Value::Int()`, and return its inner i32. Err if any part
/// of this fails.
pub fn require_int_parameter(
    func_name: &str,
    args: &[Value],
    index: usize,
) -> Result<IntType, RuntimeError> {
    match require_parameter(func_name, args, index) {
        Ok(val) => match val.as_int() {
            Some(x) => Ok(x),
            None => Err(RuntimeError {
                msg: format!(
                    "Function \"{}\" requires argument {} to be an integer; got {}",
                    func_name,
                    index + 1,
                    val.type_name()
                ),
            }),
        },
        Err(err) => Err(err),
    }
}

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`,
/// assumed to be a `Value::Float()`, and return its inner f32. Err if any part
/// of this fails.
pub fn require_float_parameter(
    func_name: &str,
    args: &[Value],
    index: usize,
) -> Result<FloatType, RuntimeError> {
    match require_parameter(func_name, args, index) {
        Ok(val) => match val.as_float() {
            Some(x) => Ok(x),
            None => Err(RuntimeError {
                msg: format!(
                    "Function \"{}\" requires argument {} to be a float; got {}",
                    func_name,
                    index + 1,
                    val.type_name()
                ),
            }),
        },
        Err(err) => Err(err),
    }
}

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`,
/// assumed to be a `Value::String()`, and return a reference to its inner
/// String. Err if any part of this fails.
pub fn require_string_parameter<'a>(
    func_name: &str,
    args: &'a [Value],
    index: usize,
) -> Result<&'a str, RuntimeError> {
    match require_parameter(func_name, args, index) {
        Ok(val) => match val.as_string() {
            Some(x) => Ok(x),
            None => Err(RuntimeError {
                msg: format!(
                    "Function \"{}\" requires argument {} to be a string; got {}",
                    func_name,
                    index + 1,
                    val.type_name()
                ),
            }),
        },
        Err(err) => Err(err),
    }
}

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`,
/// assumed to be a `Value::List()` or a `Value::NIL`, erring if that isn't
/// the case.
pub fn require_list_parameter<'a>(
    func_name: &str,
    args: &'a [Value],
    index: usize,
) -> Result<&'a List, RuntimeError> {
    match require_parameter(func_name, args, index) {
        Ok(val) => match val {
            Value::List(list) => Ok(list),
            _ => Err(RuntimeError {
                msg: format!(
                    "Function \"{}\" requires argument {} to be a list; got {}",
                    func_name,
                    index + 1,
                    val.type_name()
                ),
            }),
        },
        Err(err) => Err(err),
    }
}

/// Given a `Value` assumed to be a `Value::List()`, grab the item at `index`,
/// assumed to be a `Value::HashMap()`, and return a reference to its inner
/// HashMap. Err if any part of this fails.
pub fn require_hash_parameter<'a>(
    func_name: &str,
    args: &'a [Value],
    index: usize,
) -> Result<&'a Rc<RefCell<HashMap<Value, Value>>>, RuntimeError> {
    match require_parameter(func_name, args, index) {
        Ok(val) => match val {
            Value::HashMap(hash) => Ok(hash),
            _ => Err(RuntimeError {
                msg: format!(
                    "Function \"{}\" requires argument {} to be a hash map; got {}",
                    func_name,
                    index + 1,
                    val.type_name()
                ),
            }),
        },
        Err(err) => Err(err),
    }
}
