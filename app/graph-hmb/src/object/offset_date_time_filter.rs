use async_graphql::InputObject;

use crate::object::offset_date_time_scalar::OffsetDateTimeScalar;

#[derive(Debug, Clone, Default, InputObject)]
pub struct OffsetDateTimeFilter {
    pub eq: Option<OffsetDateTimeScalar>,
    pub ne: Option<OffsetDateTimeScalar>,
    pub gt: Option<OffsetDateTimeScalar>,
    pub gte: Option<OffsetDateTimeScalar>,
    pub lt: Option<OffsetDateTimeScalar>,
    pub lte: Option<OffsetDateTimeScalar>,
    // pub timezone: Option<String>,
}