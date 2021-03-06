//! `ast` standard module for abstract syntax trees.
//!
//! This module makes use of the parser logic, and translates all ast nodes
//! into python ast.AST objects.

use std::ops::Deref;

use num_complex::Complex64;

use rustpython_parser::{ast, parser};

use crate::function::PyFuncArgs;
use crate::obj::objstr;
use crate::obj::objtype::PyClassRef;
use crate::pyobject::{PyContext, PyObject, PyObjectRef, PyResult, PyValue, TypeProtocol};
use crate::vm::VirtualMachine;

#[derive(Debug)]
struct AstNode;
// type AstNodeRef = PyRef<AstNode>;

impl PyValue for AstNode {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.class("ast", "AST")
    }
}

/*
 * Idea: maybe we can create a sort of struct with some helper functions?
struct AstToPyAst {
    ctx: &PyContext,
}

impl AstToPyAst {
    fn new(ctx: &PyContext) -> Self {
        AstToPyAst {
            ctx: ctx,
        }
    }

}
*/

fn program_to_ast(vm: &VirtualMachine, program: &ast::Program) -> PyObjectRef {
    let mut body = vec![];
    for statement in &program.statements {
        body.push(statement_to_ast(&vm, statement));
    }
    // TODO: create Module node:
    // let ast_node = ctx.new_instance(this.Module);
    let ast_node = create_node(vm, "program");
    let py_body = vm.ctx.new_list(body);
    vm.ctx.set_attr(&ast_node, "body", py_body);
    ast_node
}

// Create a node class instance
fn create_node(vm: &VirtualMachine, _name: &str) -> PyObjectRef {
    // TODO: instantiate a class of type given by name
    // TODO: lookup in the current module?
    PyObject::new(AstNode, AstNode::class(vm), Some(vm.ctx.new_dict()))
}

fn statements_to_ast(vm: &VirtualMachine, statements: &[ast::LocatedStatement]) -> PyObjectRef {
    let mut py_statements = vec![];
    for statement in statements {
        py_statements.push(statement_to_ast(vm, statement));
    }
    vm.ctx.new_list(py_statements)
}

fn statement_to_ast(vm: &VirtualMachine, statement: &ast::LocatedStatement) -> PyObjectRef {
    let node = match &statement.node {
        ast::Statement::ClassDef {
            name,
            body,
            decorator_list,
            ..
        } => {
            let node = create_node(vm, "ClassDef");

            // Set name:
            vm.ctx
                .set_attr(&node, "name", vm.ctx.new_str(name.to_string()));

            // Set body:
            let py_body = statements_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_decorator_list = expressions_to_ast(vm, decorator_list);
            vm.ctx.set_attr(&node, "decorator_list", py_decorator_list);
            node
        }
        ast::Statement::FunctionDef {
            name,
            args,
            body,
            decorator_list,
            returns,
        } => {
            let node = create_node(vm, "FunctionDef");

            // Set name:
            vm.ctx
                .set_attr(&node, "name", vm.ctx.new_str(name.to_string()));

            vm.ctx.set_attr(&node, "args", parameters_to_ast(vm, args));

            // Set body:
            let py_body = statements_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_decorator_list = expressions_to_ast(vm, decorator_list);
            vm.ctx.set_attr(&node, "decorator_list", py_decorator_list);

            let py_returns = if let Some(hint) = returns {
                expression_to_ast(vm, hint)
            } else {
                vm.ctx.none()
            };
            vm.ctx.set_attr(&node, "returns", py_returns);
            node
        }
        ast::Statement::Continue => create_node(vm, "Continue"),
        ast::Statement::Break => create_node(vm, "Break"),
        ast::Statement::Pass => create_node(vm, "Pass"),
        ast::Statement::Assert { test, msg } => {
            let node = create_node(vm, "Pass");

            vm.ctx.set_attr(&node, "test", expression_to_ast(vm, test));

            let py_msg = match msg {
                Some(msg) => expression_to_ast(vm, msg),
                None => vm.ctx.none(),
            };
            vm.ctx.set_attr(&node, "msg", py_msg);

            node
        }
        ast::Statement::Delete { targets } => {
            let node = create_node(vm, "Delete");

            let py_targets = vm
                .ctx
                .new_tuple(targets.iter().map(|v| expression_to_ast(vm, v)).collect());
            vm.ctx.set_attr(&node, "targets", py_targets);

            node
        }
        ast::Statement::Return { value } => {
            let node = create_node(vm, "Return");

            let py_value = if let Some(value) = value {
                vm.ctx
                    .new_tuple(value.iter().map(|v| expression_to_ast(vm, v)).collect())
            } else {
                vm.ctx.none()
            };
            vm.ctx.set_attr(&node, "value", py_value);

            node
        }
        ast::Statement::If { test, body, orelse } => {
            let node = create_node(vm, "If");

            let py_test = expression_to_ast(vm, test);
            vm.ctx.set_attr(&node, "test", py_test);

            let py_body = statements_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_orelse = if let Some(orelse) = orelse {
                statements_to_ast(vm, orelse)
            } else {
                vm.ctx.none()
            };
            vm.ctx.set_attr(&node, "orelse", py_orelse);

            node
        }
        ast::Statement::For {
            target,
            iter,
            body,
            orelse,
        } => {
            let node = create_node(vm, "For");

            let py_target = expression_to_ast(vm, target);
            vm.ctx.set_attr(&node, "target", py_target);

            let py_iter = expressions_to_ast(vm, iter);
            vm.ctx.set_attr(&node, "iter", py_iter);

            let py_body = statements_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_orelse = if let Some(orelse) = orelse {
                statements_to_ast(vm, orelse)
            } else {
                vm.ctx.none()
            };
            vm.ctx.set_attr(&node, "orelse", py_orelse);

            node
        }
        ast::Statement::While { test, body, orelse } => {
            let node = create_node(vm, "While");

            let py_test = expression_to_ast(vm, test);
            vm.ctx.set_attr(&node, "test", py_test);

            let py_body = statements_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_orelse = if let Some(orelse) = orelse {
                statements_to_ast(vm, orelse)
            } else {
                vm.ctx.none()
            };
            vm.ctx.set_attr(&node, "orelse", py_orelse);

            node
        }
        ast::Statement::Expression { expression } => {
            let node = create_node(vm, "Expr");

            let value = expression_to_ast(vm, expression);
            vm.ctx.set_attr(&node, "value", value);

            node
        }
        x => {
            unimplemented!("{:?}", x);
        }
    };

    // set lineno on node:
    let lineno = vm.ctx.new_int(statement.location.get_row());
    vm.ctx.set_attr(&node, "lineno", lineno);

    node
}

fn expressions_to_ast(vm: &VirtualMachine, expressions: &[ast::Expression]) -> PyObjectRef {
    let mut py_expression_nodes = vec![];
    for expression in expressions {
        py_expression_nodes.push(expression_to_ast(vm, expression));
    }
    vm.ctx.new_list(py_expression_nodes)
}

fn expression_to_ast(vm: &VirtualMachine, expression: &ast::Expression) -> PyObjectRef {
    let node = match &expression {
        ast::Expression::Call { function, args, .. } => {
            let node = create_node(vm, "Call");

            let py_func_ast = expression_to_ast(vm, function);
            vm.ctx.set_attr(&node, "func", py_func_ast);

            let py_args = expressions_to_ast(vm, args);
            vm.ctx.set_attr(&node, "args", py_args);

            node
        }
        ast::Expression::Binop { a, op, b } => {
            let node = create_node(vm, "BinOp");

            let py_a = expression_to_ast(vm, a);
            vm.ctx.set_attr(&node, "left", py_a);

            // Operator:
            let str_op = match op {
                ast::Operator::Add => "Add",
                ast::Operator::Sub => "Sub",
                ast::Operator::Mult => "Mult",
                ast::Operator::MatMult => "MatMult",
                ast::Operator::Div => "Div",
                ast::Operator::Mod => "Mod",
                ast::Operator::Pow => "Pow",
                ast::Operator::LShift => "LShift",
                ast::Operator::RShift => "RShift",
                ast::Operator::BitOr => "BitOr",
                ast::Operator::BitXor => "BitXor",
                ast::Operator::BitAnd => "BitAnd",
                ast::Operator::FloorDiv => "FloorDiv",
            };
            let py_op = vm.ctx.new_str(str_op.to_string());
            vm.ctx.set_attr(&node, "op", py_op);

            let py_b = expression_to_ast(vm, b);
            vm.ctx.set_attr(&node, "right", py_b);
            node
        }
        ast::Expression::Unop { op, a } => {
            let node = create_node(vm, "UnaryOp");

            let str_op = match op {
                ast::UnaryOperator::Not => "Not",
                ast::UnaryOperator::Inv => "Invert",
                ast::UnaryOperator::Neg => "USub",
                ast::UnaryOperator::Pos => "UAdd",
            };
            let py_op = vm.ctx.new_str(str_op.to_string());
            vm.ctx.set_attr(&node, "op", py_op);

            let py_a = expression_to_ast(vm, a);
            vm.ctx.set_attr(&node, "operand", py_a);

            node
        }
        ast::Expression::BoolOp { a, op, b } => {
            let node = create_node(vm, "BoolOp");

            // Attach values:
            let py_a = expression_to_ast(vm, a);
            let py_b = expression_to_ast(vm, b);
            let py_values = vm.ctx.new_tuple(vec![py_a, py_b]);
            vm.ctx.set_attr(&node, "values", py_values);

            let str_op = match op {
                ast::BooleanOperator::And => "And",
                ast::BooleanOperator::Or => "Or",
            };
            let py_op = vm.ctx.new_str(str_op.to_string());
            vm.ctx.set_attr(&node, "op", py_op);

            node
        }
        ast::Expression::Compare { a, op, b } => {
            let node = create_node(vm, "Compare");

            let py_a = expression_to_ast(vm, a);
            vm.ctx.set_attr(&node, "left", py_a);

            // Operator:
            let str_op = match op {
                ast::Comparison::Equal => "Eq",
                ast::Comparison::NotEqual => "NotEq",
                ast::Comparison::Less => "Lt",
                ast::Comparison::LessOrEqual => "LtE",
                ast::Comparison::Greater => "Gt",
                ast::Comparison::GreaterOrEqual => "GtE",
                ast::Comparison::In => "In",
                ast::Comparison::NotIn => "NotIn",
                ast::Comparison::Is => "Is",
                ast::Comparison::IsNot => "IsNot",
            };
            let py_ops = vm.ctx.new_list(vec![vm.ctx.new_str(str_op.to_string())]);
            vm.ctx.set_attr(&node, "ops", py_ops);

            let py_b = vm.ctx.new_list(vec![expression_to_ast(vm, b)]);
            vm.ctx.set_attr(&node, "comparators", py_b);
            node
        }
        ast::Expression::Identifier { name } => {
            let node = create_node(vm, "Identifier");

            // Id:
            let py_name = vm.ctx.new_str(name.clone());
            vm.ctx.set_attr(&node, "id", py_name);
            node
        }
        ast::Expression::Lambda { args, body } => {
            let node = create_node(vm, "Lambda");

            vm.ctx.set_attr(&node, "args", parameters_to_ast(vm, args));

            let py_body = expression_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            node
        }
        ast::Expression::IfExpression { test, body, orelse } => {
            let node = create_node(vm, "IfExp");

            let py_test = expression_to_ast(vm, test);
            vm.ctx.set_attr(&node, "test", py_test);

            let py_body = expression_to_ast(vm, body);
            vm.ctx.set_attr(&node, "body", py_body);

            let py_orelse = expression_to_ast(vm, orelse);
            vm.ctx.set_attr(&node, "orelse", py_orelse);

            node
        }
        ast::Expression::Number { value } => {
            let node = create_node(vm, "Num");

            let py_n = match value {
                ast::Number::Integer { value } => vm.ctx.new_int(value.clone()),
                ast::Number::Float { value } => vm.ctx.new_float(*value),
                ast::Number::Complex { real, imag } => {
                    vm.ctx.new_complex(Complex64::new(*real, *imag))
                }
            };
            vm.ctx.set_attr(&node, "n", py_n);

            node
        }
        ast::Expression::True => {
            let node = create_node(vm, "NameConstant");

            vm.ctx.set_attr(&node, "value", vm.ctx.new_bool(true));

            node
        }
        ast::Expression::False => {
            let node = create_node(vm, "NameConstant");

            vm.ctx.set_attr(&node, "value", vm.ctx.new_bool(false));

            node
        }
        ast::Expression::None => {
            let node = create_node(vm, "NameConstant");

            vm.ctx.set_attr(&node, "value", vm.ctx.none());

            node
        }
        ast::Expression::Ellipsis => create_node(vm, "Ellipsis"),
        ast::Expression::List { elements } => {
            let node = create_node(vm, "List");

            let elts = elements.iter().map(|e| expression_to_ast(vm, e)).collect();
            let py_elts = vm.ctx.new_list(elts);
            vm.ctx.set_attr(&node, "elts", py_elts);

            node
        }
        ast::Expression::Tuple { elements } => {
            let node = create_node(vm, "Tuple");

            let elts = elements.iter().map(|e| expression_to_ast(vm, e)).collect();
            let py_elts = vm.ctx.new_list(elts);
            vm.ctx.set_attr(&node, "elts", py_elts);

            node
        }
        ast::Expression::Set { elements } => {
            let node = create_node(vm, "Set");

            let elts = elements.iter().map(|e| expression_to_ast(vm, e)).collect();
            let py_elts = vm.ctx.new_list(elts);
            vm.ctx.set_attr(&node, "elts", py_elts);

            node
        }
        ast::Expression::Dict { elements } => {
            let node = create_node(vm, "Dict");

            let mut keys = Vec::new();
            let mut values = Vec::new();
            for (k, v) in elements {
                keys.push(expression_to_ast(vm, k));
                values.push(expression_to_ast(vm, v));
            }

            let py_keys = vm.ctx.new_list(keys);
            vm.ctx.set_attr(&node, "keys", py_keys);

            let py_values = vm.ctx.new_list(values);
            vm.ctx.set_attr(&node, "values", py_values);

            node
        }
        ast::Expression::Comprehension { kind, generators } => {
            let node = match kind.deref() {
                ast::ComprehensionKind::GeneratorExpression { .. } => {
                    create_node(vm, "GeneratorExp")
                }
                ast::ComprehensionKind::List { .. } => create_node(vm, "ListComp"),
                ast::ComprehensionKind::Set { .. } => create_node(vm, "SetComp"),
                ast::ComprehensionKind::Dict { .. } => create_node(vm, "DictComp"),
            };

            let g = generators
                .iter()
                .map(|g| comprehension_to_ast(vm, g))
                .collect();
            let py_generators = vm.ctx.new_list(g);
            vm.ctx.set_attr(&node, "generators", py_generators);

            node
        }
        ast::Expression::Yield { value } => {
            let node = create_node(vm, "Yield");

            let py_value = match value {
                Some(value) => expression_to_ast(vm, value),
                None => vm.ctx.none(),
            };
            vm.ctx.set_attr(&node, "value", py_value);

            node
        }
        ast::Expression::YieldFrom { value } => {
            let node = create_node(vm, "YieldFrom");

            let py_value = expression_to_ast(vm, value);
            vm.ctx.set_attr(&node, "value", py_value);

            node
        }
        ast::Expression::Subscript { a, b } => {
            let node = create_node(vm, "Subscript");

            let py_value = expression_to_ast(vm, a);
            vm.ctx.set_attr(&node, "value", py_value);

            let py_slice = expression_to_ast(vm, b);
            vm.ctx.set_attr(&node, "slice", py_slice);

            node
        }
        ast::Expression::Attribute { value, name } => {
            let node = create_node(vm, "Attribute");

            let py_value = expression_to_ast(vm, value);
            vm.ctx.set_attr(&node, "value", py_value);

            let py_attr = vm.ctx.new_str(name.to_string());
            vm.ctx.set_attr(&node, "attr", py_attr);

            node
        }
        ast::Expression::Starred { value } => {
            let node = create_node(vm, "Starred");

            let py_value = expression_to_ast(vm, value);
            vm.ctx.set_attr(&node, "value", py_value);

            node
        }
        ast::Expression::Slice { elements } => {
            let node = create_node(vm, "Slice");

            let py_value = expressions_to_ast(vm, elements);
            vm.ctx.set_attr(&node, "bounds", py_value);

            node
        }
        ast::Expression::String { value } => string_to_ast(vm, value),
        ast::Expression::Bytes { value } => {
            let node = create_node(vm, "Bytes");
            vm.ctx.set_attr(&node, "s", vm.ctx.new_bytes(value.clone()));
            node
        }
    };

    // TODO: retrieve correct lineno:
    let lineno = vm.ctx.new_int(1);
    vm.ctx.set_attr(&node, "lineno", lineno);

    node
}

fn parameters_to_ast(vm: &VirtualMachine, args: &ast::Parameters) -> PyObjectRef {
    let node = create_node(vm, "arguments");

    vm.ctx.set_attr(
        &node,
        "args",
        vm.ctx
            .new_list(args.args.iter().map(|a| parameter_to_ast(vm, a)).collect()),
    );

    node
}

fn parameter_to_ast(vm: &VirtualMachine, parameter: &ast::Parameter) -> PyObjectRef {
    let node = create_node(vm, "arg");

    let py_arg = vm.ctx.new_str(parameter.arg.to_string());
    vm.ctx.set_attr(&node, "arg", py_arg);

    let py_annotation = if let Some(annotation) = &parameter.annotation {
        expression_to_ast(vm, annotation)
    } else {
        vm.ctx.none()
    };
    vm.ctx.set_attr(&node, "annotation", py_annotation);

    node
}

fn comprehension_to_ast(vm: &VirtualMachine, comprehension: &ast::Comprehension) -> PyObjectRef {
    let node = create_node(vm, "comprehension");

    let py_target = expression_to_ast(vm, &comprehension.target);
    vm.ctx.set_attr(&node, "target", py_target);

    let py_iter = expression_to_ast(vm, &comprehension.iter);
    vm.ctx.set_attr(&node, "iter", py_iter);

    let py_ifs = expressions_to_ast(vm, &comprehension.ifs);
    vm.ctx.set_attr(&node, "ifs", py_ifs);

    node
}

fn string_to_ast(vm: &VirtualMachine, string: &ast::StringGroup) -> PyObjectRef {
    match string {
        ast::StringGroup::Constant { value } => {
            let node = create_node(vm, "Str");
            vm.ctx.set_attr(&node, "s", vm.ctx.new_str(value.clone()));
            node
        }
        ast::StringGroup::FormattedValue { value, .. } => {
            let node = create_node(vm, "FormattedValue");
            let py_value = expression_to_ast(vm, value);
            vm.ctx.set_attr(&node, "value", py_value);
            node
        }
        ast::StringGroup::Joined { values } => {
            let node = create_node(vm, "JoinedStr");
            let py_values = vm.ctx.new_list(
                values
                    .iter()
                    .map(|value| string_to_ast(vm, value))
                    .collect(),
            );
            vm.ctx.set_attr(&node, "values", py_values);
            node
        }
    }
}

fn ast_parse(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(source, Some(vm.ctx.str_type()))]);

    let source_string = objstr::get_value(source);
    let internal_ast = parser::parse_program(&source_string)
        .map_err(|err| vm.new_value_error(format!("{}", err)))?;
    // source.clone();
    let ast_node = program_to_ast(&vm, &internal_ast);
    Ok(ast_node)
}

pub fn make_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, "ast", {
        "parse" => ctx.new_rustfunc(ast_parse),
        "Module" => py_class!(ctx, "_ast.Module", ctx.object(), {}),
        "FunctionDef" => py_class!(ctx, "_ast.FunctionDef", ctx.object(), {}),
        "Call" => py_class!(ctx, "_ast.Call", ctx.object(), {}),
        "AST" => py_class!(ctx, "_ast.AST", ctx.object(), {}),
    })
}
