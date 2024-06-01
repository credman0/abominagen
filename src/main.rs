mod languages;
mod codeabstractions;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    languages:Vec<languages::Language>
}

fn main() {
    let opt = Opt::from_args();
    let base_actions = codeabstractions::self_creation_actions();
    let mut wrapped = base_actions;
    for lang in opt.languages.clone().into_iter().rev() {
        wrapped = lang.wrap_actions(wrapped);
    }
    codeabstractions::execute_actions(wrapped);
    let mut full_description = String::new();
    let len = opt.languages.len();
    for (i, lang) in opt.languages.into_iter().enumerate() {
        let name = lang.name();
        if ["a", "e", "i", "o", "u"].contains(&name.chars().next().unwrap().to_string().as_str()) {
            full_description.push_str("an ");
        } else {
            full_description.push_str("a ");
        }
        full_description.push_str(&name);
        full_description.push_str(" ");
        full_description.push_str(&lang.get_descriptor());
        if i != len - 1 {
            full_description.push_str(" that generates ");
        }
    }
    // capitalize the first "a"
    full_description.replace_range(0..1, "A");
    println!("{}", full_description);
}
