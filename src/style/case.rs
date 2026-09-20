//! Text case transforms applied to a cell's content before rendering.

/// A text-case transform applied to a cell's content before it is padded
/// and aligned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// Transforms all characters to uppercase (via [`str::to_uppercase`]).
    Uppercase,
    /// Transforms all characters to lowercase (via [`str::to_lowercase`]).
    Lowercase,
}

impl Case {
    /// Applies this case transform to `text`.
    pub fn apply(self, text: &str) -> String {
        match self {
            Case::Uppercase => text.to_uppercase(),
            Case::Lowercase => text.to_lowercase(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_transforms_letters() {
        assert_eq!(Case::Uppercase.apply("Percent Change"), "PERCENT CHANGE");
    }

    #[test]
    fn lowercase_transforms_letters() {
        assert_eq!(Case::Lowercase.apply("Percent Change"), "percent change");
    }
}
