use async_graphql::{Context, Object};
use tracing::instrument;

use general_table::utils::TableInserter;

use crate::entity::public::OtypeCreating;

pub struct Mutation;

#[Object]
impl Mutation {
    #[instrument(level = "info", name = "create_otype", skip_all)]
    async fn create_otype<'a>(
        &self,
        _ctx: &Context<'a>,
        args: OtypeCreating,
    ) -> TableInserter<OtypeCreating> {
        TableInserter::new(args)
    }
}
