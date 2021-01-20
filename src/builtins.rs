#![allow(unused_variables)]
use crate::interpreter::Context;
use crate::value::{Function, Value};
use anyhow::Result;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref BUILTINS: Context<'static> = {
        let mut builtins = Context::new();
        builtins.insert("abs", Value::Function(Function(abs_builtin)));
        builtins.insert("str", Value::Function(Function(str_builtin)));
        builtins.insert("len", Value::Function(Function(len_builtin)));
        builtins.insert("min", Value::Function(Function(min_builtin)));
        builtins.insert("max", Value::Function(Function(max_builtin)));
        builtins.insert("sqrt", Value::Function(Function(sqrt_builtin)));
        builtins.insert("ceil", Value::Function(Function(ceil_builtin)));
        builtins.insert("floor", Value::Function(Function(floor_builtin)));
        builtins.insert("lowercase", Value::Function(Function(lowercase_builtin)));
        builtins.insert("uppercase", Value::Function(Function(uppercase_builtin)));
        builtins.insert("number", Value::Function(Function(number_builtin)));
        builtins.insert("strip", Value::Function(Function(strip_builtin)));
        builtins.insert("rstrip", Value::Function(Function(rstrip_builtin)));
        builtins.insert("lstrip", Value::Function(Function(lstrip_builtin)));
        builtins.insert("join", Value::Function(Function(join_builtin)));
        builtins.insert("split", Value::Function(Function(split_builtin)));
        builtins.insert("fromNow", Value::Function(Function(from_now_builtin)));
        builtins.insert("typeof", Value::Function(Function(typeof_builtin)));
        builtins.insert("defined", Value::Function(Function(defined_builtin)));
        builtins
    };
}

fn abs_builtin(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(interpreter_error!("abs expects one argument"));
    }

    if let Some(arg) = args[0].as_f64() {
        return Ok(Value::Number(arg.abs()));
    } else {
        return Err(interpreter_error!("abs expects a numeric argument"));
    }
}

fn str_builtin(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(interpreter_error!("str expects one argument"));
    }
    let v = &args[0];

    match v {
        Value::Null | Value::String(_) | Value::Number(_) | Value::Bool(_) => {
            v.stringify().map(|s| Value::String(s))
        }
        _ => Err(interpreter_error!("invalid arguments to builtin: str")),
    }
}

fn len_builtin(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(interpreter_error!("len expects one argument"));
    }
    let v = &args[0];

    match v {
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        Value::Array(a) => Ok(Value::Number(a.len() as f64)),
        _ => Err(interpreter_error!("invalid arguments to builtin: len")),
    }
}

fn min_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn max_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn sqrt_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn ceil_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn floor_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn lowercase_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn uppercase_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn number_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn strip_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn rstrip_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn lstrip_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn join_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn split_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn from_now_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn typeof_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
fn defined_builtin(args: &[Value]) -> Result<Value> {
    todo!()
}
