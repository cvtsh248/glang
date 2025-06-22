use super::lexer;
use super::parser;
use super::eval;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String, // Should be identifier
    pub value: eval::RuntimeVal   
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub variables: Vec<Variable>
}
impl Environment {

    pub fn resolve_env(&self, name: &str) -> Environment{

        for variable in self.variables.clone() {
            if variable.name == name {
                return self.clone()
            }
        }

        if !self.parent.is_none() {
            return self.parent.clone().as_mut().unwrap().resolve_env(name)
        } 


        panic!("Cannot resolve environment")
    }

    pub fn declare_variable(&mut self, identifier: &str, value: &eval::RuntimeVal) -> eval::RuntimeVal{
        for variable in self.variables.clone() {
            if variable.name == identifier {
                panic!("Variable already defined: {:?}", variable.name)
            } else {
                self.variables.push(Variable { name: identifier.to_string(), value: value.clone() });
            }
        }
        *value
    }

    pub fn assign_variable(&mut self){
        // identifier.value.as_ref().unwrap().token_type.extract_str_value().unwrap().to_string()

    }

    pub fn lookup_variable(&self, identifier: &str) -> eval::RuntimeVal {
        let env = self.resolve_env(identifier);
        for variable in env.variables.clone() {
            if identifier == variable.name {
                return variable.value
            }
        }
        panic!("Variable does not exist - {:?}", identifier)
    }


}