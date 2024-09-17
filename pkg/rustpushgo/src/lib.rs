#[uniffi::export]
pub fn add(left: u32, right: u32) -> u32 {
    println!("Adding {} and {}", left, right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

uniffi::setup_scaffolding!();