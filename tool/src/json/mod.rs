use diplomat_core::{ast, Env};
use std::collections::HashMap;
use std::fmt::{self, Write};

use crate::util;

pub fn gen_bindings(env: &Env, outs: &mut HashMap<String, String>) -> fmt::Result {
    let all_types = util::get_all_custom_types(env);

    let result_out = outs
        .entry("result.json".to_string())
        .or_insert_with(String::new);

    let result_json = serde_json::to_string_pretty(&all_types.order).expect("serde_json error");

    write!(result_out, "{result_json}")?;

    Ok(())
}
