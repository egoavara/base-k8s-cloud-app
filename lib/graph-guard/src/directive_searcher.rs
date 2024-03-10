use std::collections::HashMap;

use async_graphql::parser::types::{
    ExecutableDocument, Field, FragmentDefinition, InlineFragment, OperationDefinition,
    OperationType, Selection,
};
use async_graphql::registry::{MetaDirective, MetaField, MetaType, Registry};
use async_graphql::{Pos, Positioned, ServerError};
use tracing::info;

use crate::RebacTypeDirective;

pub(crate) struct DirectiveSearcher<'a> {
    directives: Vec<FoundRebacTypeDirective>,
    #[allow(dead_code)]
    fragment: HashMap<String, Vec<MetaDirective>>,
    errors: Vec<crate::Error>,

    registry: &'a Registry,
}

pub(crate) struct FoundRebacTypeDirective {
    pub(crate) type_directive: RebacTypeDirective,
    pub(crate) pos: Pos,
}

impl<'a> DirectiveSearcher<'a> {
    pub(crate) fn new(reg: &'a Registry) -> Self {
        Self {
            directives: Vec::new(),
            fragment: HashMap::new(),
            errors: Vec::new(),
            registry: reg,
        }
    }
    pub(crate) fn search(mut self, doc: &'a ExecutableDocument) -> Self {
        doc.fragments.iter().for_each(|(_, def)| {
            self.search_fragment_definition(&def.node);
        });
        doc.operations.iter().for_each(|(_, def)| {
            self.search_operation_definition(&def.node);
        });

        self
    }
    fn search_operation_definition(&mut self, def: &'a OperationDefinition) {
        let r_root_type = match def.ty {
            OperationType::Query => Some(self.registry.query_type.to_string()),
            OperationType::Mutation => self.registry.mutation_type.clone(),
            OperationType::Subscription => self.registry.subscription_type.clone(),
        }
        .ok_or_else(|| crate::Error::RuntimeUnavailableOperationType(def.ty.to_string()));
        match r_root_type {
            Ok(root_type) => {
                def.selection_set
                    .node
                    .items
                    .iter()
                    .for_each(|s| self.search_selection(&s.node, root_type.clone()));
            }
            Err(err) => {
                self.errors.push(err);
                return;
            }
        }
    }
    fn search_fragment_definition(&mut self, def: &'a FragmentDefinition) {
        let otype = def.type_condition.node.on.node.to_string();
        def.selection_set
            .node
            .items
            .iter()
            .for_each(|s| self.search_selection(&s.node, otype.clone()))
    }
    fn search_selection(&mut self, def: &'a Selection, parent_otype: String) {
        match def {
            Selection::Field(field) => self.search_field(&parent_otype, field),
            Selection::FragmentSpread(_sfrag) => {
                // do nothing, predefined fragment already checked in rsearch_rabac_fragdef
            }
            Selection::InlineFragment(ifrag) => self.search_inline_fragment(parent_otype, ifrag),
        }
    }

    fn search_inline_fragment(
        &mut self,
        parent_otype: String,
        ifrag: &'a Positioned<InlineFragment>,
    ) {
        match &ifrag.node.type_condition {
            None => ifrag
                .node
                .selection_set
                .node
                .items
                .iter()
                .for_each(|s| self.search_selection(&s.node, parent_otype.clone())),
            Some(otype) => {
                let otype = otype.node.on.node.to_string();
                ifrag
                    .node
                    .selection_set
                    .node
                    .items
                    .iter()
                    .for_each(|s| self.search_selection(&s.node, otype.clone()))
            }
        }
    }

    fn search_field(&mut self, parent_otype: &String, field: &'a Positioned<Field>) {
        let field_name = field.node.name.node.to_string();
        let (child_field, child_type) =
            match self.get_field_type_by_name(&parent_otype, &field_name) {
                Ok(ok) => ok,
                Err(err) => {
                    self.errors.push(err);
                    return;
                }
            };
        // ex. Query ... { find: [Entity!]! (Entity) ... }
        info!(
            "searching at {} ... {{ {}: {} ({}) ... }}",
            &parent_otype,
            &field_name,
            child_field.ty,
            child_type.name()
        );
        let result = child_field
            .directive_invocations
            .iter()
            .filter(|d| d.name == "rebac")
            .map(|d| {
                info!(
                    "found at {} ... {{ {}: {} ({}) ... }}",
                    &parent_otype,
                    &field_name,
                    child_field.ty,
                    child_type.name()
                );
                RebacTypeDirective::try_from(d).map(|x| FoundRebacTypeDirective {
                    type_directive: x,
                    pos: field.pos,
                })
            })
            .collect::<Result<Vec<_>, _>>();
        match result {
            Ok(directives) => {
                self.directives.extend(directives);
            }
            Err(err) => {
                self.errors.push(err);
            }
        }
        field
            .node
            .selection_set
            .node
            .items
            .iter()
            .for_each(|s| self.search_selection(&s.node, child_type.name().to_string()))
    }

    fn get_field_type_by_name(
        &self,
        otype: &String,
        field_name: &str,
    ) -> Result<(MetaField, MetaType), crate::Error> {
        let meta_type = self.registry.types.get(otype);
        match meta_type {
            Some(meta_type) => match meta_type.fields() {
                Some(meta_field) => meta_field
                    .get(field_name)
                    .ok_or_else(|| crate::Error::RuntimeUnknownTypeField {
                        otype: otype.clone(),
                        field: field_name.to_string(),
                    })
                    .and_then(|f| {
                        self.registry
                            .concrete_type_by_name(&f.ty)
                            .map(|x| (f.clone(), x.clone()))
                            .ok_or_else(|| crate::Error::RuntimeUnknownType {
                                otype: otype.clone(),
                            })
                    }),
                None => Err(crate::Error::RuntimeUnknownTypeField {
                    otype: otype.clone(),
                    field: field_name.to_string(),
                }),
            },
            None => Err(crate::Error::RuntimeUnknownType {
                otype: otype.clone(),
            }),
        }
    }
    pub(crate) fn end(self) -> Result<Vec<FoundRebacTypeDirective>, ServerError> {
        if self.errors.is_empty() {
            Ok(self.directives)
        } else {
            let message = self
                .errors
                .into_iter()
                .map(|e| e.to_string())
                .reduce(|a, b| a + "\n" + &b)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(ServerError::new(
                format!("graph_guard::rebac, \n{}", message),
                None,
            ))
        }
    }
}
