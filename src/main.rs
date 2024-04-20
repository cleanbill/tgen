use std::{ env, fs };

fn setup_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let qty_args_ok = args.len() == 3;

    if !qty_args_ok {
        println!("Usage: {} <template file> <translation dir>", args[0]);
        println!("Using default values");
    } else {
        print!("Using {} ", args[1]);
    }

    if qty_args_ok {
        return (args[1].to_string(), args[2].to_string());
    }

    return ("./template.html".to_string(), "./".to_string());
}

fn populate(json_translation_filename: String, template_filename: &str) {
    let mut template = fs
        ::read_to_string(template_filename)
        .expect("Should have been able to read template file");
    let mut output_filename = String::new();
    let len = json_translation_filename.len() - 5;
    output_filename.push_str(&json_translation_filename[0..len]);
    output_filename.push_str(".html");

    println!(
        "Populating {} with {} to produce {} ",
        json_translation_filename,
        template_filename,
        output_filename
    );

    let json_contents = fs
        ::read_to_string(json_translation_filename)
        .expect("Should have been able to read the json file");
    let json: serde_json::Value = serde_json
        ::from_str(json_contents.as_str())
        .expect("JSON was not well-formatted");
    for key in json.as_object().unwrap().keys() {
        let mut token = String::new();
        token.push_str("${");
        token.push_str(key);
        token.push_str("}");
        let value = json.get(key).unwrap().to_string().replace("\"", "");
        template = template.replace(token.as_str(), value.as_str());
    }
    fs::write(output_filename, template).expect("Unable to write file");
}

fn main() {
    let args: (String, String) = setup_args();
    println!("The template file is {} and the json translation directory is {}", args.0, args.1);

    let paths = fs::read_dir(args.1).unwrap();
    for path in paths {
        let filename = path.unwrap();
        let mut path_name = String::new();
        path_name.push_str(&filename.file_name().into_string().unwrap());
        if path_name.contains("json") {
            populate(path_name, &args.0);
        }
    }
}
