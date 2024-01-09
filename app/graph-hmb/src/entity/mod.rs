mod story;

use sea_query::IdenStatic;
pub use story::Story;

#[derive(Debug, Clone, Copy, IdenStatic)]
#[iden = "hmb"]
pub struct Hmb;