use crate::transpile::std_lib::StdLib;

/// The builder that builds the rust script from the bs script
pub(crate) struct Builder {

    /// All lines of code in the .bs file
    pub(crate) lines: Vec<String>,

    /// All lines of code in the main function of the .bs file
    pub(crate) main_funcs: Vec<String>
}

impl Builder {

    /// Inits a new builder
    pub(crate) fn new(lines: Vec<String>) -> Self {
        Builder {
            lines,
            main_funcs: vec![]
        }
    }

    /// Builds the rust code from bscript
    pub(crate) fn build(&mut self) -> String {
        for x in 0..self.lines.len() {
           // Only std lib supported currently
           // logic and functions will be added soon
           self.add_to_main(self.lines.get(x).unwrap().clone());
        }
        Builder::build_function("main".to_string(), self.main_funcs.clone())
    }

    /// Builds a specific function
    fn build_function(name: String, methods: Vec<String>) -> String {
        let mut body = String::new();
        for x in methods {
            body = format!("{}{}", body, x)
        }
        return format!("pub fn {}(){{ {} }}", name, body);
    }

    /// Adds new lines to main function
    fn add_to_main(&mut self, line: String) {
        let (func, params) = Builder::seperate_params(line);
        let result = StdLib::try_get_value(func, params);
        if result.is_some() {
            self.main_funcs.push(result.unwrap());
        }
    }

    /// Seperates params from the method calls of the std lib
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