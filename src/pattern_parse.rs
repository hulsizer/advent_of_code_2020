
pub enum ParseValue {
    // An `enum` may either be `unit-like`,
    Char(char),
    String(String),
    USize(usize),
    F64(f64),
    I64(i64),
}

impl ParseValue {
    
    pub fn char(&self) -> Option<char> {
        match self {
            ParseValue::Char(value) => Some(*value),
            _ => None
        }
    }

    pub fn string(&self) -> Option<&String> {
        match self {
            ParseValue::String(value) => Some(value),
            _ => None
        }
    }

    pub fn uSize(&self) -> Option<usize> {
        match self {
            ParseValue::USize(value) => Some(*value),
            _ => None
        }
    }

    pub fn f64(&self) -> Option<f64> {
        match self {
            ParseValue::F64(value) => Some(*value),
            _ => None
        }
    }

    pub fn i64(&self) -> Option<i64> {
        match self {
            ParseValue::I64(value) => Some(*value),
            _ => None
        }
    }
}
pub trait ParsePattern<'a> {
    fn parse(&'a self, _: &str) -> Vec<ParseValue>;
}

impl<'a> ParsePattern<'a> for &'a str {
    fn parse(&'a self, pattern: &str) -> Vec<ParseValue> { 

        let mut parseValues: Vec<ParseValue> = vec![];

        let mut regex_str = pattern.replace("%s", "(.*)");
        regex_str = regex_str.replace("%c", "(.*)");
        regex_str = regex_str.replace("%i", "(.*)");
        regex_str = regex_str.replace("%f", "(.*)");
        regex_str = regex_str.replace("%u", "(.*)");

        let re = regex::Regex::new(&regex_str).unwrap();
        let keys = re.captures(pattern).unwrap();
        let values = re.captures(self).unwrap();
        
        for index in 0..values.len() {
            let key = keys.get(index).unwrap().as_str();
            match key {
                "%s" => parseValues.push(ParseValue::String(values.get(index).unwrap().as_str().to_string())),
                "%c" => parseValues.push(ParseValue::Char(values.get(index).unwrap().as_str().chars().next().unwrap())),
                "%u" => parseValues.push(ParseValue::USize(values.get(index).unwrap().as_str().parse::<usize>().unwrap())),
                "%i" => parseValues.push(ParseValue::I64(values.get(index).unwrap().as_str().parse::<i64>().unwrap())),
                "%f" => parseValues.push(ParseValue::F64(values.get(index).unwrap().as_str().parse::<f64>().unwrap())),
                _ => ()
            }
        }
        return parseValues
    }
}
