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
}

impl StringVecExtra for Vec<String> {
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
}


pub trait VecExtra<T: Eq + Clone> {
    /// Returns the index of the first instance of `pattern` in this vector
    ///
    /// Returns None if `pattern` is not found in this vector
    fn find(&self, pattern: &T) -> Option<usize>;

    /// Returns the index of the first instance of any elements in `patterns` in this vector
    ///
    /// Returns None if none of the elements in `patterns` are found
    fn find_from_vec(&self, patterns: &[T]) -> Option<usize>;

    /// Returns the index of the first matched pattern after a given index
    fn find_after_index(&self, index: usize, pattern: &T) -> Option<usize>;

    /// Copys all elements from `other` and appends them into this vector
    ///
    /// This does not require the other vector be mutable
    fn append_immut(&mut self, appended_slice: &[T]);
}
impl<T: Eq + Clone> VecExtra<T> for Vec<T> {
    fn find(&self, pattern: &T) -> Option<usize> {
        let mut result: Option<usize> = None;

        // Find the first instance of the given pattern
        for (i, element) in self.iter().enumerate() {
            if element == pattern { result = Some(i); break; }
        }

        return result
    }

    fn find_from_vec(&self, patterns: &[T]) -> Option<usize> {
        let mut result: Option<usize> = None;

        for (element_index, element) in self.iter().enumerate() {
            let mut pattern_found = false;

            for pattern in patterns.iter() {
                if element == pattern { result = Some(element_index); pattern_found = true; break; }
            }

            if pattern_found { break }
        }

        return result
    }

    fn find_after_index(&self, index: usize, pattern: &T) -> Option<usize> {
        let mut result: Option<usize> = None;

        for (i, element) in self.iter().enumerate() {
            if element == pattern && i > index {
                result = Some(i); break; 
            }
        }

        return result
    }

    fn append_immut(&mut self, appended_slice: &[T]) {
        for element in appended_slice.iter() {
            self.push(element.clone());
        }
    }
}

#[macro_export]
/// display_vec_pointer!(vec, "begin"; 2 => Blue,)
macro_rules! vec_pointer_debug {
    ($vec:expr, $($index:expr => $color:ident),*) => {{
        use colored::Color;
        use colored::Colorize;
        for (i, element) in $vec.iter().enumerate() {
            $(
            if i == $index {
                println!("{} {:?}", i.to_string().color(Color::$color), element);
            }
            )*
            println!("{} {:?}", i.to_string().red(), element);
        }
    }};
}

/*
pub trait VecDebugging<T: Debug> {
    /// Prints this entire vector, where the value of index `pointer` is colored
    fn display_pointer(&self, pointer: usize) -> ColoredString;
}
impl<T: Debug> VecDebugging<T> for Vec<T> {
    fn display_pointer(&self, pointer: usize) -> ColoredString {
        let mut result: ColoredString = String::new().into();
        let color = Color::Green;
        let test: ColoredString = "".color(color);

        for (i, element) in self.iter().enumerate() {
            if i != pointer { result = format!("{}\n{} {:?}", result, i.to_string().red(), element).into()  }
            else            { result = format!("{}\n{} {:?}", result, i.to_string().blue(), element).into() }
        }

        return result
    }
}
*/
