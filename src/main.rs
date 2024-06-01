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
    for lang in opt.languages.into_iter().rev() {
        wrapped = lang.wrap_actions(wrapped);
    }
    codeabstractions::execute_actions(wrapped);
}
