use std::{ env, fs };

use serde_json::Value;

fn setup_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let qty_args_ok = args.len() == 3;

    if !qty_args_ok {
        println!("Usage: {} <template file> <translation config>", args[0]);
        println!("Using default values");
    } else {
        print!("Using {} ", args[1]);
    }
    println!("");

    if qty_args_ok {
        return (args[1].to_string(), args[2].to_string());
    }

    return ("./template.html".to_string(), "./locale-config.json".to_string());
}

fn populate(index: i32, locale: String, config: &Value, template_filename: &str) {
    let mut template = fs
        ::read_to_string(template_filename)
        .expect("Should have been able to read template file");
    let mut output_filename = String::new();
    output_filename.push_str(&locale);
    output_filename.push_str(".html");

    println!(
        "{}. Populating {} with {} to produce {} ",
        index,
        locale,
        template_filename,
        output_filename
    );

    let labels = config.get("labels").unwrap();
    let links = config.get("footerLinks").unwrap();

    for key in labels.as_object().unwrap().keys() {
        let mut token = String::new();
        token.push_str("${");
        token.push_str(key);
        token.push_str("}");
        let value = labels.get(key.to_string()).unwrap().to_string().replace("\"", "");
        template = template.replace(token.as_str(), value.as_str());
    }
    let mut links_output = String::new();
    for link in links.as_array().unwrap() {
        links_output.push_str("<a href=\"");
        links_output.push_str(link.get("url").unwrap().as_str().unwrap());
        links_output.push_str("\">");
        links_output.push_str(link.get("text").unwrap().as_str().unwrap());
        links_output.push_str("</a> ");
    }
    template = template.replace("${footer}", &links_output);
    fs::write(output_filename, template).expect("Unable to write file");
}

fn main() {
    let args: (String, String) = setup_args();
    println!("The template file is {} and the json translation directory is {}", args.0, args.1);
    println!("");
    let translations = fs
        ::read_to_string(args.1)
        .expect("Should have been able to read translation file");
    let json: serde_json::Value = serde_json
        ::from_str(translations.as_str())
        .expect("translation JSON was not well-formatted");
    let mut index = 1;
    for key in json.as_object().unwrap().keys() {
        let config: &Value = json.get(key).unwrap();
        populate(index, key.to_string(), config, &args.0);
        index = index + 1;
    }
}
