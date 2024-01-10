use async_graphql::TypeDirective;


#[TypeDirective(
    location = "FieldDefinition",
    location = "Object",
)]
pub fn rebac(rel: String, otype: String, oid: String, result: bool) {}