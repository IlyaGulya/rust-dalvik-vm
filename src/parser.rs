use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::{InputStream, TidExt};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::tree::{ParseTreeListener, Tree};

use gen::smali::smalilexer::SmaliLexer;

use crate::gen::smali::smalilexer::LocalTokenFactory;
use crate::gen::smali::smaliparser::{FieldDirectiveContextExt, SmaliParser, SmaliParserContext, SourceDirectiveContextExt, StatementContextExt};
use crate::gen::smali::smaliparserlistener::SmaliParserListener;
use crate::smali::ast::directives::directives::{FieldDirective, SourceDirective};
use crate::smali::ast::directives::parse_class::{parse_class_directive, parse_method_directive, parse_super_directive};
use crate::smali::ast::instructions::i::Identifier;
use crate::smali::ast::literals::StringLiteral;
use crate::smali::ast::statement::Statement;
use crate::smali::ast::try_downcast::TryDowncast;
use crate::smali::ast::types::any::AnyType;
use crate::smali::ast::types::field_name_and_type::FieldNameAndType;
use crate::smali::ast::util::Ctx;

pub mod gen;
pub mod smali;

fn dump<'input>(spaces: u32, context: Rc<dyn SmaliParserContext<'input> + 'input>) {
    let prefix = String::from_iter((0..spaces).map(|_| ' '));
    println!("{:?}{:?}", prefix, context);

    context.get_children().for_each(|child| {
        dump(spaces + 2, child)
    })
}

fn parse_top<'input>(context: Rc<dyn SmaliParserContext<'input> + 'input>) -> Vec<Statement> {
    let mut result: Vec<Statement> = vec![];

    // Todo: use .map() instead of .for_each()?
    // dunno how to handle lifetimes properly
    context.get_children().for_each(|child| {
        let child = child.as_ref();
        let statement: &Ctx<StatementContextExt> = child.downcast_ref().expect("Top-level statement should be StatementContext");
        let directive = statement.get_child(0).expect("Top-level statement should contain exactly one directive");

        match directive.get_rule_index() {
            gen::smali::smaliparser::RULE_classDirective => {
                result.push(
                    Statement::Class(
                        parse_class_directive(directive)
                            .expect("Unable to parse class directive")
                    )
                );
            }

            gen::smali::smaliparser::RULE_superDirective => {
                result.push(
                    Statement::Super(
                        parse_super_directive(directive)
                            .expect("Unable to parse super directive")
                    )
                );
            }

            gen::smali::smaliparser::RULE_sourceDirective => {
                let source_directive: &Ctx<SourceDirectiveContextExt> =
                    directive
                        .downcast_ref()
                        .expect("RULE_source_directive should correspond to ClassDirectiveContextExt");
                println!("Source directive: {:?}", source_directive);

                result.push(
                    Statement::Source(SourceDirective {
                        name: StringLiteral {
                            value: String::from("------TODO------") // TODO,
                        }, // TODO,
                    })
                )
            }

            gen::smali::smaliparser::RULE_fieldDirective => {
                let field_directive: &Ctx<FieldDirectiveContextExt> =
                    directive
                        .downcast_ref()
                        .expect("RULE_field_directive should correspond to ClassDirectiveContextExt");
                println!("Field directive: {:?}", field_directive);

                result.push(
                    Statement::Field(FieldDirective {
                        modifiers: vec![], // TODO,
                        name_and_type: FieldNameAndType {
                            name: Identifier::Simple(String::from("--------todo-------")), // TODO,
                            field_type: AnyType::VoidType, // TODO,
                        }, // TODO,
                        value: None, // TODO,
                    })
                )
            }

            gen::smali::smaliparser::RULE_methodDirective => {
                dump(0, directive.clone());

                result.push(
                    Statement::Method(
                        parse_method_directive(directive)
                            .expect("Unable to parse method directive")
                    )
                )
            }

            _ => {
                panic!("Unknown child: {:?}", child);
            }
        }
    });

    result
}

fn main() {
    let mut input_str = String::new();

    let _ = File::open("tests/interpreter/hello.smali")
        .expect("File is requred!")
        .read_to_string(&mut input_str);

    let input = InputStream::new(&input_str[..]);

    let tf = LocalTokenFactory::default();
    let lexer = SmaliLexer::new_with_token_factory(input, &tf);

    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SmaliParser::new(token_source);

    let context = parser.parse().expect("No parse tree generated!");

    let statements = parse_top(context);

    println!("Statements: {:?}", statements);
    // dump(0, context)
}
