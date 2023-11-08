use async_graphql::Object;

pub struct Story;
// #[Object]
impl Story {
    // async fn value(&self) -> String {
    //     self.value.to_string()
    // }
    // #[graphql]
    // async fn value_from_db(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(desc = "Id of object")] id: i64
    // ) -> Result<String> {
    //
    //     let conn = ctx.data::<DbPool>()?.take();
    //     Ok(conn.query_something(id)?.name)
    // }
}