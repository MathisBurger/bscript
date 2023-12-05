use std::fmt::format;

pub(crate) struct StdLib {
    func: Vec<String>
}

impl StdLib {

    pub fn try_get_value(func: String, parameters: Vec<String>) -> Option<String> {
        let line = StdLib::check_lib_hit(func);
        if line == None {
            return None;
        }
        let mut raw_line: String = line.unwrap();
        for x in 0..parameters.len() {
            raw_line = raw_line.replace(
                format!("${}", x).as_str(),
                parameters.get(x).unwrap().as_str()
            );
        }
        Some(raw_line.to_string())
    }

    fn check_lib_hit(func: String) -> Option<String> {
        match func.as_str() {
            "print" => Some("println!(\"{}\", $0);".to_string()),
            _ => None
        }
    }
}