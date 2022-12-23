use jsonrpc_http_server::{jsonrpc_core::Error, jsonrpc_core::Params, jsonrpc_core::Value};

pub const METHOD: crate::methods::Method = crate::methods::Method {
    prefix: "net",
    name: "listening",
};

pub async fn execute(_params: Params) -> Result<Value, Error> {
    return Ok(Value::Bool(false));
}