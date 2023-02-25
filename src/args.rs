use std::env;

enum ConvertorType {
    SnakeCase,
    CamelCase,
    KebabCase,
}

pub struct Args {
    convertor_type: ConvertorType,
    variable_name: String,
}

impl Args {
    pub fn new() -> Result<Args, String> {
        let raw_args: Vec<String> = env::args().collect();
        let mut i: i32 = 0;
        let mut convertor_type: ConvertorType = ConvertorType::SnakeCase;
        let mut variable_name: String = "".to_owned();

        for raw_arg in raw_args {
            if i == 0 {
                match Args::get_convertor_type(&raw_arg) {
                    Ok(result) => convertor_type = result,
                    Err(error) => return Err(error),
                }
            } else if i == 1 {
                variable_name = raw_arg.to_owned();
            } else {
                return Err("Too many args".to_owned());
            }

            i = i + 1;
        }

        return Ok(Args {
            convertor_type,
            variable_name,
        });
    }

    pub fn to_string(self) -> String {
        return format!(
            "Args{{convertorType={}, variableName={}}}",
            Args::convertor_type_to_string(self.convertor_type),
            self.variable_name
        );
    }

    fn get_convertor_type(raw_arg: &String) -> Result<ConvertorType, String> {
        if raw_arg.eq("snake-case") {
            return Ok(ConvertorType::CamelCase);
        }

        if raw_arg.eq("camel-case") {
            return Ok(ConvertorType::CamelCase);
        }

        if raw_arg.eq("kebab-case") {
            return Ok(ConvertorType::KebabCase);
        }

        return Err(format!("Not supported raw convertor type [{}]", raw_arg));
    }

    fn convertor_type_to_string(convertor_type: ConvertorType) -> String {
        match convertor_type {
            ConvertorType::SnakeCase => return "SnakeCase".to_owned(),
            ConvertorType::CamelCase => return "CamelCase".to_owned(),
            ConvertorType::KebabCase => return "KebabCase".to_owned(),
        }
    }
}
