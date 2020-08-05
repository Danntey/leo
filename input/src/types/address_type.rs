use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_address))]
pub struct AddressType {}