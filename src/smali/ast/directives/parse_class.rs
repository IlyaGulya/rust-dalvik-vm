use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::{TidAble, TidExt};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::tree::{ParseTree, Tree};

use crate::gen::smali::smaliparser::{ArrayTypeContext, ClassDirectiveContext, ClassModifierContext, ClassNameContext, MethodArgumentsContext, MethodBodyContext, MethodDeclarationContext, MethodDirectiveContext, MethodIdentifierContext, MethodModifierContext, MethodParameterTypeContext, MethodSignatureContext, ReferenceTypeContext, RULE_arrayType, RULE_nonArrayType, RULE_nonVoidType, RULE_primitiveType, RULE_referenceType, SmaliParserContext, SuperDirectiveContext, SuperNameContext};
use crate::smali::ast::directives::directives::{ClassDirective, MethodDirective, SuperDirective};
use crate::smali::ast::directives::parser::StatementParseError;
use crate::smali::ast::literals::StringLiteral;
use crate::smali::ast::method_body::{MethodParameterType, MethodSignature};
use crate::smali::ast::modifiers::{ClassModifier, MethodModifier};
use crate::smali::ast::types::any::AnyType;
use crate::smali::ast::types::array::ArrayType;
use crate::smali::ast::types::non_array_type::NonArrayType;
use crate::smali::ast::types::non_void::NonVoidType;
use crate::smali::ast::types::reference::ReferenceType;

pub fn downcast<'a, T>(context: Rc<dyn SmaliParserContext<'a>>) -> Result<Rc<T>, StatementParseError>
    where
        T: 'a + TidAble<'a>, {
    context
        .downcast_rc::<T>()
        .map_err(|_| StatementParseError::InvalidDirective)
}

pub fn parse_class_directive<'a>(context: Rc<dyn SmaliParserContext<'a> + 'a>) -> Result<ClassDirective, StatementParseError> {
    let context = downcast::<ClassDirectiveContext>(context)?;

    let class_name_ctx =
        context
            .child_of_type::<ClassNameContext>(0)
            .expect("ClassDirective should contain a ClassNameContext");

    let class_name_ref =
        class_name_ctx
            .child_of_type::<ReferenceTypeContext>(0)
            .expect("ClassNameContext should contain a class name reference");

    let ref_name = class_name_ref.get_child(0).unwrap();

    let modifiers =
        context
            .children_of_type::<ClassModifierContext>()
            .into_iter()
            .map(|modifier| {
                match parse_class_modifier(modifier) {
                    Ok(modifier) => modifier,
                    Err(ModifierParseError::InvalidNode) => panic!("Invalid node"),
                    Err(ModifierParseError::InvalidModifier(modifier)) => {
                        panic!("Invalid modifier: {}", modifier)
                    }
                }
            })
            .collect::<Vec<_>>();

    Ok(ClassDirective {
        modifiers,
        name: StringLiteral {
            value: ref_name.get_text(),
        },
    })
}

pub fn parse_super_directive<'a>(context: Rc<dyn SmaliParserContext<'a> + 'a>) -> Result<SuperDirective, StatementParseError> {
    let context = downcast::<SuperDirectiveContext>(context)?;

    let super_name_ctx =
        context
            .child_of_type::<SuperNameContext>(0)
            .expect("SuperDirective should contain a SuperNameContext");

    let super_name_ref =
        super_name_ctx
            .child_of_type::<ReferenceTypeContext>(0)
            .expect("SuperNameContext should contain a super name reference");

    let ref_name = super_name_ref.get_child(0).unwrap();

    Ok(SuperDirective {
        name: ReferenceType {
            type_name: ref_name.get_text(),
        },
    })
}

pub fn parse_method_directive<'a>(context: Rc<dyn SmaliParserContext<'a>>) -> Result<MethodDirective, StatementParseError> {
    let context = downcast::<'a, MethodDirectiveContext>(context)?;

    let declaration =
        context
            .child_of_type::<MethodDeclarationContext>(0)
            .expect("MethodDirective should contain a MethodDeclarationContext");

    let modifiers =
        declaration
            .children_of_type::<MethodModifierContext>()
            .into_iter()
            .map(|modifier| {
                match parse_method_modifier(modifier) {
                    Ok(modifier) => modifier,
                    Err(ModifierParseError::InvalidNode) => panic!("Invalid node"),
                    Err(ModifierParseError::InvalidModifier(modifier)) => {
                        panic!("Invalid modifier: {}", modifier)
                    }
                }
            })
            .collect::<Vec<_>>();

    let signature_ctx =
        declaration
            .child_of_type::<MethodSignatureContext>(0)
            .expect("MethodDeclaration should contain a MethodSignatureContext");

    let identifier =
        signature_ctx
            .child_of_type::<MethodIdentifierContext>(0)
            .expect("MethodDeclaration should contain a MethodSignatureContext")
            .get_text();

    let arguments_ctx =
        signature_ctx
            .child_of_type::<MethodArgumentsContext>(0)
            .expect("MethodDeclaration should contain a MethodArgumentsContext");

    let mut arguments = vec![];
    arguments_ctx
        .children_of_type::<MethodParameterTypeContext>()
        .into_iter()
        .for_each(|child| {
            match parse_method_argument(child) {
                Ok(methodParameterType) => {
                    arguments.push(methodParameterType)
                }
                Err(err) => {
                    panic!("Unable to parse method parameter type: {:?}", err)
                }
            }
        });


    let signature = MethodSignature {
        identifier: StringLiteral { value: identifier },
        arguments,
        return_type: AnyType::VoidType,
    };

    let body =
        context
            .child_of_type::<MethodBodyContext>(0)
            .expect("MethodDirective should contain a MethodBodyContext");

    // let

    // MethodDirective {
    //     declaration: MethodDeclaration {
    //         modifiers: vec![],
    //         signature: MethodSignature {
    //             identifier: StringLiteral { value: "".to_string() },
    //             arguments: vec![],
    //             return_type: AnyType::VoidType,
    //         }, // todo
    //     }, // TODO,
    //     body: None, // TODO,
    // };
    Err(StatementParseError::NotImplemented)
    // let super_name_ctx =
    //     super_directive_ctx
    //         .child_of_type::<SuperNameContext>(0)
    //         .expect("SuperDirective should contain a SuperNameContext");
    //
    //
    // Ok(SuperDirective {
    //     name: ReferenceType {
    //         type_name: super_name_ctx.get_text(),
    //     },
    // })
}

#[derive(Debug)]
enum MethodParameterParseError {}

fn parse_method_argument<'a>(context: Rc<MethodParameterTypeContext>) -> Result<MethodParameterType, MethodParameterParseError> {
    match context.get_rule_index() {
        RULE_nonVoidType => {
            let rc = context.get_child(0).unwrap();
            Ok(MethodParameterType::NonVoid(parse_non_void_type(rc)))
        }
        _ => panic!("Invalid method parameter type. Rule index: ${}", context.get_rule_index())
    }
}

fn parse_non_void_type<'a>(context: Rc<dyn SmaliParserContext<'a>>) -> NonVoidType {
    match context.get_rule_index() {
        RULE_arrayType => {
            NonVoidType::ArrayType(
                parse_array_type(context)
            )
        }
        RULE_nonArrayType => {
            todo!("parse non-array type")
        }
        _ => panic!("Invalid non-void type. Rule index: ${}", context.get_rule_index())
    }
}

fn parse_array_type(context: Rc<dyn SmaliParserContext>) -> ArrayType {
    match context.get_rule_index() {
        RULE_arrayType => {
            let array_type = parse_array_type(context.get_child(0).unwrap());
            ArrayType::Array(Box::new(array_type))
        }
        RULE_nonArrayType => {
            ArrayType::NonArray(
                Box::new(
                    parse_non_array_type(context.get_child(0).unwrap())
                )
            )
        }
        _ => panic!("Invalid array type. Rule index: ${}", context.get_rule_index()),
    }
}

fn parse_non_array_type(context: Rc<dyn SmaliParserContext>) -> NonArrayType {
    match context.get_rule_index() {
        RULE_primitiveType => NonArrayType::Primitive(todo!("parse primitive type")),
        RULE_referenceType => NonArrayType::Reference(
            ReferenceType {
                type_name: context.get_text(),
            }
        ),
        _ => panic!("Invalid non-array type. Rule index: ${}", context.get_rule_index()),
    }
}


fn parse_method_modifier(context: Rc<MethodModifierContext>) -> Result<MethodModifier, ModifierParseError> {
    let text = context.get_text();
    let modifier_name = text.deref();

    let modifier =
        match modifier_name {
            "public" => MethodModifier::Public,
            "private" => MethodModifier::Private,
            "protected" => MethodModifier::Protected,
            "final" => MethodModifier::Final,
            "synthetic" => MethodModifier::Synthetic,
            "static" => MethodModifier::Static,
            "abstract" => MethodModifier::Abstract,
            "constructor" => MethodModifier::Constructor,
            "bridge" => MethodModifier::Bridge,
            "declared-synchronized" => MethodModifier::DeclaredSynchronized,
            "strictfp" => MethodModifier::Strictfp,
            "varargs" => MethodModifier::Varargs,
            "native" => MethodModifier::Native,
            _ => return Err(ModifierParseError::InvalidModifier(modifier_name.to_owned())),
        };

    Ok(modifier)
}

fn parse_class_modifier<'a>(context: Rc<ClassModifierContext>) -> Result<ClassModifier, ModifierParseError> {
    let text = context.get_text();
    let modifier_name = text.deref();

    let modifier = match modifier_name {
        "public" => ClassModifier::Public,
        "private" => ClassModifier::Private,
        "protected" => ClassModifier::Protected,
        "final" => ClassModifier::Final,
        "annotation" => ClassModifier::Annotation,
        "synthetic" => ClassModifier::Synthetic,
        "static" => ClassModifier::Static,
        "abstract" => ClassModifier::Abstract,
        "enum" => ClassModifier::Enum,
        "interface" => ClassModifier::Interface,
        _ => return Err(ModifierParseError::InvalidModifier(modifier_name.to_owned())),
    };

    Ok(modifier)
}

#[derive(Debug)]
pub enum ModifierParseError {
    InvalidNode,
    InvalidModifier(String),
}