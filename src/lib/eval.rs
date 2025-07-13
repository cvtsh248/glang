use std::env;

use super::lexer;
use super::parser;
use super::environment;

#[derive(Debug, Clone)]
pub enum RuntimeValType {
    Null,
    NumericInteger(i64),
    NumericFloat(f64),
    StringLiteral(String),
    Boolean(bool),
    Runtime
}
impl RuntimeValType {
    pub fn extract_int_value(&self) -> Option<&i64> {
        if let RuntimeValType::NumericInteger(int) = self {
            Some(int)
        } else {
            None
        }
    }

    pub fn extract_float_value(&self) -> Option<&f64> {
        if let RuntimeValType::NumericFloat(float) = self {
            Some(float)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone)]
pub struct RuntimeVal {
    pub runtime_val_type : RuntimeValType,
}

pub fn eval(node: &parser::Node, env: &mut environment::Environment) -> RuntimeVal {
    match node.node_type {
        parser::NodeType::NumericLiteral => {
            let token = &node.value.as_ref().unwrap();

            match token.token_type {
                lexer::TokenType::Float(_) => {
                    let token_value = token.token_type.extract_float_value().unwrap();
                    RuntimeVal {
                        runtime_val_type: RuntimeValType::NumericFloat(*token_value),
                    }
                },
                lexer::TokenType::Integer(_) => {
                    let token_value = token.token_type.extract_int_value().unwrap();
                    RuntimeVal {
                        runtime_val_type: RuntimeValType::NumericInteger(*token_value),
                    }
                },
                _=>panic!()

            }
        },
        parser::NodeType::BinaryExpr(_) => {
            eval_binary_expr(node, env)
        },
        parser::NodeType::Identifier => {
            eval_identifier(node, env)
        },
        parser::NodeType::Assignment => {
            let mut rhs: parser::Node;
            eval_assignment(node, env)
        },
        parser::NodeType::Declaration => {
            eval_declaration(node, env)
        },
        parser::NodeType::StringLiteral => {
            let token = &node.value.as_ref().unwrap();
            let token_value = token.token_type.extract_str_value().unwrap();
            RuntimeVal {
                runtime_val_type: RuntimeValType::StringLiteral(token_value.to_string())
            }
        },
        parser::NodeType::Boolean => {
            let token = &node.value.as_ref().unwrap();
            let token_value = token.token_type.extract_bool_value().unwrap();
            RuntimeVal {
                runtime_val_type: RuntimeValType::Boolean(token_value)
            }
        },
        parser::NodeType::EOL => {
            panic!("This is impossible to reach")
        }
        _ => {
            panic!()
        }
    }
}

pub fn eval_identifier(identifier: &parser::Node, env: &mut environment::Environment) -> RuntimeVal{
    let identifier_string = identifier.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    env.lookup_variable(&identifier_string)
}

pub fn eval_assignment(node: &parser::Node, env: &mut environment::Environment) -> RuntimeVal{
    let identifier_string = node.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    let eval_rhs = eval(&node.body[0], env);
    env.assign_variable(&identifier_string, &eval_rhs)
}

pub fn eval_declaration(node: &parser::Node, env: &mut environment::Environment) -> RuntimeVal{
    let identifier_string = node.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    let eval_rhs = eval(&node.body[0], env);
    env.declare_variable(&identifier_string, &eval_rhs)
}


pub fn eval_numeric_binary_expr(left: &RuntimeVal, right: &RuntimeVal, operator: &str) -> RuntimeVal{
    match operator {
        "+" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericInteger(*left_value + *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericFloat(*left_value + *right_value)
                        }
                    },

                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "-" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericInteger(*left_value - *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericFloat(*left_value - *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "*" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericInteger(*left_value * *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericFloat(*left_value * *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "/" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericInteger(*left_value / *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericFloat(*left_value / *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "==" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value==*right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value==*right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        _ => {
            panic!()
        }
    }
}

fn eval_binary_expr(node: &parser::Node, env: &mut environment::Environment) -> RuntimeVal {
    let left = eval(&node.body[0], env);
    let right = eval(&node.body[1], env);

    if matches!(left.runtime_val_type, RuntimeValType::NumericInteger(_)) && matches!(right.runtime_val_type, RuntimeValType::NumericInteger(_)){
        eval_numeric_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else if matches!(left.runtime_val_type, RuntimeValType::NumericFloat(_)) && matches!(right.runtime_val_type, RuntimeValType::NumericFloat(_)) {
        eval_numeric_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else {
        panic!()
    }
}

pub fn eval_program(program: &parser::Node, env: &mut environment::Environment) -> RuntimeVal{
    let mut last_eval: RuntimeVal = RuntimeVal { runtime_val_type: RuntimeValType::Null };
    for node in &program.body {
        if matches!(node.node_type, parser::NodeType::EOL){

        } else {
            last_eval = eval(node, env);
        }
    }
    last_eval
}