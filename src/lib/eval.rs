use std::cell::Ref;
use std::env;

use super::lexer;
use super::parser;
use super::environment;
use std::rc::Rc;
use std::cell::RefCell;

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

    pub fn extract_bool_value(&self) -> Option<&bool> {
        if let RuntimeValType::Boolean(boolean) = self {
            Some(boolean)
        } else {
            None
        }
    }

    pub fn extract_string_value(&self) -> Option<&String> {
        if let RuntimeValType::StringLiteral(string_) = self {
            Some(string_)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone)]
pub struct RuntimeVal {
    pub runtime_val_type : RuntimeValType,
}

pub fn eval(node: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal {
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
        parser::NodeType::UnaryExpr(_) => {
            eval_unary_expr(node, env)
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
                runtime_val_type: RuntimeValType::Boolean(*token_value)
            }
        },
        parser::NodeType::EOL => {
            panic!("This is impossible to reach")
        },
        parser::NodeType::Scope => {
            eval(node, env)
        }
        _ => {
            panic!()
        }
    }
}

pub fn eval_identifier(identifier: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal{
    let identifier_string = identifier.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    environment::lookup_variable(env, &identifier_string)
}

pub fn eval_assignment(node: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal{
    let identifier_string = node.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    let eval_rhs = eval(&node.body[0], env.clone());
    environment::assign_variable(env, &identifier_string, &eval_rhs)
}

pub fn eval_declaration(node: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal{
    let identifier_string = node.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string();
    let eval_rhs = eval(&node.body[0], env.clone());
    environment::declare_variable(env, &identifier_string, &eval_rhs)
}

fn eval_numeric_unary_expr(right: &RuntimeVal, operator: &str) -> RuntimeVal {
    match operator {
        "!" => {
            let right_type = &right.runtime_val_type;
            if matches!(right_type, RuntimeValType::NumericInteger(_)){
                let right_value = right_type.extract_int_value().unwrap();
                return RuntimeVal {
                    runtime_val_type: RuntimeValType::NumericInteger(!*right_value)
                }
            }
            panic!("Incorrect type for ! operator")

        },
        _ => {
            panic!()
        }
    }
}

fn eval_boolean_unary_expr(right: &RuntimeVal, operator: &str) -> RuntimeVal {
    match operator {
        "!" => {
            let right_value = right.runtime_val_type.extract_bool_value().unwrap();
            RuntimeVal {
                runtime_val_type: RuntimeValType::Boolean(!*right_value)
            }
        },
        _ => {
            panic!()
        }
    }
}

fn eval_unary_expr(node: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal {
    let right = eval(&node.body[0], env.clone());
    if matches!(right.runtime_val_type, RuntimeValType::NumericInteger(_)){
        eval_numeric_unary_expr(&right, node.node_type.extract_unexp_operator().unwrap())
    } else if matches!(right.runtime_val_type, RuntimeValType::Boolean(_)) {
        eval_boolean_unary_expr(&right, node.node_type.extract_unexp_operator().unwrap())
    } else {
        panic!("Incorrect type")
    }
    // panic!("{:?}",node.node_type.extract_unexp_operator().unwrap())
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
        "%" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericInteger(*left_value % *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::NumericFloat(*left_value % *right_value)
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
        "!=" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value != *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value != *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        ">" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value > *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value > *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "<" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value < *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value < *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        ">=" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value >= *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value >= *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "<=" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::NumericInteger(_) => {
                        let left_value = left_type.extract_int_value().unwrap();
                        let right_value = right_type.extract_int_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value <= *right_value)
                        }
                    },
                    RuntimeValType::NumericFloat(_) => {
                        let left_value = left_type.extract_float_value().unwrap();
                        let right_value = right_type.extract_float_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value <= *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        _ => {
            panic!("Invalid operator for numeric type")
        }
    }
}

fn eval_bool_binary_expr(left: &RuntimeVal, right: &RuntimeVal, operator: &str) -> RuntimeVal{
    match operator {
        "==" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::Boolean(_) => {
                        let left_value = left_type.extract_bool_value().unwrap();
                        let right_value = right_type.extract_bool_value().unwrap();
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
        "!=" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::Boolean(_) => {
                        let left_value = left_type.extract_bool_value().unwrap();
                        let right_value = right_type.extract_bool_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value != *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "&&" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::Boolean(_) => {
                        let left_value = left_type.extract_bool_value().unwrap();
                        let right_value = right_type.extract_bool_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value && *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        "||" => {
            let left_type = &left.runtime_val_type;
            let right_type = &right.runtime_val_type;

            if matches!(left_type, right_type){
                match left_type {
                    RuntimeValType::Boolean(_) => {
                        let left_value = left_type.extract_bool_value().unwrap();
                        let right_value = right_type.extract_bool_value().unwrap();
                        RuntimeVal {
                            runtime_val_type: RuntimeValType::Boolean(*left_value || *right_value)
                        }
                    }
                    _ => panic!()
                }

            } else {
                panic!("Mismatched types")
            }
        },
        _ => {
            panic!("Invalid operator for boolean type")
        }
    }
}

fn eval_binary_expr(node: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal {
    let left = eval(&node.body[0], env.clone());
    let right = eval(&node.body[1], env.clone());

    if matches!(left.runtime_val_type, RuntimeValType::NumericInteger(_)) && matches!(right.runtime_val_type, RuntimeValType::NumericInteger(_)){
        eval_numeric_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else if matches!(left.runtime_val_type, RuntimeValType::NumericFloat(_)) && matches!(right.runtime_val_type, RuntimeValType::NumericFloat(_)) {
        eval_numeric_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else if matches!(left.runtime_val_type, RuntimeValType::Boolean(_)) && matches!(right.runtime_val_type, RuntimeValType::Boolean(_)) {
        eval_bool_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else {
        panic!()
    }
}

pub fn eval_program(program: &parser::Node, env: Rc<RefCell<environment::Environment>>) -> RuntimeVal{
    let mut last_eval: RuntimeVal = RuntimeVal { runtime_val_type: RuntimeValType::Null };
    let mut loop_flag = false;
    let mut if_check_fail_flag = false;
    let mut loop_condition: Option<&parser::Node> = None;
    let mut loop_node: Option<&parser::Node> = None;
    let mut program_counter: usize = 0;
    while program_counter < program.body.len() {
        let node = &program.body[program_counter];
        
        if matches!(node.node_type, parser::NodeType::Loop) || loop_flag == true {

            if loop_flag == false {
                loop_node = Some(&node);
                loop_condition = Some(&node.body[0]);
            }

            let condition_eval = eval(loop_condition.unwrap(),env.clone());

            let mut new_env = Rc::new(RefCell::new(environment::Environment {parent: Some(env.clone()), variables: vec![]})); 
            
            // println!("{:?}", loop_flag);

            eval_program(&loop_node.unwrap(),new_env);

            loop_flag = true;
            if *condition_eval.runtime_val_type.extract_bool_value().unwrap() == false {
                program_counter += 1;
                loop_flag = false;
            }

        } else if matches!(node.node_type, parser::NodeType::If) {
            let if_condition = &node.body[0];
            let if_condition_eval = eval(if_condition, env.clone());

            if *if_condition_eval.runtime_val_type.extract_bool_value().unwrap() == true {
                let mut new_env = Rc::new(RefCell::new(environment::Environment {parent: Some(env.clone()), variables: vec![]})); 
                eval_program(node,new_env);
                program_counter += 1;
            } else {
                if_check_fail_flag = true;
                program_counter += 1;
            }

        } else if matches!(node.node_type, parser::NodeType::ElseIf) {
            let if_condition = &node.body[0];
            let if_condition_eval = eval(if_condition, env.clone());

            if *if_condition_eval.runtime_val_type.extract_bool_value().unwrap() == true && if_check_fail_flag == true {
                let mut new_env = Rc::new(RefCell::new(environment::Environment {parent: Some(env.clone()), variables: vec![]})); 
                eval_program(node,new_env);
                if_check_fail_flag = false;
                program_counter += 1;
            } else {
                if_check_fail_flag = true;
                program_counter += 1;
            }

        } else if matches!(node.node_type, parser::NodeType::Else){
            if if_check_fail_flag == true {
                let mut new_env = Rc::new(RefCell::new(environment::Environment {parent: Some(env.clone()), variables: vec![]})); 
                eval_program(node,new_env);
                program_counter += 1;
                if_check_fail_flag = false;
            } else {
                program_counter += 1;
            }
        } else if matches!(node.node_type, parser::NodeType::Scope){
            let mut new_env = Rc::new(RefCell::new(environment::Environment {parent: Some(env.clone()), variables: vec![]})); 
            
            eval_program(node,new_env);
            program_counter += 1;
            // println!("{:?}", new_env);
            
        } else if matches!(node.node_type, parser::NodeType::Print) {
            let val = eval(&node.body[0], env.clone());
            // println!("{:?}",&node.body[0]);
            match val.runtime_val_type {
                RuntimeValType::NumericInteger(_) => {
                    println!("{:?}",val.runtime_val_type.extract_int_value().unwrap());
                },
                RuntimeValType::NumericFloat(_) => {
                    println!("{:?}",val.runtime_val_type.extract_float_value().unwrap());
                },
                RuntimeValType::Boolean(_) => {
                    println!("{:?}",val.runtime_val_type.extract_bool_value().unwrap());
                },
                RuntimeValType::StringLiteral(_) => {
                    println!("{:?}",val.runtime_val_type.extract_string_value().unwrap());
                },
                _=> panic!("Invalid type to print")
            }
            program_counter += 1;
            // println!(val.va)
        } else if matches!(node.node_type, parser::NodeType::EOL){
            program_counter += 1;
            continue
        } else {
            last_eval = eval(&node, env.clone());
            program_counter += 1;
        }
    }
    
    last_eval
}