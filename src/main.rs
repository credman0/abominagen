mod languages;
mod codeabstractions;

fn main() {
    let self_actions = codeabstractions::self_creation_actions();
    let wrapped = languages::rust::wrap_actions(self_actions);
    println!("{:?}", wrapped);
    codeabstractions::execute_actions(wrapped);
}
