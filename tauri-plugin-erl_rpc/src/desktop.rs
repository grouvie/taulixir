use std::io::{Error, ErrorKind};

use erl_dist::term::{Atom, BigInteger, Float, ImproperList, List, Term};
use erl_rpc::RpcClientHandle;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{error, models::*};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<ErlRpc<R>> {
    Ok(ErlRpc(app.clone()))
}

/// Access to the erl-rpc APIs.
pub struct ErlRpc<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ErlRpc<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
    pub async fn invoke_erl_rpc(
        &self,
        module: &str,
        function: &str,
        payload: Option<Value>,
        mut rpc_handle: RpcClientHandle,
    ) -> crate::Result<RpcResponse> {
        let arguments: Vec<erl_dist::term::Term> = payload
            .map(convert_value)
            .transpose()?
            .map(|v| vec![v])
            .unwrap_or_default();

        let list = if arguments.is_empty() {
            erl_dist::term::List::nil()
        } else {
            erl_dist::term::List {
                elements: arguments,
            }
        };

        let term = rpc_handle
            .call(module.into(), function.into(), list)
            .await
            .map_err(|error| {
                error::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, error))
            })?;

        // Convert the returned term to serde_json::Value
        let response_value = convert_term(term)?;

        // TODO: We still have to come up with a way to handle returning None
        Ok(RpcResponse {
            value: Some(response_value),
        })
    }
}

fn convert_term(term: Term) -> crate::Result<Value> {
    match term {
        Term::Atom(atom) => Ok(Value::from(atom.name)),
        Term::FixInteger(fix_integer) => Ok(Value::from(fix_integer.value)),
        Term::Float(float) => Ok(Value::from(float.value)),
        Term::Binary(binary) => Ok(Value::from(binary.bytes)),
        Term::BitBinary(bit_binary) => Ok(Value::from(bit_binary.bytes)),
        Term::List(list) => {
            let mut vec_of_values = Vec::with_capacity(list.elements.len());
            for element in list.elements {
                let value = convert_term(element)?;
                vec_of_values.push(value);
            }
            Ok(Value::from(vec_of_values))
        }
        Term::ImproperList(improper_list) => {
            let mut head_vals = Vec::with_capacity(improper_list.elements.len());
            for element in improper_list.elements {
                let value = convert_term(element)?;
                head_vals.push(value);
            }
            let tail_value = convert_term(*improper_list.last)?;
            Ok(json!({
                "elements": head_vals,
                "last": tail_value,
            }))
        }
        Term::Tuple(tuple) => {
            let mut tuple_vals = Vec::with_capacity(tuple.elements.len());
            for element in tuple.elements {
                let value = convert_term(element)?;
                tuple_vals.push(value);
            }
            Ok(Value::from(tuple_vals))
        }
        Term::Map(map) => {
            let mut hm = serde_json::Map::new();
            for (k, v) in map.entries {
                let key = convert_term(k)?;
                let value = convert_term(v)?;
                hm.insert(key.to_string(), value);
            }
            Ok(Value::from(hm))
        }
        _ => Err(error::Error::Io(Error::new(
            ErrorKind::Other,
            "Not supported",
        ))),
    }
}

pub fn convert_value(value: Value) -> crate::Result<Term> {
    match value {
        Value::String(s) => {
            // Assume JSON strings map to Atoms.
            Ok(Term::Atom(Atom { name: s }))
        }
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(Term::BigInteger(BigInteger { value: i.into() }))
            } else if let Some(f) = num.as_f64() {
                Ok(Term::Float(Float { value: f }))
            } else {
                Err(error::Error::Io(Error::new(
                    ErrorKind::Other,
                    "Unsupported number type",
                )))
            }
        }
        Value::Bool(b) => {
            // You might choose how to represent booleans. For example, as atoms:
            Ok(Term::Atom(Atom {
                name: b.to_string(),
            }))
        }
        Value::Array(arr) => {
            let mut terms = Vec::with_capacity(arr.len());
            for elem in arr {
                let term = convert_value(elem)?;
                terms.push(term);
            }
            Ok(Term::List(List { elements: terms }))
        }
        Value::Object(mut map) => {
            // Check if this object represents an ImproperList.
            if map.contains_key("elements") && map.contains_key("last") {
                // Remove keys so we don't confuse with a regular map.
                let elems = map.remove("elements").unwrap();
                let last = map.remove("last").unwrap();
                // Convert "elements" to Vec<Term> (expecting an array).
                let elements = match elems {
                    Value::Array(v) => {
                        let mut terms = Vec::with_capacity(v.len());
                        for elem in v {
                            terms.push(convert_value(elem)?);
                        }
                        terms
                    }
                    _ => {
                        return Err(error::Error::Io(Error::new(
                            ErrorKind::Other,
                            "Expected an array for 'elements' in ImproperList",
                        )));
                    }
                };
                // Convert the tail.
                let last_term = Box::new(convert_value(last)?);
                Ok(Term::ImproperList(ImproperList {
                    elements,
                    last: last_term,
                }))
            } else {
                // Otherwise, treat it as a Map.
                let mut entries = Vec::with_capacity(map.len());
                for (k, v) in map {
                    let key_term = Term::Atom(Atom { name: k });
                    let value_term = convert_value(v)?;
                    entries.push((key_term, value_term));
                }
                Ok(Term::Map(erl_dist::term::Map { entries }))
            }
        }
        Value::Null => {
            // Decide what null maps to; one option is to use an Atom "null".
            Ok(Term::Atom(Atom {
                name: "null".to_string(),
            }))
        }
    }
}
