use async_graphql::InputObject;
use uuid::Uuid;

#[derive(InputObject)]
pub struct UuidCursor {
    // #[graphql(desc = "해당 커서보다 이후의 것을 가져옵니다.")]
    after: Option<Uuid>,
    // #[graphql(desc = "해당 커서보다 이전의 것을 가져옵니다.")]
    before: Option<Uuid>,
    // #[graphql(desc = "after 이후 n개를 가져옵니다.")]
    first: Option<i32>,
    // #[graphql(desc = "before 이전 n 개를 가져옵니다.")]
    last: Option<i32>,
}