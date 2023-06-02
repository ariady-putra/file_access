use crate::internal::traits::to_vec_string::*;

impl<Line: AsRef<str>> ToVecString for Vec<Line> {
    fn to_vec_string(&self) -> Vec<String> {
        self.iter().map(|line| line.as_ref().to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        any::{Any, TypeId},
        io::Result,
    };

    fn is_vec_string(a: &dyn Any) -> bool {
        a.type_id() == TypeId::of::<Vec<String>>()
    }

    #[test]
    fn vec_str_to_vec_string() -> Result<()> {
        Ok({
            // Arrange
            let vec_str = vec!["hello", "world"];

            // Action
            let vec_string = vec_str.to_vec_string();

            // Assert
            assert!(
                is_vec_string(&vec_string),
                "vec_string should be a Vec<String>"
            );
            assert!(
                !is_vec_string(&vec_str),
                "vec_str should NOT be a Vec<String>"
            );
        })
    }
}
