use super::eval;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String, // Should be identifier
    pub value: eval::RuntimeVal   
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub variables: Vec<Variable>
}

pub fn resolve_env(env: Rc<RefCell<Environment>>, name: &str) -> Rc<RefCell<Environment>>{
    let env_rc = env.borrow();
    for variable in env_rc.variables.clone() {
        if variable.name == name {
            return env.clone()
        }
    }

    if let Some(ref parent) = env.borrow().parent {
        return resolve_env(parent.clone(), name)
    } 

    panic!("Cannot resolve environment")
}

pub fn declare_variable (env: Rc<RefCell<Environment>>, identifier: &str, value: &eval::RuntimeVal) -> eval::RuntimeVal{
    let mut env_rc = env.borrow_mut();
    for variable in env_rc.variables.clone() {
        if variable.name == identifier {
            panic!("Variable already defined: {:?}", variable.name)
        } else {
            env_rc.variables.push(Variable { name: identifier.to_string(), value: value.clone() });
            return value.clone()
        }
    }
    env_rc.variables.push(Variable { name: identifier.to_string(), value: value.clone() });
    value.clone()
}

pub fn assign_variable (env: Rc<RefCell<Environment>>, identifier: &str, value: &eval::RuntimeVal) -> eval::RuntimeVal{
    // identifier.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string()
    let env_r = resolve_env(env, identifier);
    let mut env_rc = env_r.borrow_mut();
    for (count, variable) in env_rc.variables.clone().iter().enumerate() {
        if identifier == variable.name {
            env_rc.variables[count].value = value.clone();
            return eval::RuntimeVal { runtime_val_type: eval::RuntimeValType::Null }
        }
    }
    // println!("{:?}",env);
    panic!("Cannot assign uninitialised variable - {:?}", identifier)
}

pub fn lookup_variable (env: Rc<RefCell<Environment>>, identifier: &str) -> eval::RuntimeVal {
    let env_r = resolve_env(env, identifier);
    let mut env_rc = env_r.borrow_mut();
    for variable in env_rc.variables.clone() {
        if identifier == variable.name {
            return variable.value
        }
    }
    panic!("Variable does not exist - {:?}", identifier)
}


