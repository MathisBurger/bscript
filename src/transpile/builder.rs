use std::fmt::format;
use std::result;
use crate::transpile::hasher;
use crate::transpile::std_lib::StdLib;

pub(crate) struct Builder {
    pub(crate) lines: Vec<String>,
    pub(crate) main_funcs: Vec<String>
}

impl Builder {

    pub(crate) fn new(lines: Vec<String>) -> Self {
        Builder {
            lines,
            main_funcs: vec![]
        }
    }

    pub(crate) fn build(&mut self) -> String {
        for x in 0..self.lines.len() {
           // Only std lib supported currently
           // logic and functions will be added soon
           self.add_to_main(self.lines.get(x).unwrap().clone());
        }
        Builder::build_function("main".to_string(), self.main_funcs.clone())
    }

    fn build_function(name: String, methods: Vec<String>) -> String {
        let mut body = String::new();
        for x in methods {
            body = format!("{}{}", body, x)
        }
        return format!("pub fn {}(){{ {} }}", name, body);
    }

    fn add_to_main(&mut self, line: String) {
        let (func, params) = Builder::seperate_params(line);
        let result = StdLib::try_get_value(func, params);
        if result.is_some() {
            self.main_funcs.push(result.unwrap());
        }
    }

    fn seperate_params(line: String) -> (String, Vec<String>) {
        let spl: Vec<String> = line.split("<<")
            .map(str::to_string)
            .collect();
        if spl.len() > 1 {
            let mut params: Vec<String> = vec![];
            for x in 1..spl.len() {
                params.push(spl.get(x).unwrap().clone());
            }
            return (spl.get(0).unwrap().clone().replace(" ", ""), params);
        }
        return (spl.get(0).unwrap().clone(), vec![]);
    }
}