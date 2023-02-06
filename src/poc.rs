use async_trait::async_trait;
use convergence::engine::{Engine, Portal};
use convergence::protocol::{ErrorResponse, FieldDescription};
use convergence::protocol_ext::DataRowBatch;
use sqlparser::ast::{Statement};

struct POCPortal {
    // df: Arc<DataFrame>,
}

#[async_trait]
impl Portal for POCPortal {
    async fn fetch(&mut self, batch: &mut DataRowBatch) -> Result<(), ErrorResponse> {
        Ok(())
    }
}

struct POCEngine;

#[async_trait]
impl Engine for POCEngine {
    type PortalType = POCPortal;

    async fn prepare(&mut self, stmt: &Statement) -> Result<Vec<FieldDescription>, ErrorResponse> {
        todo!()
    }

    async fn create_portal(&mut self, _: &sqlparser::ast::Statement) -> Result<Self::PortalType, ErrorResponse> {
        Ok(POCPortal{})
    }
}