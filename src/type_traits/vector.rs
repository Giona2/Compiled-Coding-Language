pub trait StrVecExtra {
    /// Converts a `Vec<&str>` to a `Vec<String>`
    fn to_string_vec(&self) -> Vec<String>;
}
impl StrVecExtra for Vec<&str> {
    fn to_string_vec(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for value in self.clone() {
            result.push(value.to_string());
        }

        return result
    }
}

pub trait StringVecExtra {
    /// Returns the index of the first matched pattern after a given index
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["3".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]
    ///
    /// // index_of_3 = 2
    /// let index_of_3 = my_vec.find_after_index(1, "3")
    /// ```
    fn find_after_index(&self, index: usize, pattern: &str) -> Option<usize>;

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

    /// Returns a reordered version of this `Vec<String>` where the longest string is at index `0`
    /// and the smallest string is at the last index
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["hello there", "hi", "hello",];
    ///
    /// let ordered_vec = my_vec.sort_by_size();
    ///
    /// // vec!["hello there", "hello", "hi"]
    /// println!("{ordered_vec:?");
    /// ```
    fn sort_by_size(&self) -> Vec<String>;

    /// Returns a reordored version of this `Vec<String>` where the two indexes specified switch
    /// places
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["1", "2", "3", "4"];
    ///
    /// let ordered_vec = my_vec.swap_elements(1, 3);
    ///
    /// // vec!["1", "4", "3", "2"]
    /// println!("{ordered_vec:?}");
    /// ```
    fn swap_elements(&self, first: usize, second: usize) -> Vec<String>;

    /// Checks if the given `char` is the first `char` in one of the elements
    ///
    /// If so, this will return (true, Some(index of first matched element))
    ///
    /// If not, this will return (false, None)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let my_vec: Vec<String> = vec!["one", "two", "three"];
    ///
    /// // (true, Some(1))
    /// println!("{:?}", my_vec.is_start_to_element('t').0);
    /// ```
    fn is_start_to_element(&self, target: char) -> (bool, Option<usize>);

    /// Copies all the elements of `other` into `self`
    ///
    /// Overrides the original append function to allow `other` to not change
    ///
    /// # Examples
    fn append(&mut self, other: &Vec<String>);
}

impl StringVecExtra for Vec<String> {
    fn find_after_index(&self, index: usize, pattern: &str) -> Option<usize> {
        let mut result: Option<usize> = None;

        for (i, element) in self.iter().enumerate() {
            if element == pattern && i > index {
                result = Some(i); break; 
            }
        }

        return result
    }

    fn find(&self, pattern: &str) -> Option<usize> {
        let mut result: Option<usize> = None;

        // Find the first instance of the given pattern
        for (i, element) in self.iter().enumerate() {
            if element == pattern { result = Some(i); break; }
        }

        return result
    }

    fn sort_by_size(&self) -> Vec<String> {
        fn sort_by_size_recur(vector: Vec<String>, current_index: usize) -> Vec<String> {
            if current_index == vector.len() - 1 {
                return vector
            } else {
                let mut largest_element_len: usize = 0;
                let mut largest_element_index: usize = 0;

                let mut i: usize = current_index;
                while i < vector.len() {
                    let current_element_chars: Vec<char> = vector[i].chars().collect();

                    if current_element_chars.len() > largest_element_len {
                        largest_element_len = current_element_chars.len();
                        largest_element_index = i;
                    }

                    i += 1;
                }

                let updated_vector = vector.swap_elements(current_index, largest_element_index);

                return sort_by_size_recur(updated_vector, current_index+1);
            }
        }

        return sort_by_size_recur(self.clone(), 0);
    }

    fn swap_elements(&self, first: usize, second: usize) -> Vec<String> {
        let first_element = self[first].clone();
        let second_element = self[second].clone();

        let mut result = self.clone();

        result[first] = second_element;
        result[second] = first_element;
        
        return result;
    }

    fn is_start_to_element(&self, target: char) -> (bool, Option<usize>) {
        let mut result = false;
        let mut result_index: Option<usize> = None;

        for (element_index, element) in self.iter().enumerate() {
            let element_chars: Vec<char> = element.chars().collect();

            if element_chars[0] == target {
                result = true;
                result_index = Some(element_index);
                break
            }
        }

        return (result, result_index)
    }

    fn append(&mut self, other: &Vec<String>) {
        for element in other { self.push(element.to_owned()); }
    }
}
