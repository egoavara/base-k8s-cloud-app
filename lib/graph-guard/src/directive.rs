use std::borrow::Cow;

use crate::User;
use async_graphql::indexmap::IndexMap;
use async_graphql::registry::{MetaDirective, MetaDirectiveInvocation, MetaInputValue};
use async_graphql::{InputType, TypeDirective};
use openfga_client::Tuple;

pub struct RebacTypeDirective {
    pub rel: String,
    pub otype: String,
    pub oid: String,
    pub result: bool,
}

#[allow(non_camel_case_types)]
pub struct rebac;

impl RebacTypeDirective {
    pub fn fga_notation(&self) -> String {
        format!("{}:{}", self.otype, self.oid)
    }
    pub fn tuple(&self, user: &User) -> Tuple {
        Tuple::new(user.fga_notation(), self.rel.clone(), self.fga_notation())
    }
}

impl<'a> TryFrom<&'a MetaDirectiveInvocation> for RebacTypeDirective {
    type Error = crate::Error;

    fn try_from(value: &'a MetaDirectiveInvocation) -> Result<Self, Self::Error> {
        let mut rel: Option<String> = None;
        let mut otype: Option<String> = None;
        let mut oid: Option<String> = None;
        let mut result: Option<bool> = None;

        for (name, value) in &value.args {
            match (name.as_str(), value) {
                ("rel", async_graphql_value::ConstValue::String(str)) => {
                    if rel.is_some() {
                        return Err(crate::Error::DirectiveDuplicateArgument("rel".to_string()));
                    }
                    rel = Some(str.to_string());
                }
                ("rel", _) => {
                    return Err(crate::Error::DirectiveArgumentMustBeAString(
                        "rel".to_string(),
                    ));
                }
                ("otype", async_graphql_value::ConstValue::String(str)) => {
                    if otype.is_some() {
                        return Err(crate::Error::DirectiveDuplicateArgument(
                            "otype".to_string(),
                        ));
                    }
                    otype = Some(str.to_string());
                }
                ("otype", _) => {
                    return Err(crate::Error::DirectiveArgumentMustBeAString(
                        "otype".to_string(),
                    ));
                }
                ("oid", async_graphql_value::ConstValue::String(str)) => {
                    if oid.is_some() {
                        return Err(crate::Error::DirectiveDuplicateArgument("oid".to_string()));
                    }
                    oid = Some(str.to_string());
                }
                ("oid", _) => {
                    return Err(crate::Error::DirectiveArgumentMustBeAString(
                        "oid".to_string(),
                    ));
                }
                ("result", async_graphql_value::ConstValue::Boolean(b)) => {
                    if result.is_some() {
                        return Err(crate::Error::DirectiveDuplicateArgument(
                            "result".to_string(),
                        ));
                    }
                    result = Some(*b);
                }
                ("result", _) => {
                    return Err(crate::Error::DirectiveArgumentMustBeABoolean(
                        "result".to_string(),
                    ));
                }
                _ => {
                    return Err(crate::Error::DirectiveUnknownArgument(
                        name.to_string(),
                        value.clone(),
                    ));
                }
            }
        }
        match (rel, otype, oid, result) {
            (Some(rel), Some(otype), Some(oid), oresult) => Ok(RebacTypeDirective {
                rel,
                otype,
                oid,
                result: oresult.unwrap_or(true),
            }),
            (rel, otype, oid, _) => {
                let not_exist_fields = vec![rel, otype, oid]
                    .into_iter()
                    .filter_map(|x| x)
                    .collect::<Vec<_>>();
                Err(crate::Error::DirectiveNoRequiredField(not_exist_fields))
            }
        }
    }
}

impl TypeDirective for rebac {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("rebac")
    }
    fn register(&self, registry: &mut async_graphql::registry::Registry) {
        let meta = MetaDirective {
            name: Cow::into_owned(Cow::Borrowed("rebac")),
            description: None,
            locations: <[_]>::into_vec(Box::new([
                async_graphql::registry::__DirectiveLocation::FIELD_DEFINITION,
                async_graphql::registry::__DirectiveLocation::OBJECT,
            ])),
            args: {
                let mut args = IndexMap::new();
                args.insert(
                    ToOwned::to_owned("rel"),
                    MetaInputValue {
                        name: ToString::to_string("rel"),
                        description: None,
                        ty: <String as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Default::default(),
                        is_secret: false,
                    },
                );
                args.insert(
                    ToOwned::to_owned("otype"),
                    MetaInputValue {
                        name: ToString::to_string("otype"),
                        description: None,
                        ty: <String as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Default::default(),
                        is_secret: false,
                    },
                );
                args.insert(
                    ToOwned::to_owned("oid"),
                    MetaInputValue {
                        name: ToString::to_string("oid"),
                        description: None,
                        ty: <String as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Default::default(),
                        is_secret: false,
                    },
                );
                args.insert(
                    ToOwned::to_owned("result"),
                    MetaInputValue {
                        name: ToString::to_string("result"),
                        description: None,
                        ty: <bool as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Default::default(),
                        is_secret: false,
                    },
                );
                args
            },
            is_repeatable: false,
            visible: None,
            composable: None,
        };
        registry.add_directive(meta);
    }
}

impl async_graphql::registry::location_traits::Directive_At_FIELD_DEFINITION for rebac {}

impl async_graphql::registry::location_traits::Directive_At_OBJECT for rebac {}

impl rebac {
    pub fn apply<REL: Into<String>, OTY: Into<String>, OID: Into<String>>(
        rel: REL,
        otype: OTY,
        oid: OID,
    ) -> MetaDirectiveInvocation {
        let rel = rel.into();
        let otype = otype.into();
        let oid = oid.into();

        if otype.contains(":") {
            panic!(
                "otype cannot contain ':', for '? {} {}:{}'",
                rel, otype, oid
            );
        }

        if oid.contains(":") {
            panic!("oid cannot contain ':', for '? {} {}:{}'", rel, otype, oid);
        }

        let directive = Cow::into_owned(Cow::Borrowed("rebac"));
        let mut args = IndexMap::new();
        if let Some(val) = InputType::as_raw_value(&rel) {
            args.insert(
                ToString::to_string("rel"),
                async_graphql::ScalarType::to_value(val),
            );
        }
        if let Some(val) = InputType::as_raw_value(&otype) {
            args.insert(
                ToString::to_string("otype"),
                async_graphql::ScalarType::to_value(val),
            );
        }
        if let Some(val) = InputType::as_raw_value(&oid) {
            args.insert(
                ToString::to_string("oid"),
                async_graphql::ScalarType::to_value(val),
            );
        }
        args.insert(
            ToString::to_string("result"),
            async_graphql::ScalarType::to_value(&true),
        );
        MetaDirectiveInvocation {
            name: directive,
            args,
        }
    }
}
