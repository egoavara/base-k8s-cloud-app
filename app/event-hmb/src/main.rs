use async_graphql::InputObject;

use table_traits::{filter};

filter! {
    #[derive(Debug, Default, InputObject)]
    struct TestFilter for String impl eq + ne + in;
}
//
// #[derive(Table)]
// pub struct Test {
//     #[table(id)]
//     pub id: Uuid,
// }

#[tokio::main]
async fn main() {
    let _a = TestFilter::default();
    println!("Hello, world!");
}
