// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.


mod sausage_factory {
    use std::env;
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        match env::var("Ginger") {
            Ok(recipe) => recipe,
            Err(_) => String::from("Ginger"), // 默认值
        }
    }

    // Export a public function
    pub fn make_sausage() {
        let secret_recipe = get_secret_recipe();
        println!("Secret recipe: {}", secret_recipe);
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
