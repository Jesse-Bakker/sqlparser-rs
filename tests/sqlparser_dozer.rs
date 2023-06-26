// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(clippy::all)]
//! Test SQL syntax specific to Dozer.

#[macro_use]
mod test_utils;
use test_utils::*;

use sqlparser::ast::*;
use sqlparser::dialect::DozerDialect;

fn dozer() -> TestedDialects {
    TestedDialects {
        dialects: vec![Box::new(DozerDialect {})],
        options: None,
    }
}

#[test]
fn parse_function_with_return_type() {
    // check that quoted identifiers in any position remain quoted after serialization
    let select = dozer().verified_only_select(
        r#"SELECT myfun<float>(0, a) FROM b"#,
    );
    assert_eq!(
        &Expr::Function(Function {
            name: ObjectName(vec![Ident::new("myfun")]),
            args: vec![
                FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(
                    number("0")
                ))),
                FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Identifier(Ident::new(
                    "a"
                )))),
            ],
            over: None,
            distinct: false,
            special: false,
            order_by: vec![],
            return_type: Some(Ident::new("float")),
        }),
        expr_from_projection(&select.projection[0]),
    );
}

#[test]
fn parse_function_with_return_type_in_map_access() {
    let sql = r#"SELECT some_map[some_fun<string>(some_arg, 'another_arg')] FROM some_table"#;
    let select = dozer().verified_only_select(sql);
    assert_eq!(
        &Expr::MapAccess {
            column: Box::new(Expr::Identifier(Ident {
                value: "some_map".to_string(),
                quote_style: None,
            })),
            keys: vec![Expr::Function(Function {
                name: ObjectName(vec!["some_fun".into()]),
                args: vec![
                    FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Identifier(Ident::new(
                        "some_arg"
                    )))),
                    FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(
                        Value::SingleQuotedString("another_arg".to_string())
                    ))),
                ],
                over: None,
                distinct: false,
                special: false,
                order_by: vec![],
                return_type: Some(Ident::new("string")),
            })],
        },
        expr_from_projection(&select.projection[0]),
    );
}
