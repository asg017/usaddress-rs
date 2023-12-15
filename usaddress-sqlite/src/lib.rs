mod usaddr_parts;

use sqlite_loadable::{api, define_scalar_function_with_aux, Error, Result};
use sqlite_loadable::{define_table_function, prelude::*};
use usaddress::Parser;

pub fn usaddress_parse_json(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    parser: &Arc<Mutex<Parser>>,
) -> Result<()> {
    let input = api::value_text(values.get(0).unwrap())?;
    let result = parser.lock().unwrap().parse(input);
    match result {
        Ok(parsed) => api::result_json(context, serde_json::to_value(parsed).unwrap())?,
        Err(_err) => return Err(Error::new_message("TODO")),
    }
    Ok(())
}
use std::sync::{Arc, Mutex};

#[sqlite_entrypoint]
pub fn sqlite3_usaddress_init(db: *mut sqlite3) -> Result<()> {
    let parser = Parser::default();
    let handle: Arc<Mutex<Parser>> = std::sync::Arc::new(std::sync::Mutex::new(parser));

    define_scalar_function_with_aux(
        db,
        "usaddress_parse_json",
        1,
        usaddress_parse_json,
        FunctionFlags::DETERMINISTIC | FunctionFlags::UTF8,
        handle.clone(),
    )?;
    define_table_function::<crate::usaddr_parts::UsaddrPartsTable>(
        db,
        "usaddr_parts",
        Some(handle.clone()),
    )?;
    Ok(())
}
