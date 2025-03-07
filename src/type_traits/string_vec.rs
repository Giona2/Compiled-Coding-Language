pub trait StringVecExtra {
    fn index_to_pattern(&self, index: usize, pattern: &str) -> Option<Vec<String>>;
}

impl StringVecExtra for Vec<String> {
    fn index_to_pattern(&self, index: usize, pattern: &str) -> Option<Vec<String>> {
        let mut index_of_pattern: Option<usize> = None;
        for (i, element) in self[index..].iter().enumerate() {
            if element == pattern { index_of_pattern = Some(i) }
        }

        if let Some(unwrapped_index_of_pattern) = index_of_pattern {
            return Some(self.clone()[index..=unwrapped_index_of_pattern].to_vec()) 
        } else {
            return None
        }
    }
}
