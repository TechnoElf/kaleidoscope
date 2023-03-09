use crate::circ::sequence::Circuit;

pub fn parse(src: String) -> Circuit {
    let normal = normalise(src);
    let tokens = tokenise(normal);

    let circ = Circuit::new();
    circ
}

pub fn normalise(src: String) -> String {
    let src = remove_comments(src);
    let src = remove_whitespace(src);
    src
}

pub fn remove_comments(src: String) -> String {
    src.lines().collect()
}

pub fn remove_whitespace(src: String) -> String {
    src.lines().collect()
}

pub fn tokenise(src: String) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC0: &str = "

    ";

    #[test]
    fn parse_source() {
        
    }
}
