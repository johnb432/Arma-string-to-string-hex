use arma_rs::{arma, Extension};

// To build, cd <directory>, then "cargo build"

#[arma]
fn init() -> Extension {
    Extension::build()
        .version("1.0.0".to_string())
        .command("get_hex", get_hex_from_string)
        .finish()
}
// The command is what has to be called in Arma
// "arma_string_to_string_hex" callExtension ["get_hex", ["487784725"]] returns ["1D130115",0,301]

fn get_hex_from_string(string_int: String) -> String {
    format!("{:X}", u32::from_str_radix(&string_int, 10).unwrap())
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn get_hex_from_string() {
        let extension = init().testing();
        let (result, _) = unsafe { extension.call("get_hex", Some(vec!["487784725".to_string()])) };
        assert_eq!(result, "1D130115");
    }
}

// Only required for cargo, don't include in your library
//fn main() {}
