use super::lexer;
use super::parser;

#[derive(Debug)]
pub enum RuntimeValType {
    Null,
    NumericInteger(i64),
    NumericFloat(f64),
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

#[derive(Debug)]
pub struct RuntimeVal {
    runtime_val_type : RuntimeValType,
}

pub fn eval(node: &parser::Node) -> RuntimeVal {
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
                }
                _=>panic!()

            }
        },
        parser::NodeType::BinaryExpr(_) => {
            eval_binary_expr(node)
        }
        _ => {
            panic!()
        }
    }
}

// pub fn eval_numeric_binary_expr (node: &parser::Node) -> RuntimeVal {
//     let operator = node.node_type.extract_binexp_operator().unwrap();

//     match operator{
//         "+" => {
//             let left = &node.body[0];
//             let right = &node.body[1];

//             let left_type = &left.value.as_ref().unwrap().token_type;
//             let right_type = &right.value.as_ref().unwrap().token_type;

//             if !matches!(left_type, right_type){
//                 panic!("mismatched token types")
//             } else {
//                 match &left.value.as_ref().unwrap().token_type {
//                     lexer::TokenType::Integer(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericInteger(*left_value + *right_value)
//                         }
//                     },
//                     lexer::TokenType::Float(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericFloat(*left_value + *right_value)
//                         }
//                     }
//                     _ => panic!()
//                 }
//             }
//         },
//         "-" => {
//             let left = &node.body[0];
//             let right = &node.body[1];

//             let left_type = &left.value.as_ref().unwrap().token_type;
//             let right_type = &right.value.as_ref().unwrap().token_type;

//             if !matches!(left_type, right_type){
//                 panic!("mismatched token types")
//             } else {
//                 match &left.value.as_ref().unwrap().token_type {
//                     lexer::TokenType::Integer(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericInteger(*left_value - *right_value)
//                         }
//                     },
//                     lexer::TokenType::Float(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericFloat(*left_value - *right_value)
//                         }
//                     }
//                     _ => panic!()
//                 }
//             }
//         },
//         "*" => {
//             let left = &node.body[0];
//             let right = &node.body[1];

//             let left_type = &left.value.as_ref().unwrap().token_type;
//             let right_type = &right.value.as_ref().unwrap().token_type;

//             if !matches!(left_type, right_type){
//                 panic!("mismatched token types")
//             } else {
//                 match &left.value.as_ref().unwrap().token_type {
//                     lexer::TokenType::Integer(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericInteger(*left_value * *right_value)
//                         }
//                     },
//                     lexer::TokenType::Float(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericFloat(*left_value * *right_value)
//                         }
//                     }
//                     _ => panic!()
//                 }
//             }
//         },
//         "/" => {
//             let left = &node.body[0];
//             let right = &node.body[1];

//             let left_type = &left.value.as_ref().unwrap().token_type;
//             let right_type = &right.value.as_ref().unwrap().token_type;

//             if !matches!(left_type, right_type){
//                 panic!("mismatched token types")
//             } else {
//                 match &left.value.as_ref().unwrap().token_type {
//                     lexer::TokenType::Integer(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_int_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericInteger(*left_value / *right_value)
//                         }
//                     },
//                     lexer::TokenType::Float(_) => {
//                         let left_value = left.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         let right_value = right.value.as_ref().unwrap().token_type.extract_float_value().unwrap();
//                         RuntimeVal {
//                             runtime_val_type: RuntimeValType::NumericFloat(*left_value / *right_value)
//                         }
//                     }
//                     _ => panic!()
//                 }
//             }
//         }
//         _ => {
//             panic!()
//         }
//     }
// }

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
                    }
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
        _ => {
            panic!()
        }
    }
}

fn eval_binary_expr(node: &parser::Node) -> RuntimeVal {
    let left = eval(&node.body[0]);
    let right = eval(&node.body[1]);

    if matches!(left.runtime_val_type, RuntimeValType::NumericInteger(_)){
        eval_numeric_binary_expr(&left, &right, node.node_type.extract_binexp_operator().unwrap())
    } else {
        panic!()
    }
}

pub fn eval_program(program: &parser::Node) -> RuntimeVal{
    let mut last_eval: RuntimeVal = RuntimeVal { runtime_val_type: RuntimeValType::Null };
    for node in &program.body {
        last_eval = eval(node);
    }
    last_eval
}