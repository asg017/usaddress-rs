use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};
use sqlite_loadable::{prelude::*, Error};
use usaddress::Parser;

use std::{
    mem,
    os::raw::c_int,
    sync::{Arc, Mutex},
};

static CREATE_SQL: &str = "CREATE TABLE x(a, input hidde)";
enum Columns {
    A,
    Input,
}
fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::A),
        1 => Some(Columns::Input),
        _ => None,
    }
}

#[repr(C)]
pub struct UsaddrPartsTable<'vtab> {
    /// must be first
    base: sqlite3_vtab,
    handle: Arc<Mutex<Parser<'vtab>>>,
}

impl<'vtab> VTab<'vtab> for UsaddrPartsTable<'vtab> {
    type Aux = Arc<Mutex<Parser<'vtab>>>;
    type Cursor = UsaddrPartsCursor;

    fn connect(
        _db: *mut sqlite3,
        aux: Option<&Self::Aux>,
        _args: VTabArguments,
    ) -> Result<(String, UsaddrPartsTable<'vtab>)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = UsaddrPartsTable {
            base,
            handle: aux.unwrap().to_owned(),
        };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: IndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_input = false;
        for mut constraint in info.constraints() {
            match column(constraint.column_idx()) {
                Some(Columns::Input) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(1);
                        has_input = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                Some(_) | None => (),
            }
        }
        if !has_input {
            return Err(BestIndexError::Error);
        }
        info.set_estimated_cost(100000.0);
        info.set_estimated_rows(100000);
        info.set_idxnum(2);

        Ok(())
    }

    fn open(&mut self) -> Result<UsaddrPartsCursor> {
        Ok(UsaddrPartsCursor::new(self.handle.clone()))
    }

    fn create(
        db: *mut sqlite3,
        aux: Option<&Self::Aux>,
        args: VTabArguments,
    ) -> Result<(String, Self)> {
        Self::connect(db, aux, args)
    }
}

#[repr(C)]
pub struct UsaddrPartsCursor {
    base: sqlite3_vtab_cursor,
    rowid: i64,
}
impl<'cursor> UsaddrPartsCursor {
    fn new(_handle: Arc<Mutex<Parser<'cursor>>>) -> UsaddrPartsCursor {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        UsaddrPartsCursor { base, rowid: 0 }
    }
}

impl VTabCursor for UsaddrPartsCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let input = api::value_text(
            values
                .get(0)
                .ok_or_else(|| Error::new_message("expected 1st argument as input address"))?,
        )?;

        self.rowid = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.rowid += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.rowid > 0
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
        match column(i) {
            Some(Columns::A) => {
                api::result_null(context);
            }
            None | Some(Columns::Input) => api::result_null(context),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid)
    }
}
