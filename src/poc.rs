use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use duckdb::{Connection, DuckdbConnectionManager};
use convergence::engine::{Engine, Portal};
use convergence::protocol::{DataTypeOid, ErrorResponse, FieldDescription, SqlState};
use convergence::protocol_ext::DataRowBatch;
use r2d2::Pool;
use sqlparser::ast::{Expr, SelectItem, SetExpr, Statement};

pub struct POCPortal {
    // df: Arc<DataFrame>,
}

#[async_trait]
impl Portal for POCPortal {
    async fn fetch(&mut self, batch: &mut DataRowBatch) -> Result<(), ErrorResponse> {
        let mut row = batch.create_row();
        row.write_int4(1);
        Ok(())    }
}

pub struct POCEngine {
    pub pool: Pool<DuckdbConnectionManager>
    // pub duckdb: DuckdbConnectionManager
    // pub duck_conn: Arc<Connection>
}

#[async_trait]
impl Engine for POCEngine {
    type PortalType = POCPortal;

    async fn prepare(&mut self, statement: &Statement) -> Result<Vec<FieldDescription>, ErrorResponse> {
        if let Statement::Query(query) = &statement {
            if let SetExpr::Select(select) = &*query.body {
                if select.projection.len() == 1 {
                    if let SelectItem::UnnamedExpr(Expr::Identifier(column_name)) = &select.projection[0] {
                        match column_name.value.as_str() {
                            "test_error" => return Err(ErrorResponse::error(SqlState::DATA_EXCEPTION, "test error")),
                            "test_fatal" => return Err(ErrorResponse::fatal(SqlState::DATA_EXCEPTION, "fatal error")),
                            _ => (),
                        }
                    }
                }
            }
        }

        Ok(vec![FieldDescription {
            name: "test".to_owned(),
            data_type: DataTypeOid::Int4,
        }])    }

    async fn create_portal(&mut self, _: &sqlparser::ast::Statement) -> Result<Self::PortalType, ErrorResponse> {
        Ok(POCPortal{})
    }
}