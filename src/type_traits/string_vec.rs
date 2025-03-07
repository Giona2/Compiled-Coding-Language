pub trait StringVecExtra {
    /// Returns a section of this vector from index to the first instance of pattern after the
    /// given index
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]
    ///
    /// // vec_slice = ["2", "3"]
    /// let vec_slice: Vec<String> = my_vec.index_to_pattern(1, "3")
    /// ```
    fn index_to_pattern(&self, index: usize, pattern: &str) -> Option<Vec<String>>;

    /// Returns the index of the first instance of `pattern` in this vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["2".to_string(), "3".to_string(), "4".to_string(), "3".to_string()]
    ///
    /// // index_of_3_after_1 = 1
    /// let index_of_3: Vec<String> = my_vec.find("3")
    /// ```
    fn find(&self, pattern: &str) -> Option<usize>;
}

impl StringVecExtra for Vec<String> {
    fn index_to_pattern(&self, index: usize, pattern: &str) -> Option<Vec<String>> {
        // Find the first instance of the given pattern after the given index
        let mut index_of_pattern: Option<usize> = None;
        for (i, element) in self.iter().enumerate() {
            if element == pattern && i > index {
                index_of_pattern = Some(i); break; 
            }
        }

        // Returns the chunk of self from given index to first instance of pattern
        if let Some(unwrapped_index_of_pattern) = index_of_pattern {
            return Some(self.clone()[index..=unwrapped_index_of_pattern].to_vec()) 
        } else {
            return None
        }
    }

    fn find(&self, pattern: &str) -> Option<usize> {
        let mut result: Option<usize> = None;

        // Find the first instance of the given pattern
        for (i, element) in self.iter().enumerate() {
            if element == pattern { result = Some(i); break; }
        }

        return result
    }
}
