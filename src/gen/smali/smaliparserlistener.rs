#![allow(nonstandard_style)]
// Generated from SmaliParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::smaliparser::*;

pub trait SmaliParserListener<'input> : ParseTreeListener<'input,SmaliParserContextType>{
/**
 * Enter a parse tree produced by {@link SmaliParser#registerIdentifier}.
 * @param ctx the parse tree
 */
fn enter_registerIdentifier(&mut self, _ctx: &RegisterIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#registerIdentifier}.
 * @param ctx the parse tree
 */
fn exit_registerIdentifier(&mut self, _ctx: &RegisterIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#negativeNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_negativeNumericLiteral(&mut self, _ctx: &NegativeNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#negativeNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_negativeNumericLiteral(&mut self, _ctx: &NegativeNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#decimalNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_decimalNumericLiteral(&mut self, _ctx: &DecimalNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#decimalNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_decimalNumericLiteral(&mut self, _ctx: &DecimalNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#hexNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_hexNumericLiteral(&mut self, _ctx: &HexNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#hexNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_hexNumericLiteral(&mut self, _ctx: &HexNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#octNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_octNumericLiteral(&mut self, _ctx: &OctNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#octNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_octNumericLiteral(&mut self, _ctx: &OctNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#binaryNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_binaryNumericLiteral(&mut self, _ctx: &BinaryNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#binaryNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_binaryNumericLiteral(&mut self, _ctx: &BinaryNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#floatNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_floatNumericLiteral(&mut self, _ctx: &FloatNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#floatNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_floatNumericLiteral(&mut self, _ctx: &FloatNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#hexFloatLiteral}.
 * @param ctx the parse tree
 */
fn enter_hexFloatLiteral(&mut self, _ctx: &HexFloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#hexFloatLiteral}.
 * @param ctx the parse tree
 */
fn exit_hexFloatLiteral(&mut self, _ctx: &HexFloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#positiveNumericLiteral}.
 * @param ctx the parse tree
 */
fn enter_positiveNumericLiteral(&mut self, _ctx: &PositiveNumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#positiveNumericLiteral}.
 * @param ctx the parse tree
 */
fn exit_positiveNumericLiteral(&mut self, _ctx: &PositiveNumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#referenceType}.
 * @param ctx the parse tree
 */
fn enter_referenceType(&mut self, _ctx: &ReferenceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#referenceType}.
 * @param ctx the parse tree
 */
fn exit_referenceType(&mut self, _ctx: &ReferenceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#voidType}.
 * @param ctx the parse tree
 */
fn enter_voidType(&mut self, _ctx: &VoidTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#voidType}.
 * @param ctx the parse tree
 */
fn exit_voidType(&mut self, _ctx: &VoidTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#booleanType}.
 * @param ctx the parse tree
 */
fn enter_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#booleanType}.
 * @param ctx the parse tree
 */
fn exit_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#byteType}.
 * @param ctx the parse tree
 */
fn enter_byteType(&mut self, _ctx: &ByteTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#byteType}.
 * @param ctx the parse tree
 */
fn exit_byteType(&mut self, _ctx: &ByteTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shortType}.
 * @param ctx the parse tree
 */
fn enter_shortType(&mut self, _ctx: &ShortTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shortType}.
 * @param ctx the parse tree
 */
fn exit_shortType(&mut self, _ctx: &ShortTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#charType}.
 * @param ctx the parse tree
 */
fn enter_charType(&mut self, _ctx: &CharTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#charType}.
 * @param ctx the parse tree
 */
fn exit_charType(&mut self, _ctx: &CharTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intType}.
 * @param ctx the parse tree
 */
fn enter_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intType}.
 * @param ctx the parse tree
 */
fn exit_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#longType}.
 * @param ctx the parse tree
 */
fn enter_longType(&mut self, _ctx: &LongTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#longType}.
 * @param ctx the parse tree
 */
fn exit_longType(&mut self, _ctx: &LongTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#floatType}.
 * @param ctx the parse tree
 */
fn enter_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#floatType}.
 * @param ctx the parse tree
 */
fn exit_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#doubleType}.
 * @param ctx the parse tree
 */
fn enter_doubleType(&mut self, _ctx: &DoubleTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#doubleType}.
 * @param ctx the parse tree
 */
fn exit_doubleType(&mut self, _ctx: &DoubleTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#nonArrayType}.
 * @param ctx the parse tree
 */
fn enter_nonArrayType(&mut self, _ctx: &NonArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#nonArrayType}.
 * @param ctx the parse tree
 */
fn exit_nonArrayType(&mut self, _ctx: &NonArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodParameterLiteral}.
 * @param ctx the parse tree
 */
fn enter_methodParameterLiteral(&mut self, _ctx: &MethodParameterLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodParameterLiteral}.
 * @param ctx the parse tree
 */
fn exit_methodParameterLiteral(&mut self, _ctx: &MethodParameterLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayType}.
 * @param ctx the parse tree
 */
fn enter_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayType}.
 * @param ctx the parse tree
 */
fn exit_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#referenceOrArrayType}.
 * @param ctx the parse tree
 */
fn enter_referenceOrArrayType(&mut self, _ctx: &ReferenceOrArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#referenceOrArrayType}.
 * @param ctx the parse tree
 */
fn exit_referenceOrArrayType(&mut self, _ctx: &ReferenceOrArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#nonVoidType}.
 * @param ctx the parse tree
 */
fn enter_nonVoidType(&mut self, _ctx: &NonVoidTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#nonVoidType}.
 * @param ctx the parse tree
 */
fn exit_nonVoidType(&mut self, _ctx: &NonVoidTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#anyType}.
 * @param ctx the parse tree
 */
fn enter_anyType(&mut self, _ctx: &AnyTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#anyType}.
 * @param ctx the parse tree
 */
fn exit_anyType(&mut self, _ctx: &AnyTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#assignableValue}.
 * @param ctx the parse tree
 */
fn enter_assignableValue(&mut self, _ctx: &AssignableValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#assignableValue}.
 * @param ctx the parse tree
 */
fn exit_assignableValue(&mut self, _ctx: &AssignableValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#classModifier}.
 * @param ctx the parse tree
 */
fn enter_classModifier(&mut self, _ctx: &ClassModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#classModifier}.
 * @param ctx the parse tree
 */
fn exit_classModifier(&mut self, _ctx: &ClassModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodModifier}.
 * @param ctx the parse tree
 */
fn enter_methodModifier(&mut self, _ctx: &MethodModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodModifier}.
 * @param ctx the parse tree
 */
fn exit_methodModifier(&mut self, _ctx: &MethodModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldModifier}.
 * @param ctx the parse tree
 */
fn enter_fieldModifier(&mut self, _ctx: &FieldModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldModifier}.
 * @param ctx the parse tree
 */
fn exit_fieldModifier(&mut self, _ctx: &FieldModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#labelName}.
 * @param ctx the parse tree
 */
fn enter_labelName(&mut self, _ctx: &LabelNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#labelName}.
 * @param ctx the parse tree
 */
fn exit_labelName(&mut self, _ctx: &LabelNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#label}.
 * @param ctx the parse tree
 */
fn enter_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#label}.
 * @param ctx the parse tree
 */
fn exit_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#leftRegister}.
 * @param ctx the parse tree
 */
fn enter_leftRegister(&mut self, _ctx: &LeftRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#leftRegister}.
 * @param ctx the parse tree
 */
fn exit_leftRegister(&mut self, _ctx: &LeftRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#rightRegister}.
 * @param ctx the parse tree
 */
fn enter_rightRegister(&mut self, _ctx: &RightRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#rightRegister}.
 * @param ctx the parse tree
 */
fn exit_rightRegister(&mut self, _ctx: &RightRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#registerListRegisters}.
 * @param ctx the parse tree
 */
fn enter_registerListRegisters(&mut self, _ctx: &RegisterListRegistersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#registerListRegisters}.
 * @param ctx the parse tree
 */
fn exit_registerListRegisters(&mut self, _ctx: &RegisterListRegistersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#registerRange}.
 * @param ctx the parse tree
 */
fn enter_registerRange(&mut self, _ctx: &RegisterRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#registerRange}.
 * @param ctx the parse tree
 */
fn exit_registerRange(&mut self, _ctx: &RegisterRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#registerList}.
 * @param ctx the parse tree
 */
fn enter_registerList(&mut self, _ctx: &RegisterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#registerList}.
 * @param ctx the parse tree
 */
fn exit_registerList(&mut self, _ctx: &RegisterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#gotoInstruction}.
 * @param ctx the parse tree
 */
fn enter_gotoInstruction(&mut self, _ctx: &GotoInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#gotoInstruction}.
 * @param ctx the parse tree
 */
fn exit_gotoInstruction(&mut self, _ctx: &GotoInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#goto16Instruction}.
 * @param ctx the parse tree
 */
fn enter_goto16Instruction(&mut self, _ctx: &Goto16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#goto16Instruction}.
 * @param ctx the parse tree
 */
fn exit_goto16Instruction(&mut self, _ctx: &Goto16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#goto32Instruction}.
 * @param ctx the parse tree
 */
fn enter_goto32Instruction(&mut self, _ctx: &Goto32InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#goto32Instruction}.
 * @param ctx the parse tree
 */
fn exit_goto32Instruction(&mut self, _ctx: &Goto32InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveResultInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveResultInstruction(&mut self, _ctx: &MoveResultInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveResultInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveResultInstruction(&mut self, _ctx: &MoveResultInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveResultWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveResultWideInstruction(&mut self, _ctx: &MoveResultWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveResultWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveResultWideInstruction(&mut self, _ctx: &MoveResultWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveResultObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveResultObjectInstruction(&mut self, _ctx: &MoveResultObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveResultObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveResultObjectInstruction(&mut self, _ctx: &MoveResultObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveExceptionInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveExceptionInstruction(&mut self, _ctx: &MoveExceptionInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveExceptionInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveExceptionInstruction(&mut self, _ctx: &MoveExceptionInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#returnInstruction}.
 * @param ctx the parse tree
 */
fn enter_returnInstruction(&mut self, _ctx: &ReturnInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#returnInstruction}.
 * @param ctx the parse tree
 */
fn exit_returnInstruction(&mut self, _ctx: &ReturnInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#returnWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_returnWideInstruction(&mut self, _ctx: &ReturnWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#returnWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_returnWideInstruction(&mut self, _ctx: &ReturnWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#returnObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_returnObjectInstruction(&mut self, _ctx: &ReturnObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#returnObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_returnObjectInstruction(&mut self, _ctx: &ReturnObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#monitorEnterInstruction}.
 * @param ctx the parse tree
 */
fn enter_monitorEnterInstruction(&mut self, _ctx: &MonitorEnterInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#monitorEnterInstruction}.
 * @param ctx the parse tree
 */
fn exit_monitorEnterInstruction(&mut self, _ctx: &MonitorEnterInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#monitorExitInstruction}.
 * @param ctx the parse tree
 */
fn enter_monitorExitInstruction(&mut self, _ctx: &MonitorExitInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#monitorExitInstruction}.
 * @param ctx the parse tree
 */
fn exit_monitorExitInstruction(&mut self, _ctx: &MonitorExitInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#throwInstruction}.
 * @param ctx the parse tree
 */
fn enter_throwInstruction(&mut self, _ctx: &ThrowInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#throwInstruction}.
 * @param ctx the parse tree
 */
fn exit_throwInstruction(&mut self, _ctx: &ThrowInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#returnVoidInstruction}.
 * @param ctx the parse tree
 */
fn enter_returnVoidInstruction(&mut self, _ctx: &ReturnVoidInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#returnVoidInstruction}.
 * @param ctx the parse tree
 */
fn exit_returnVoidInstruction(&mut self, _ctx: &ReturnVoidInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#nopInstruction}.
 * @param ctx the parse tree
 */
fn enter_nopInstruction(&mut self, _ctx: &NopInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#nopInstruction}.
 * @param ctx the parse tree
 */
fn exit_nopInstruction(&mut self, _ctx: &NopInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveInstruction(&mut self, _ctx: &MoveInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveInstruction(&mut self, _ctx: &MoveInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveFrom16Instruction}.
 * @param ctx the parse tree
 */
fn enter_moveFrom16Instruction(&mut self, _ctx: &MoveFrom16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveFrom16Instruction}.
 * @param ctx the parse tree
 */
fn exit_moveFrom16Instruction(&mut self, _ctx: &MoveFrom16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#move16Instruction}.
 * @param ctx the parse tree
 */
fn enter_move16Instruction(&mut self, _ctx: &Move16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#move16Instruction}.
 * @param ctx the parse tree
 */
fn exit_move16Instruction(&mut self, _ctx: &Move16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveWideInstruction(&mut self, _ctx: &MoveWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveWideInstruction(&mut self, _ctx: &MoveWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveWideFrom16Instruction}.
 * @param ctx the parse tree
 */
fn enter_moveWideFrom16Instruction(&mut self, _ctx: &MoveWideFrom16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveWideFrom16Instruction}.
 * @param ctx the parse tree
 */
fn exit_moveWideFrom16Instruction(&mut self, _ctx: &MoveWideFrom16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveWide16Instruction}.
 * @param ctx the parse tree
 */
fn enter_moveWide16Instruction(&mut self, _ctx: &MoveWide16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveWide16Instruction}.
 * @param ctx the parse tree
 */
fn exit_moveWide16Instruction(&mut self, _ctx: &MoveWide16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_moveObjectInstruction(&mut self, _ctx: &MoveObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_moveObjectInstruction(&mut self, _ctx: &MoveObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveObjectFrom16Instruction}.
 * @param ctx the parse tree
 */
fn enter_moveObjectFrom16Instruction(&mut self, _ctx: &MoveObjectFrom16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveObjectFrom16Instruction}.
 * @param ctx the parse tree
 */
fn exit_moveObjectFrom16Instruction(&mut self, _ctx: &MoveObjectFrom16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#moveObject16Instruction}.
 * @param ctx the parse tree
 */
fn enter_moveObject16Instruction(&mut self, _ctx: &MoveObject16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#moveObject16Instruction}.
 * @param ctx the parse tree
 */
fn exit_moveObject16Instruction(&mut self, _ctx: &MoveObject16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constInstruction}.
 * @param ctx the parse tree
 */
fn enter_constInstruction(&mut self, _ctx: &ConstInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constInstruction}.
 * @param ctx the parse tree
 */
fn exit_constInstruction(&mut self, _ctx: &ConstInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#const4Instruction}.
 * @param ctx the parse tree
 */
fn enter_const4Instruction(&mut self, _ctx: &Const4InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#const4Instruction}.
 * @param ctx the parse tree
 */
fn exit_const4Instruction(&mut self, _ctx: &Const4InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#const16Instruction}.
 * @param ctx the parse tree
 */
fn enter_const16Instruction(&mut self, _ctx: &Const16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#const16Instruction}.
 * @param ctx the parse tree
 */
fn exit_const16Instruction(&mut self, _ctx: &Const16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constHigh16Instruction}.
 * @param ctx the parse tree
 */
fn enter_constHigh16Instruction(&mut self, _ctx: &ConstHigh16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constHigh16Instruction}.
 * @param ctx the parse tree
 */
fn exit_constHigh16Instruction(&mut self, _ctx: &ConstHigh16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constWide16Instruction}.
 * @param ctx the parse tree
 */
fn enter_constWide16Instruction(&mut self, _ctx: &ConstWide16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constWide16Instruction}.
 * @param ctx the parse tree
 */
fn exit_constWide16Instruction(&mut self, _ctx: &ConstWide16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constWide32Instruction}.
 * @param ctx the parse tree
 */
fn enter_constWide32Instruction(&mut self, _ctx: &ConstWide32InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constWide32Instruction}.
 * @param ctx the parse tree
 */
fn exit_constWide32Instruction(&mut self, _ctx: &ConstWide32InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_constWideInstruction(&mut self, _ctx: &ConstWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_constWideInstruction(&mut self, _ctx: &ConstWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constWideHigh16Instruction}.
 * @param ctx the parse tree
 */
fn enter_constWideHigh16Instruction(&mut self, _ctx: &ConstWideHigh16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constWideHigh16Instruction}.
 * @param ctx the parse tree
 */
fn exit_constWideHigh16Instruction(&mut self, _ctx: &ConstWideHigh16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constString}.
 * @param ctx the parse tree
 */
fn enter_constString(&mut self, _ctx: &ConstStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constString}.
 * @param ctx the parse tree
 */
fn exit_constString(&mut self, _ctx: &ConstStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constStringJumbo}.
 * @param ctx the parse tree
 */
fn enter_constStringJumbo(&mut self, _ctx: &ConstStringJumboContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constStringJumbo}.
 * @param ctx the parse tree
 */
fn exit_constStringJumbo(&mut self, _ctx: &ConstStringJumboContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#constClass}.
 * @param ctx the parse tree
 */
fn enter_constClass(&mut self, _ctx: &ConstClassContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#constClass}.
 * @param ctx the parse tree
 */
fn exit_constClass(&mut self, _ctx: &ConstClassContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetInstruction(&mut self, _ctx: &SGetInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetInstruction(&mut self, _ctx: &SGetInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetWideInstruction(&mut self, _ctx: &SGetWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetWideInstruction(&mut self, _ctx: &SGetWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetObjectInstruction(&mut self, _ctx: &SGetObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetObjectInstruction(&mut self, _ctx: &SGetObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetBooleanInstruction(&mut self, _ctx: &SGetBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetBooleanInstruction(&mut self, _ctx: &SGetBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetByteInstruction(&mut self, _ctx: &SGetByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetByteInstruction(&mut self, _ctx: &SGetByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetCharInstruction(&mut self, _ctx: &SGetCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetCharInstruction(&mut self, _ctx: &SGetCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sGetShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_sGetShortInstruction(&mut self, _ctx: &SGetShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sGetShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_sGetShortInstruction(&mut self, _ctx: &SGetShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutInstruction(&mut self, _ctx: &SPutInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutInstruction(&mut self, _ctx: &SPutInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutWideInstruction(&mut self, _ctx: &SPutWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutWideInstruction(&mut self, _ctx: &SPutWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutObjectInstruction(&mut self, _ctx: &SPutObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutObjectInstruction(&mut self, _ctx: &SPutObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutBooleanInstruction(&mut self, _ctx: &SPutBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutBooleanInstruction(&mut self, _ctx: &SPutBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutByteInstruction(&mut self, _ctx: &SPutByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutByteInstruction(&mut self, _ctx: &SPutByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutCharInstruction(&mut self, _ctx: &SPutCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutCharInstruction(&mut self, _ctx: &SPutCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sPutShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_sPutShortInstruction(&mut self, _ctx: &SPutShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sPutShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_sPutShortInstruction(&mut self, _ctx: &SPutShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeVirtualInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeVirtualInstruction(&mut self, _ctx: &InvokeVirtualInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeVirtualInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeVirtualInstruction(&mut self, _ctx: &InvokeVirtualInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeSuperInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeSuperInstruction(&mut self, _ctx: &InvokeSuperInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeSuperInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeSuperInstruction(&mut self, _ctx: &InvokeSuperInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeDirectInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeDirectInstruction(&mut self, _ctx: &InvokeDirectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeDirectInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeDirectInstruction(&mut self, _ctx: &InvokeDirectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeStaticInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeStaticInstruction(&mut self, _ctx: &InvokeStaticInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeStaticInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeStaticInstruction(&mut self, _ctx: &InvokeStaticInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeInterfaceInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeInterfaceInstruction(&mut self, _ctx: &InvokeInterfaceInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeInterfaceInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeInterfaceInstruction(&mut self, _ctx: &InvokeInterfaceInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeVirtualRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeVirtualRangeInstruction(&mut self, _ctx: &InvokeVirtualRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeVirtualRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeVirtualRangeInstruction(&mut self, _ctx: &InvokeVirtualRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeSuperRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeSuperRangeInstruction(&mut self, _ctx: &InvokeSuperRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeSuperRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeSuperRangeInstruction(&mut self, _ctx: &InvokeSuperRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeDirectRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeDirectRangeInstruction(&mut self, _ctx: &InvokeDirectRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeDirectRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeDirectRangeInstruction(&mut self, _ctx: &InvokeDirectRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeStaticRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeStaticRangeInstruction(&mut self, _ctx: &InvokeStaticRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeStaticRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeStaticRangeInstruction(&mut self, _ctx: &InvokeStaticRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeInterfaceRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeInterfaceRangeInstruction(&mut self, _ctx: &InvokeInterfaceRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeInterfaceRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeInterfaceRangeInstruction(&mut self, _ctx: &InvokeInterfaceRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToLongInstruction(&mut self, _ctx: &IntToLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToLongInstruction(&mut self, _ctx: &IntToLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToFloatInstruction(&mut self, _ctx: &IntToFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToFloatInstruction(&mut self, _ctx: &IntToFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToDoubleInstruction(&mut self, _ctx: &IntToDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToDoubleInstruction(&mut self, _ctx: &IntToDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#longToIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_longToIntInstruction(&mut self, _ctx: &LongToIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#longToIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_longToIntInstruction(&mut self, _ctx: &LongToIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#longToFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_longToFloatInstruction(&mut self, _ctx: &LongToFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#longToFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_longToFloatInstruction(&mut self, _ctx: &LongToFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#longToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_longToDoubleInstruction(&mut self, _ctx: &LongToDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#longToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_longToDoubleInstruction(&mut self, _ctx: &LongToDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#floatToIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_floatToIntInstruction(&mut self, _ctx: &FloatToIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#floatToIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_floatToIntInstruction(&mut self, _ctx: &FloatToIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#floatToLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_floatToLongInstruction(&mut self, _ctx: &FloatToLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#floatToLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_floatToLongInstruction(&mut self, _ctx: &FloatToLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#floatToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_floatToDoubleInstruction(&mut self, _ctx: &FloatToDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#floatToDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_floatToDoubleInstruction(&mut self, _ctx: &FloatToDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#doubleToIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_doubleToIntInstruction(&mut self, _ctx: &DoubleToIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#doubleToIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_doubleToIntInstruction(&mut self, _ctx: &DoubleToIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#doubleToLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_doubleToLongInstruction(&mut self, _ctx: &DoubleToLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#doubleToLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_doubleToLongInstruction(&mut self, _ctx: &DoubleToLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#doubleToFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_doubleToFloatInstruction(&mut self, _ctx: &DoubleToFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#doubleToFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_doubleToFloatInstruction(&mut self, _ctx: &DoubleToFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToByteInstruction(&mut self, _ctx: &IntToByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToByteInstruction(&mut self, _ctx: &IntToByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToCharInstruction(&mut self, _ctx: &IntToCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToCharInstruction(&mut self, _ctx: &IntToCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#intToShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_intToShortInstruction(&mut self, _ctx: &IntToShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#intToShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_intToShortInstruction(&mut self, _ctx: &IntToShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifLabel}.
 * @param ctx the parse tree
 */
fn enter_ifLabel(&mut self, _ctx: &IfLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifLabel}.
 * @param ctx the parse tree
 */
fn exit_ifLabel(&mut self, _ctx: &IfLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifEqzInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifEqzInstruction(&mut self, _ctx: &IfEqzInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifEqzInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifEqzInstruction(&mut self, _ctx: &IfEqzInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifNezInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifNezInstruction(&mut self, _ctx: &IfNezInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifNezInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifNezInstruction(&mut self, _ctx: &IfNezInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifLtzInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifLtzInstruction(&mut self, _ctx: &IfLtzInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifLtzInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifLtzInstruction(&mut self, _ctx: &IfLtzInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifGezInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifGezInstruction(&mut self, _ctx: &IfGezInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifGezInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifGezInstruction(&mut self, _ctx: &IfGezInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifGtzInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifGtzInstruction(&mut self, _ctx: &IfGtzInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifGtzInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifGtzInstruction(&mut self, _ctx: &IfGtzInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifLezInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifLezInstruction(&mut self, _ctx: &IfLezInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifLezInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifLezInstruction(&mut self, _ctx: &IfLezInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#negIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_negIntInstruction(&mut self, _ctx: &NegIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#negIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_negIntInstruction(&mut self, _ctx: &NegIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#notIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_notIntInstruction(&mut self, _ctx: &NotIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#notIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_notIntInstruction(&mut self, _ctx: &NotIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#negLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_negLongInstruction(&mut self, _ctx: &NegLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#negLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_negLongInstruction(&mut self, _ctx: &NegLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#notLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_notLongInstruction(&mut self, _ctx: &NotLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#notLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_notLongInstruction(&mut self, _ctx: &NotLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#negFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_negFloatInstruction(&mut self, _ctx: &NegFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#negFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_negFloatInstruction(&mut self, _ctx: &NegFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#negDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_negDoubleInstruction(&mut self, _ctx: &NegDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#negDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_negDoubleInstruction(&mut self, _ctx: &NegDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifEqInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifEqInstruction(&mut self, _ctx: &IfEqInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifEqInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifEqInstruction(&mut self, _ctx: &IfEqInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifNeInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifNeInstruction(&mut self, _ctx: &IfNeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifNeInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifNeInstruction(&mut self, _ctx: &IfNeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifLtInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifLtInstruction(&mut self, _ctx: &IfLtInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifLtInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifLtInstruction(&mut self, _ctx: &IfLtInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifGeInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifGeInstruction(&mut self, _ctx: &IfGeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifGeInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifGeInstruction(&mut self, _ctx: &IfGeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifGtInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifGtInstruction(&mut self, _ctx: &IfGtInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifGtInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifGtInstruction(&mut self, _ctx: &IfGtInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ifLeInstruction}.
 * @param ctx the parse tree
 */
fn enter_ifLeInstruction(&mut self, _ctx: &IfLeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ifLeInstruction}.
 * @param ctx the parse tree
 */
fn exit_ifLeInstruction(&mut self, _ctx: &IfLeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_addInt2addrInstruction(&mut self, _ctx: &AddInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_addInt2addrInstruction(&mut self, _ctx: &AddInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_subInt2addrInstruction(&mut self, _ctx: &SubInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_subInt2addrInstruction(&mut self, _ctx: &SubInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulInt2addrInstruction(&mut self, _ctx: &MulInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulInt2addrInstruction(&mut self, _ctx: &MulInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_divInt2addrInstruction(&mut self, _ctx: &DivInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_divInt2addrInstruction(&mut self, _ctx: &DivInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_remInt2addrInstruction(&mut self, _ctx: &RemInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_remInt2addrInstruction(&mut self, _ctx: &RemInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_andInt2addrInstruction(&mut self, _ctx: &AndInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_andInt2addrInstruction(&mut self, _ctx: &AndInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_orInt2addrInstruction(&mut self, _ctx: &OrInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_orInt2addrInstruction(&mut self, _ctx: &OrInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_xorInt2addrInstruction(&mut self, _ctx: &XorInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_xorInt2addrInstruction(&mut self, _ctx: &XorInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shlInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_shlInt2addrInstruction(&mut self, _ctx: &ShlInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shlInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_shlInt2addrInstruction(&mut self, _ctx: &ShlInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shrInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_shrInt2addrInstruction(&mut self, _ctx: &ShrInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shrInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_shrInt2addrInstruction(&mut self, _ctx: &ShrInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ushrInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_ushrInt2addrInstruction(&mut self, _ctx: &UshrInt2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ushrInt2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_ushrInt2addrInstruction(&mut self, _ctx: &UshrInt2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_addLong2addrInstruction(&mut self, _ctx: &AddLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_addLong2addrInstruction(&mut self, _ctx: &AddLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_subLong2addrInstruction(&mut self, _ctx: &SubLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_subLong2addrInstruction(&mut self, _ctx: &SubLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulLong2addrInstruction(&mut self, _ctx: &MulLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulLong2addrInstruction(&mut self, _ctx: &MulLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_divLong2addrInstruction(&mut self, _ctx: &DivLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_divLong2addrInstruction(&mut self, _ctx: &DivLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_remLong2addrInstruction(&mut self, _ctx: &RemLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_remLong2addrInstruction(&mut self, _ctx: &RemLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_andLong2addrInstruction(&mut self, _ctx: &AndLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_andLong2addrInstruction(&mut self, _ctx: &AndLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_orLong2addrInstruction(&mut self, _ctx: &OrLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_orLong2addrInstruction(&mut self, _ctx: &OrLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_xorLong2addrInstruction(&mut self, _ctx: &XorLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_xorLong2addrInstruction(&mut self, _ctx: &XorLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shlLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_shlLong2addrInstruction(&mut self, _ctx: &ShlLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shlLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_shlLong2addrInstruction(&mut self, _ctx: &ShlLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shrLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_shrLong2addrInstruction(&mut self, _ctx: &ShrLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shrLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_shrLong2addrInstruction(&mut self, _ctx: &ShrLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ushrLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_ushrLong2addrInstruction(&mut self, _ctx: &UshrLong2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ushrLong2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_ushrLong2addrInstruction(&mut self, _ctx: &UshrLong2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_addFloat2addrInstruction(&mut self, _ctx: &AddFloat2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_addFloat2addrInstruction(&mut self, _ctx: &AddFloat2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_subFloat2addrInstruction(&mut self, _ctx: &SubFloat2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_subFloat2addrInstruction(&mut self, _ctx: &SubFloat2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulFloat2addrInstruction(&mut self, _ctx: &MulFloat2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulFloat2addrInstruction(&mut self, _ctx: &MulFloat2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_divFloat2addrInstruction(&mut self, _ctx: &DivFloat2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_divFloat2addrInstruction(&mut self, _ctx: &DivFloat2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_remFloat2addrInstruction(&mut self, _ctx: &RemFloat2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remFloat2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_remFloat2addrInstruction(&mut self, _ctx: &RemFloat2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_addDouble2addrInstruction(&mut self, _ctx: &AddDouble2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_addDouble2addrInstruction(&mut self, _ctx: &AddDouble2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_subDouble2addrInstruction(&mut self, _ctx: &SubDouble2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_subDouble2addrInstruction(&mut self, _ctx: &SubDouble2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulDouble2addrInstruction(&mut self, _ctx: &MulDouble2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulDouble2addrInstruction(&mut self, _ctx: &MulDouble2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_divDouble2addrInstruction(&mut self, _ctx: &DivDouble2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_divDouble2addrInstruction(&mut self, _ctx: &DivDouble2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn enter_remDouble2addrInstruction(&mut self, _ctx: &RemDouble2addrInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remDouble2addrInstruction}.
 * @param ctx the parse tree
 */
fn exit_remDouble2addrInstruction(&mut self, _ctx: &RemDouble2addrInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#cmplFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_cmplFloatInstruction(&mut self, _ctx: &CmplFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#cmplFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_cmplFloatInstruction(&mut self, _ctx: &CmplFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#cmpgFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_cmpgFloatInstruction(&mut self, _ctx: &CmpgFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#cmpgFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_cmpgFloatInstruction(&mut self, _ctx: &CmpgFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#cmplDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_cmplDoubleInstruction(&mut self, _ctx: &CmplDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#cmplDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_cmplDoubleInstruction(&mut self, _ctx: &CmplDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#cmpgDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_cmpgDoubleInstruction(&mut self, _ctx: &CmpgDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#cmpgDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_cmpgDoubleInstruction(&mut self, _ctx: &CmpgDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#cmpLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_cmpLongInstruction(&mut self, _ctx: &CmpLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#cmpLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_cmpLongInstruction(&mut self, _ctx: &CmpLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#field}.
 * @param ctx the parse tree
 */
fn enter_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#field}.
 * @param ctx the parse tree
 */
fn exit_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayRegister}.
 * @param ctx the parse tree
 */
fn enter_arrayRegister(&mut self, _ctx: &ArrayRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayRegister}.
 * @param ctx the parse tree
 */
fn exit_arrayRegister(&mut self, _ctx: &ArrayRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#indexRegister}.
 * @param ctx the parse tree
 */
fn enter_indexRegister(&mut self, _ctx: &IndexRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#indexRegister}.
 * @param ctx the parse tree
 */
fn exit_indexRegister(&mut self, _ctx: &IndexRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#instanceRegister}.
 * @param ctx the parse tree
 */
fn enter_instanceRegister(&mut self, _ctx: &InstanceRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#instanceRegister}.
 * @param ctx the parse tree
 */
fn exit_instanceRegister(&mut self, _ctx: &InstanceRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sourceRegister}.
 * @param ctx the parse tree
 */
fn enter_sourceRegister(&mut self, _ctx: &SourceRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sourceRegister}.
 * @param ctx the parse tree
 */
fn exit_sourceRegister(&mut self, _ctx: &SourceRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#targetRegister}.
 * @param ctx the parse tree
 */
fn enter_targetRegister(&mut self, _ctx: &TargetRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#targetRegister}.
 * @param ctx the parse tree
 */
fn exit_targetRegister(&mut self, _ctx: &TargetRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#instanceField}.
 * @param ctx the parse tree
 */
fn enter_instanceField(&mut self, _ctx: &InstanceFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#instanceField}.
 * @param ctx the parse tree
 */
fn exit_instanceField(&mut self, _ctx: &InstanceFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetInstruction(&mut self, _ctx: &AgetInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetInstruction(&mut self, _ctx: &AgetInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetWideInstruction(&mut self, _ctx: &AgetWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetWideInstruction(&mut self, _ctx: &AgetWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetObjectInstruction(&mut self, _ctx: &AgetObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetObjectInstruction(&mut self, _ctx: &AgetObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetBooleanInstruction(&mut self, _ctx: &AgetBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetBooleanInstruction(&mut self, _ctx: &AgetBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetByteInstruction(&mut self, _ctx: &AgetByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetByteInstruction(&mut self, _ctx: &AgetByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetCharInstruction(&mut self, _ctx: &AgetCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetCharInstruction(&mut self, _ctx: &AgetCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#agetShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_agetShortInstruction(&mut self, _ctx: &AgetShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#agetShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_agetShortInstruction(&mut self, _ctx: &AgetShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputInstruction(&mut self, _ctx: &AputInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputInstruction(&mut self, _ctx: &AputInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputWideInstruction(&mut self, _ctx: &AputWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputWideInstruction(&mut self, _ctx: &AputWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputObjectInstruction(&mut self, _ctx: &AputObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputObjectInstruction(&mut self, _ctx: &AputObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputBooleanInstruction(&mut self, _ctx: &AputBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputBooleanInstruction(&mut self, _ctx: &AputBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputByteInstruction(&mut self, _ctx: &AputByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputByteInstruction(&mut self, _ctx: &AputByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputCharInstruction(&mut self, _ctx: &AputCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputCharInstruction(&mut self, _ctx: &AputCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#aputShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_aputShortInstruction(&mut self, _ctx: &AputShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#aputShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_aputShortInstruction(&mut self, _ctx: &AputShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetInstruction(&mut self, _ctx: &IgetInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetInstruction(&mut self, _ctx: &IgetInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetWideInstruction(&mut self, _ctx: &IgetWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetWideInstruction(&mut self, _ctx: &IgetWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetObjectInstruction(&mut self, _ctx: &IgetObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetObjectInstruction(&mut self, _ctx: &IgetObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetBooleanInstruction(&mut self, _ctx: &IgetBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetBooleanInstruction(&mut self, _ctx: &IgetBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetByteInstruction(&mut self, _ctx: &IgetByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetByteInstruction(&mut self, _ctx: &IgetByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetCharInstruction(&mut self, _ctx: &IgetCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetCharInstruction(&mut self, _ctx: &IgetCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#igetShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_igetShortInstruction(&mut self, _ctx: &IgetShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#igetShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_igetShortInstruction(&mut self, _ctx: &IgetShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputInstruction(&mut self, _ctx: &IputInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputInstruction(&mut self, _ctx: &IputInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputWideInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputWideInstruction(&mut self, _ctx: &IputWideInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputWideInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputWideInstruction(&mut self, _ctx: &IputWideInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputObjectInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputObjectInstruction(&mut self, _ctx: &IputObjectInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputObjectInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputObjectInstruction(&mut self, _ctx: &IputObjectInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputBooleanInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputBooleanInstruction(&mut self, _ctx: &IputBooleanInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputBooleanInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputBooleanInstruction(&mut self, _ctx: &IputBooleanInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputByteInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputByteInstruction(&mut self, _ctx: &IputByteInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputByteInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputByteInstruction(&mut self, _ctx: &IputByteInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputCharInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputCharInstruction(&mut self, _ctx: &IputCharInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputCharInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputCharInstruction(&mut self, _ctx: &IputCharInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#iputShortInstruction}.
 * @param ctx the parse tree
 */
fn enter_iputShortInstruction(&mut self, _ctx: &IputShortInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#iputShortInstruction}.
 * @param ctx the parse tree
 */
fn exit_iputShortInstruction(&mut self, _ctx: &IputShortInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_addIntInstruction(&mut self, _ctx: &AddIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_addIntInstruction(&mut self, _ctx: &AddIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_subIntInstruction(&mut self, _ctx: &SubIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_subIntInstruction(&mut self, _ctx: &SubIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulIntInstruction(&mut self, _ctx: &MulIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulIntInstruction(&mut self, _ctx: &MulIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_divIntInstruction(&mut self, _ctx: &DivIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_divIntInstruction(&mut self, _ctx: &DivIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_remIntInstruction(&mut self, _ctx: &RemIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_remIntInstruction(&mut self, _ctx: &RemIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_andIntInstruction(&mut self, _ctx: &AndIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_andIntInstruction(&mut self, _ctx: &AndIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_orIntInstruction(&mut self, _ctx: &OrIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_orIntInstruction(&mut self, _ctx: &OrIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_xorIntInstruction(&mut self, _ctx: &XorIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_xorIntInstruction(&mut self, _ctx: &XorIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shlIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_shlIntInstruction(&mut self, _ctx: &ShlIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shlIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_shlIntInstruction(&mut self, _ctx: &ShlIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shrIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_shrIntInstruction(&mut self, _ctx: &ShrIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shrIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_shrIntInstruction(&mut self, _ctx: &ShrIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ushrIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_ushrIntInstruction(&mut self, _ctx: &UshrIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ushrIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_ushrIntInstruction(&mut self, _ctx: &UshrIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#rsubIntInstruction}.
 * @param ctx the parse tree
 */
fn enter_rsubIntInstruction(&mut self, _ctx: &RsubIntInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#rsubIntInstruction}.
 * @param ctx the parse tree
 */
fn exit_rsubIntInstruction(&mut self, _ctx: &RsubIntInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_addLongInstruction(&mut self, _ctx: &AddLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_addLongInstruction(&mut self, _ctx: &AddLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_subLongInstruction(&mut self, _ctx: &SubLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_subLongInstruction(&mut self, _ctx: &SubLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulLongInstruction(&mut self, _ctx: &MulLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulLongInstruction(&mut self, _ctx: &MulLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_divLongInstruction(&mut self, _ctx: &DivLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_divLongInstruction(&mut self, _ctx: &DivLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_remLongInstruction(&mut self, _ctx: &RemLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_remLongInstruction(&mut self, _ctx: &RemLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_andLongInstruction(&mut self, _ctx: &AndLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_andLongInstruction(&mut self, _ctx: &AndLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_orLongInstruction(&mut self, _ctx: &OrLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_orLongInstruction(&mut self, _ctx: &OrLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_xorLongInstruction(&mut self, _ctx: &XorLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_xorLongInstruction(&mut self, _ctx: &XorLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shlLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_shlLongInstruction(&mut self, _ctx: &ShlLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shlLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_shlLongInstruction(&mut self, _ctx: &ShlLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shrLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_shrLongInstruction(&mut self, _ctx: &ShrLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shrLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_shrLongInstruction(&mut self, _ctx: &ShrLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ushrLongInstruction}.
 * @param ctx the parse tree
 */
fn enter_ushrLongInstruction(&mut self, _ctx: &UshrLongInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ushrLongInstruction}.
 * @param ctx the parse tree
 */
fn exit_ushrLongInstruction(&mut self, _ctx: &UshrLongInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_addFloatInstruction(&mut self, _ctx: &AddFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_addFloatInstruction(&mut self, _ctx: &AddFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_subFloatInstruction(&mut self, _ctx: &SubFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_subFloatInstruction(&mut self, _ctx: &SubFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulFloatInstruction(&mut self, _ctx: &MulFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulFloatInstruction(&mut self, _ctx: &MulFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_divFloatInstruction(&mut self, _ctx: &DivFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_divFloatInstruction(&mut self, _ctx: &DivFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remFloatInstruction}.
 * @param ctx the parse tree
 */
fn enter_remFloatInstruction(&mut self, _ctx: &RemFloatInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remFloatInstruction}.
 * @param ctx the parse tree
 */
fn exit_remFloatInstruction(&mut self, _ctx: &RemFloatInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_addDoubleInstruction(&mut self, _ctx: &AddDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_addDoubleInstruction(&mut self, _ctx: &AddDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#subDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_subDoubleInstruction(&mut self, _ctx: &SubDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#subDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_subDoubleInstruction(&mut self, _ctx: &SubDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_mulDoubleInstruction(&mut self, _ctx: &MulDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_mulDoubleInstruction(&mut self, _ctx: &MulDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_divDoubleInstruction(&mut self, _ctx: &DivDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_divDoubleInstruction(&mut self, _ctx: &DivDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remDoubleInstruction}.
 * @param ctx the parse tree
 */
fn enter_remDoubleInstruction(&mut self, _ctx: &RemDoubleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remDoubleInstruction}.
 * @param ctx the parse tree
 */
fn exit_remDoubleInstruction(&mut self, _ctx: &RemDoubleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_addIntLit16Instruction(&mut self, _ctx: &AddIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_addIntLit16Instruction(&mut self, _ctx: &AddIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_mulIntLit16Instruction(&mut self, _ctx: &MulIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_mulIntLit16Instruction(&mut self, _ctx: &MulIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_divIntLit16Instruction(&mut self, _ctx: &DivIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_divIntLit16Instruction(&mut self, _ctx: &DivIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_remIntLit16Instruction(&mut self, _ctx: &RemIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_remIntLit16Instruction(&mut self, _ctx: &RemIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_andIntLit16Instruction(&mut self, _ctx: &AndIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_andIntLit16Instruction(&mut self, _ctx: &AndIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_orIntLit16Instruction(&mut self, _ctx: &OrIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_orIntLit16Instruction(&mut self, _ctx: &OrIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn enter_xorIntLit16Instruction(&mut self, _ctx: &XorIntLit16InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorIntLit16Instruction}.
 * @param ctx the parse tree
 */
fn exit_xorIntLit16Instruction(&mut self, _ctx: &XorIntLit16InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#addIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_addIntLit8Instruction(&mut self, _ctx: &AddIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#addIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_addIntLit8Instruction(&mut self, _ctx: &AddIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#rsubIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_rsubIntLit8Instruction(&mut self, _ctx: &RsubIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#rsubIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_rsubIntLit8Instruction(&mut self, _ctx: &RsubIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#mulIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_mulIntLit8Instruction(&mut self, _ctx: &MulIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#mulIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_mulIntLit8Instruction(&mut self, _ctx: &MulIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#divIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_divIntLit8Instruction(&mut self, _ctx: &DivIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#divIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_divIntLit8Instruction(&mut self, _ctx: &DivIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#remIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_remIntLit8Instruction(&mut self, _ctx: &RemIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#remIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_remIntLit8Instruction(&mut self, _ctx: &RemIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#andIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_andIntLit8Instruction(&mut self, _ctx: &AndIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#andIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_andIntLit8Instruction(&mut self, _ctx: &AndIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#orIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_orIntLit8Instruction(&mut self, _ctx: &OrIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#orIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_orIntLit8Instruction(&mut self, _ctx: &OrIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#xorIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_xorIntLit8Instruction(&mut self, _ctx: &XorIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#xorIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_xorIntLit8Instruction(&mut self, _ctx: &XorIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shlIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_shlIntLit8Instruction(&mut self, _ctx: &ShlIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shlIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_shlIntLit8Instruction(&mut self, _ctx: &ShlIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#shrIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_shrIntLit8Instruction(&mut self, _ctx: &ShrIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#shrIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_shrIntLit8Instruction(&mut self, _ctx: &ShrIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ushrIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn enter_ushrIntLit8Instruction(&mut self, _ctx: &UshrIntLit8InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ushrIntLit8Instruction}.
 * @param ctx the parse tree
 */
fn exit_ushrIntLit8Instruction(&mut self, _ctx: &UshrIntLit8InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#newInstanceType}.
 * @param ctx the parse tree
 */
fn enter_newInstanceType(&mut self, _ctx: &NewInstanceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#newInstanceType}.
 * @param ctx the parse tree
 */
fn exit_newInstanceType(&mut self, _ctx: &NewInstanceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#newInstanceInstruction}.
 * @param ctx the parse tree
 */
fn enter_newInstanceInstruction(&mut self, _ctx: &NewInstanceInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#newInstanceInstruction}.
 * @param ctx the parse tree
 */
fn exit_newInstanceInstruction(&mut self, _ctx: &NewInstanceInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#checkCastType}.
 * @param ctx the parse tree
 */
fn enter_checkCastType(&mut self, _ctx: &CheckCastTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#checkCastType}.
 * @param ctx the parse tree
 */
fn exit_checkCastType(&mut self, _ctx: &CheckCastTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#checkCastInstruction}.
 * @param ctx the parse tree
 */
fn enter_checkCastInstruction(&mut self, _ctx: &CheckCastInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#checkCastInstruction}.
 * @param ctx the parse tree
 */
fn exit_checkCastInstruction(&mut self, _ctx: &CheckCastInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayLengthInstruction}.
 * @param ctx the parse tree
 */
fn enter_arrayLengthInstruction(&mut self, _ctx: &ArrayLengthInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayLengthInstruction}.
 * @param ctx the parse tree
 */
fn exit_arrayLengthInstruction(&mut self, _ctx: &ArrayLengthInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayElementType}.
 * @param ctx the parse tree
 */
fn enter_arrayElementType(&mut self, _ctx: &ArrayElementTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayElementType}.
 * @param ctx the parse tree
 */
fn exit_arrayElementType(&mut self, _ctx: &ArrayElementTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayElementRegisterRange}.
 * @param ctx the parse tree
 */
fn enter_arrayElementRegisterRange(&mut self, _ctx: &ArrayElementRegisterRangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayElementRegisterRange}.
 * @param ctx the parse tree
 */
fn exit_arrayElementRegisterRange(&mut self, _ctx: &ArrayElementRegisterRangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayElementRegisters}.
 * @param ctx the parse tree
 */
fn enter_arrayElementRegisters(&mut self, _ctx: &ArrayElementRegistersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayElementRegisters}.
 * @param ctx the parse tree
 */
fn exit_arrayElementRegisters(&mut self, _ctx: &ArrayElementRegistersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#filledNewArrayRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_filledNewArrayRangeInstruction(&mut self, _ctx: &FilledNewArrayRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#filledNewArrayRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_filledNewArrayRangeInstruction(&mut self, _ctx: &FilledNewArrayRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#filledNewArrayInstruction}.
 * @param ctx the parse tree
 */
fn enter_filledNewArrayInstruction(&mut self, _ctx: &FilledNewArrayInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#filledNewArrayInstruction}.
 * @param ctx the parse tree
 */
fn exit_filledNewArrayInstruction(&mut self, _ctx: &FilledNewArrayInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#filledArrayDataLabel}.
 * @param ctx the parse tree
 */
fn enter_filledArrayDataLabel(&mut self, _ctx: &FilledArrayDataLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#filledArrayDataLabel}.
 * @param ctx the parse tree
 */
fn exit_filledArrayDataLabel(&mut self, _ctx: &FilledArrayDataLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fillArrayDataInstruction}.
 * @param ctx the parse tree
 */
fn enter_fillArrayDataInstruction(&mut self, _ctx: &FillArrayDataInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fillArrayDataInstruction}.
 * @param ctx the parse tree
 */
fn exit_fillArrayDataInstruction(&mut self, _ctx: &FillArrayDataInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#checkInstanceType}.
 * @param ctx the parse tree
 */
fn enter_checkInstanceType(&mut self, _ctx: &CheckInstanceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#checkInstanceType}.
 * @param ctx the parse tree
 */
fn exit_checkInstanceType(&mut self, _ctx: &CheckInstanceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#instanceOfInstruction}.
 * @param ctx the parse tree
 */
fn enter_instanceOfInstruction(&mut self, _ctx: &InstanceOfInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#instanceOfInstruction}.
 * @param ctx the parse tree
 */
fn exit_instanceOfInstruction(&mut self, _ctx: &InstanceOfInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arraySizeRegister}.
 * @param ctx the parse tree
 */
fn enter_arraySizeRegister(&mut self, _ctx: &ArraySizeRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arraySizeRegister}.
 * @param ctx the parse tree
 */
fn exit_arraySizeRegister(&mut self, _ctx: &ArraySizeRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#newArrayInstruction}.
 * @param ctx the parse tree
 */
fn enter_newArrayInstruction(&mut self, _ctx: &NewArrayInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#newArrayInstruction}.
 * @param ctx the parse tree
 */
fn exit_newArrayInstruction(&mut self, _ctx: &NewArrayInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchRegister}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchRegister(&mut self, _ctx: &PackedSwitchRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchRegister}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchRegister(&mut self, _ctx: &PackedSwitchRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchLabel}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchLabel(&mut self, _ctx: &PackedSwitchLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchLabel}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchLabel(&mut self, _ctx: &PackedSwitchLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sparseSwitchRegister}.
 * @param ctx the parse tree
 */
fn enter_sparseSwitchRegister(&mut self, _ctx: &SparseSwitchRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sparseSwitchRegister}.
 * @param ctx the parse tree
 */
fn exit_sparseSwitchRegister(&mut self, _ctx: &SparseSwitchRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sparseSwitchLabel}.
 * @param ctx the parse tree
 */
fn enter_sparseSwitchLabel(&mut self, _ctx: &SparseSwitchLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sparseSwitchLabel}.
 * @param ctx the parse tree
 */
fn exit_sparseSwitchLabel(&mut self, _ctx: &SparseSwitchLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchInstruction}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchInstruction(&mut self, _ctx: &PackedSwitchInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchInstruction}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchInstruction(&mut self, _ctx: &PackedSwitchInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sparseSwitchInstruction}.
 * @param ctx the parse tree
 */
fn enter_sparseSwitchInstruction(&mut self, _ctx: &SparseSwitchInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sparseSwitchInstruction}.
 * @param ctx the parse tree
 */
fn exit_sparseSwitchInstruction(&mut self, _ctx: &SparseSwitchInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokePolymorphicInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokePolymorphicInstruction(&mut self, _ctx: &InvokePolymorphicInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokePolymorphicInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokePolymorphicInstruction(&mut self, _ctx: &InvokePolymorphicInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokePolymorphicRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokePolymorphicRangeInstruction(&mut self, _ctx: &InvokePolymorphicRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokePolymorphicRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokePolymorphicRangeInstruction(&mut self, _ctx: &InvokePolymorphicRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeCustomInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeCustomInstruction(&mut self, _ctx: &InvokeCustomInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeCustomInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeCustomInstruction(&mut self, _ctx: &InvokeCustomInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeCustomRangeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeCustomRangeInstruction(&mut self, _ctx: &InvokeCustomRangeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeCustomRangeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeCustomRangeInstruction(&mut self, _ctx: &InvokeCustomRangeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeConstMethodHandleInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeConstMethodHandleInstruction(&mut self, _ctx: &InvokeConstMethodHandleInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeConstMethodHandleInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeConstMethodHandleInstruction(&mut self, _ctx: &InvokeConstMethodHandleInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#invokeConstMethodTypeInstruction}.
 * @param ctx the parse tree
 */
fn enter_invokeConstMethodTypeInstruction(&mut self, _ctx: &InvokeConstMethodTypeInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#invokeConstMethodTypeInstruction}.
 * @param ctx the parse tree
 */
fn exit_invokeConstMethodTypeInstruction(&mut self, _ctx: &InvokeConstMethodTypeInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#binaryInstruction}.
 * @param ctx the parse tree
 */
fn enter_binaryInstruction(&mut self, _ctx: &BinaryInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#binaryInstruction}.
 * @param ctx the parse tree
 */
fn exit_binaryInstruction(&mut self, _ctx: &BinaryInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#ternaryInstruction}.
 * @param ctx the parse tree
 */
fn enter_ternaryInstruction(&mut self, _ctx: &TernaryInstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#ternaryInstruction}.
 * @param ctx the parse tree
 */
fn exit_ternaryInstruction(&mut self, _ctx: &TernaryInstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodInvocationTarget}.
 * @param ctx the parse tree
 */
fn enter_methodInvocationTarget(&mut self, _ctx: &MethodInvocationTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodInvocationTarget}.
 * @param ctx the parse tree
 */
fn exit_methodInvocationTarget(&mut self, _ctx: &MethodInvocationTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldInvocationTarget}.
 * @param ctx the parse tree
 */
fn enter_fieldInvocationTarget(&mut self, _ctx: &FieldInvocationTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldInvocationTarget}.
 * @param ctx the parse tree
 */
fn exit_fieldInvocationTarget(&mut self, _ctx: &FieldInvocationTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldName}.
 * @param ctx the parse tree
 */
fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldName}.
 * @param ctx the parse tree
 */
fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldType}.
 * @param ctx the parse tree
 */
fn enter_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldType}.
 * @param ctx the parse tree
 */
fn exit_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldNameAndType}.
 * @param ctx the parse tree
 */
fn enter_fieldNameAndType(&mut self, _ctx: &FieldNameAndTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldNameAndType}.
 * @param ctx the parse tree
 */
fn exit_fieldNameAndType(&mut self, _ctx: &FieldNameAndTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#fieldDirective}.
 * @param ctx the parse tree
 */
fn enter_fieldDirective(&mut self, _ctx: &FieldDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#fieldDirective}.
 * @param ctx the parse tree
 */
fn exit_fieldDirective(&mut self, _ctx: &FieldDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#className}.
 * @param ctx the parse tree
 */
fn enter_className(&mut self, _ctx: &ClassNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#className}.
 * @param ctx the parse tree
 */
fn exit_className(&mut self, _ctx: &ClassNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#classDirective}.
 * @param ctx the parse tree
 */
fn enter_classDirective(&mut self, _ctx: &ClassDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#classDirective}.
 * @param ctx the parse tree
 */
fn exit_classDirective(&mut self, _ctx: &ClassDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#superName}.
 * @param ctx the parse tree
 */
fn enter_superName(&mut self, _ctx: &SuperNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#superName}.
 * @param ctx the parse tree
 */
fn exit_superName(&mut self, _ctx: &SuperNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#superDirective}.
 * @param ctx the parse tree
 */
fn enter_superDirective(&mut self, _ctx: &SuperDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#superDirective}.
 * @param ctx the parse tree
 */
fn exit_superDirective(&mut self, _ctx: &SuperDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sourceName}.
 * @param ctx the parse tree
 */
fn enter_sourceName(&mut self, _ctx: &SourceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sourceName}.
 * @param ctx the parse tree
 */
fn exit_sourceName(&mut self, _ctx: &SourceNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sourceDirective}.
 * @param ctx the parse tree
 */
fn enter_sourceDirective(&mut self, _ctx: &SourceDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sourceDirective}.
 * @param ctx the parse tree
 */
fn exit_sourceDirective(&mut self, _ctx: &SourceDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodIdentifier}.
 * @param ctx the parse tree
 */
fn enter_methodIdentifier(&mut self, _ctx: &MethodIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodIdentifier}.
 * @param ctx the parse tree
 */
fn exit_methodIdentifier(&mut self, _ctx: &MethodIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodReturnType}.
 * @param ctx the parse tree
 */
fn enter_methodReturnType(&mut self, _ctx: &MethodReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodReturnType}.
 * @param ctx the parse tree
 */
fn exit_methodReturnType(&mut self, _ctx: &MethodReturnTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodParameterType}.
 * @param ctx the parse tree
 */
fn enter_methodParameterType(&mut self, _ctx: &MethodParameterTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodParameterType}.
 * @param ctx the parse tree
 */
fn exit_methodParameterType(&mut self, _ctx: &MethodParameterTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodArguments}.
 * @param ctx the parse tree
 */
fn enter_methodArguments(&mut self, _ctx: &MethodArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodArguments}.
 * @param ctx the parse tree
 */
fn exit_methodArguments(&mut self, _ctx: &MethodArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodSignature}.
 * @param ctx the parse tree
 */
fn enter_methodSignature(&mut self, _ctx: &MethodSignatureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodSignature}.
 * @param ctx the parse tree
 */
fn exit_methodSignature(&mut self, _ctx: &MethodSignatureContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationScope}.
 * @param ctx the parse tree
 */
fn enter_annotationScope(&mut self, _ctx: &AnnotationScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationScope}.
 * @param ctx the parse tree
 */
fn exit_annotationScope(&mut self, _ctx: &AnnotationScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationType}.
 * @param ctx the parse tree
 */
fn enter_annotationType(&mut self, _ctx: &AnnotationTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationType}.
 * @param ctx the parse tree
 */
fn exit_annotationType(&mut self, _ctx: &AnnotationTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationFieldValue}.
 * @param ctx the parse tree
 */
fn enter_annotationFieldValue(&mut self, _ctx: &AnnotationFieldValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationFieldValue}.
 * @param ctx the parse tree
 */
fn exit_annotationFieldValue(&mut self, _ctx: &AnnotationFieldValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationValueScoped}.
 * @param ctx the parse tree
 */
fn enter_annotationValueScoped(&mut self, _ctx: &AnnotationValueScopedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationValueScoped}.
 * @param ctx the parse tree
 */
fn exit_annotationValueScoped(&mut self, _ctx: &AnnotationValueScopedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationField}.
 * @param ctx the parse tree
 */
fn enter_annotationField(&mut self, _ctx: &AnnotationFieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationField}.
 * @param ctx the parse tree
 */
fn exit_annotationField(&mut self, _ctx: &AnnotationFieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#annotationDirective}.
 * @param ctx the parse tree
 */
fn enter_annotationDirective(&mut self, _ctx: &AnnotationDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#annotationDirective}.
 * @param ctx the parse tree
 */
fn exit_annotationDirective(&mut self, _ctx: &AnnotationDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#locaDirectiveVariableName}.
 * @param ctx the parse tree
 */
fn enter_locaDirectiveVariableName(&mut self, _ctx: &LocaDirectiveVariableNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#locaDirectiveVariableName}.
 * @param ctx the parse tree
 */
fn exit_locaDirectiveVariableName(&mut self, _ctx: &LocaDirectiveVariableNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localDirectiveType}.
 * @param ctx the parse tree
 */
fn enter_localDirectiveType(&mut self, _ctx: &LocalDirectiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localDirectiveType}.
 * @param ctx the parse tree
 */
fn exit_localDirectiveType(&mut self, _ctx: &LocalDirectiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localDirectiveGenericHint}.
 * @param ctx the parse tree
 */
fn enter_localDirectiveGenericHint(&mut self, _ctx: &LocalDirectiveGenericHintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localDirectiveGenericHint}.
 * @param ctx the parse tree
 */
fn exit_localDirectiveGenericHint(&mut self, _ctx: &LocalDirectiveGenericHintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localDirectiveRegister}.
 * @param ctx the parse tree
 */
fn enter_localDirectiveRegister(&mut self, _ctx: &LocalDirectiveRegisterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localDirectiveRegister}.
 * @param ctx the parse tree
 */
fn exit_localDirectiveRegister(&mut self, _ctx: &LocalDirectiveRegisterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localDirective}.
 * @param ctx the parse tree
 */
fn enter_localDirective(&mut self, _ctx: &LocalDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localDirective}.
 * @param ctx the parse tree
 */
fn exit_localDirective(&mut self, _ctx: &LocalDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localEndDirective}.
 * @param ctx the parse tree
 */
fn enter_localEndDirective(&mut self, _ctx: &LocalEndDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localEndDirective}.
 * @param ctx the parse tree
 */
fn exit_localEndDirective(&mut self, _ctx: &LocalEndDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localRestartDirective}.
 * @param ctx the parse tree
 */
fn enter_localRestartDirective(&mut self, _ctx: &LocalRestartDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localRestartDirective}.
 * @param ctx the parse tree
 */
fn exit_localRestartDirective(&mut self, _ctx: &LocalRestartDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#lineLabel}.
 * @param ctx the parse tree
 */
fn enter_lineLabel(&mut self, _ctx: &LineLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#lineLabel}.
 * @param ctx the parse tree
 */
fn exit_lineLabel(&mut self, _ctx: &LineLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodBodyStatement}.
 * @param ctx the parse tree
 */
fn enter_methodBodyStatement(&mut self, _ctx: &MethodBodyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodBodyStatement}.
 * @param ctx the parse tree
 */
fn exit_methodBodyStatement(&mut self, _ctx: &MethodBodyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodBody}.
 * @param ctx the parse tree
 */
fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodBody}.
 * @param ctx the parse tree
 */
fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchIdent}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchIdent(&mut self, _ctx: &PackedSwitchIdentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchIdent}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchIdent(&mut self, _ctx: &PackedSwitchIdentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchDirectiveLabel}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchDirectiveLabel(&mut self, _ctx: &PackedSwitchDirectiveLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchDirectiveLabel}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchDirectiveLabel(&mut self, _ctx: &PackedSwitchDirectiveLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchDirectiveLabels}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchDirectiveLabels(&mut self, _ctx: &PackedSwitchDirectiveLabelsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchDirectiveLabels}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchDirectiveLabels(&mut self, _ctx: &PackedSwitchDirectiveLabelsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#packedSwitchDirective}.
 * @param ctx the parse tree
 */
fn enter_packedSwitchDirective(&mut self, _ctx: &PackedSwitchDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#packedSwitchDirective}.
 * @param ctx the parse tree
 */
fn exit_packedSwitchDirective(&mut self, _ctx: &PackedSwitchDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#methodDirective}.
 * @param ctx the parse tree
 */
fn enter_methodDirective(&mut self, _ctx: &MethodDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#methodDirective}.
 * @param ctx the parse tree
 */
fn exit_methodDirective(&mut self, _ctx: &MethodDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#registersDirective}.
 * @param ctx the parse tree
 */
fn enter_registersDirective(&mut self, _ctx: &RegistersDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#registersDirective}.
 * @param ctx the parse tree
 */
fn exit_registersDirective(&mut self, _ctx: &RegistersDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#localsDirective}.
 * @param ctx the parse tree
 */
fn enter_localsDirective(&mut self, _ctx: &LocalsDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#localsDirective}.
 * @param ctx the parse tree
 */
fn exit_localsDirective(&mut self, _ctx: &LocalsDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#simpleParamDirective}.
 * @param ctx the parse tree
 */
fn enter_simpleParamDirective(&mut self, _ctx: &SimpleParamDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#simpleParamDirective}.
 * @param ctx the parse tree
 */
fn exit_simpleParamDirective(&mut self, _ctx: &SimpleParamDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#extendedParamDirective}.
 * @param ctx the parse tree
 */
fn enter_extendedParamDirective(&mut self, _ctx: &ExtendedParamDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#extendedParamDirective}.
 * @param ctx the parse tree
 */
fn exit_extendedParamDirective(&mut self, _ctx: &ExtendedParamDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#paramDirective}.
 * @param ctx the parse tree
 */
fn enter_paramDirective(&mut self, _ctx: &ParamDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#paramDirective}.
 * @param ctx the parse tree
 */
fn exit_paramDirective(&mut self, _ctx: &ParamDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#lineDirective}.
 * @param ctx the parse tree
 */
fn enter_lineDirective(&mut self, _ctx: &LineDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#lineDirective}.
 * @param ctx the parse tree
 */
fn exit_lineDirective(&mut self, _ctx: &LineDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchFromLabel}.
 * @param ctx the parse tree
 */
fn enter_catchFromLabel(&mut self, _ctx: &CatchFromLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchFromLabel}.
 * @param ctx the parse tree
 */
fn exit_catchFromLabel(&mut self, _ctx: &CatchFromLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchToLabel}.
 * @param ctx the parse tree
 */
fn enter_catchToLabel(&mut self, _ctx: &CatchToLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchToLabel}.
 * @param ctx the parse tree
 */
fn exit_catchToLabel(&mut self, _ctx: &CatchToLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchGotoLabel}.
 * @param ctx the parse tree
 */
fn enter_catchGotoLabel(&mut self, _ctx: &CatchGotoLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchGotoLabel}.
 * @param ctx the parse tree
 */
fn exit_catchGotoLabel(&mut self, _ctx: &CatchGotoLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchExceptionType}.
 * @param ctx the parse tree
 */
fn enter_catchExceptionType(&mut self, _ctx: &CatchExceptionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchExceptionType}.
 * @param ctx the parse tree
 */
fn exit_catchExceptionType(&mut self, _ctx: &CatchExceptionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchDirective}.
 * @param ctx the parse tree
 */
fn enter_catchDirective(&mut self, _ctx: &CatchDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchDirective}.
 * @param ctx the parse tree
 */
fn exit_catchDirective(&mut self, _ctx: &CatchDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#catchAllDirective}.
 * @param ctx the parse tree
 */
fn enter_catchAllDirective(&mut self, _ctx: &CatchAllDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#catchAllDirective}.
 * @param ctx the parse tree
 */
fn exit_catchAllDirective(&mut self, _ctx: &CatchAllDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayDataDirective}.
 * @param ctx the parse tree
 */
fn enter_arrayDataDirective(&mut self, _ctx: &ArrayDataDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayDataDirective}.
 * @param ctx the parse tree
 */
fn exit_arrayDataDirective(&mut self, _ctx: &ArrayDataDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#arrayDataEntry}.
 * @param ctx the parse tree
 */
fn enter_arrayDataEntry(&mut self, _ctx: &ArrayDataEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#arrayDataEntry}.
 * @param ctx the parse tree
 */
fn exit_arrayDataEntry(&mut self, _ctx: &ArrayDataEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sparseSwitchDirectiveValue}.
 * @param ctx the parse tree
 */
fn enter_sparseSwitchDirectiveValue(&mut self, _ctx: &SparseSwitchDirectiveValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sparseSwitchDirectiveValue}.
 * @param ctx the parse tree
 */
fn exit_sparseSwitchDirectiveValue(&mut self, _ctx: &SparseSwitchDirectiveValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#sparseSwitchDirective}.
 * @param ctx the parse tree
 */
fn enter_sparseSwitchDirective(&mut self, _ctx: &SparseSwitchDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#sparseSwitchDirective}.
 * @param ctx the parse tree
 */
fn exit_sparseSwitchDirective(&mut self, _ctx: &SparseSwitchDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SmaliParser#parse}.
 * @param ctx the parse tree
 */
fn enter_parse(&mut self, _ctx: &ParseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SmaliParser#parse}.
 * @param ctx the parse tree
 */
fn exit_parse(&mut self, _ctx: &ParseContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SmaliParserListener<'input> }


