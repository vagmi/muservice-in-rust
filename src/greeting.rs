pub fn greet(name: &str) -> String {
    format!("hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn should_greet() {
        let name = "confoo";
        let result = greet(&name);
        assert_eq!("hello confoo".to_string(), result);
    }

}

