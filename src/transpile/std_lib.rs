
/// Implements the standard library
pub(crate) struct StdLib;

impl StdLib {

    /// Tries to get the full string with parameters filled in
    pub fn try_get_value(func: String, parameters: Vec<String>) -> Option<String> {
        let line = StdLib::check_lib_hit(func);
        if line == None {
            return None;
        }
        let mut raw_line: String = line.unwrap();
        for x in 0..parameters.len() {
            let mut parameter = match parameters.get(x).unwrap().replace(" ", "").as_str() {
                "int" => "isize".to_string(),
                "str" => "String".to_string(),
                _ => parameters.get(x).unwrap().to_string()
            };
            if parameter.contains("\"") {
                parameter = format!("{}.to_string()", parameter);
            }
            raw_line = raw_line.replace(
                format!("${}", x).as_str(),
                parameter.as_str()
            );
        }
        Some(raw_line.to_string())
    }

    /// Tries to get the full string without parameters filled in.
    /// The parameters are presented as placeholders
    fn check_lib_hit(func: String) -> Option<String> {
        match func.as_str() {
            "print" => Some("println!(\"{}\", $0);".to_string()),
            "init" => Some("let mut $0:$1=$2;".to_string()),
            "add" => Some("$0=$1+$2;".to_string()),
            "store" => Some("$0=$1;".to_string()),
            "sub" => Some("$0=$1-$2;".to_string()),
            "mul" => Some("$0=$1*$2;".to_string()),
            "div" => Some("$0=$1/$2;".to_string()),
            "pow" => Some("$0=isize::pow($1,$2);".to_string()),
            "cmp" => Some("$0=($1==$2) as isize;".to_string()),
            _ => None
        }
    }
}