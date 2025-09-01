mod algo {
    /// For more info on the 2d vector visit https://users.rust-lang.org/t/is-there-a-better-way-to-making-a-2d-vector-than-this/73858
    #[derive(Debug)]
    struct Matrix<T> {
        data: Vec<T>,
        num_columns: usize,
    }

    impl<T: Copy> Matrix<T> {
        pub fn get_value(&self, col: usize, row: usize) -> T {
            assert!(col < self.num_columns);
            let index = self.num_columns * row + col;
            self.data[index]
        }

        pub fn set_value(&mut self, col: usize, row: usize, value: T) {
            assert!(col < self.num_columns);
            let index = self.num_columns * row + col;
            self.data[index] = value;
        }
    }

    pub fn levenshtein_algorithm(query: &str, content: &str) -> usize {
        let query_length = query.len();
        let content_length = content.len();

        // create a 2d vector
        let mut dp: Matrix<usize> = Matrix {
            data: vec![0; (query_length + 1) * (content_length + 1)], // fill vec with 0
            num_columns: query_length + 1,
        };

        for i in 0..=query_length {
            dp.set_value(i, 0, i); // fill first column for each query char
        }

        for i in 0..=content_length {
            dp.set_value(0, i, i); // fill first row for query content char
        }

        let query_chars: Vec<char> = query.chars().collect();
        let content_chars: Vec<char> = content.chars().collect();

        for i in 1..=query_length {
            for j in 1..=content_length {
                let cost: usize = if query_chars[i - 1] == content_chars[j - 1] {
                    0
                } else {
                    1
                };

                let min_value = (dp.get_value(i - 1, j) + 1) // deletion
                    .min(dp.get_value(i, j - 1) + 1) // insertion
                    .min(dp.get_value(i - 1, j - 1) + cost); // substitution

                dp.set_value(i, j, min_value);
            }
        }

        dp.get_value(query_length, content_length)
    }
}

struct Fuzzy<'a> {
    item: &'a str,
    distance: usize,
}

pub fn fuzzy_search<'a>(
    query: &str,
    list: &[&'a str],
    max_distance: Option<usize>,
) -> (Vec<&'a str>, usize) {
    let query = &query.to_lowercase();
    let max_distance = max_distance.unwrap_or(4);

    let mut mp: Vec<Fuzzy> = list
        .iter()
        .map(|&item| Fuzzy {
            item,
            distance: algo::levenshtein_algorithm(query, &item.to_lowercase()),
        })
        .filter(|Fuzzy { distance, .. }| *distance <= max_distance)
        .collect();

    mp.sort_by_key(|a| a.distance);

    let mut dist_acc = 0;

    let result = mp
        .iter()
        .map(|Fuzzy { item, distance }| {
            dist_acc += distance;
            *item
        })
        .collect::<Vec<&str>>();

    (result, dist_acc)
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_min_distance() {
        let query = "cat";
        let content = "clipcat";
        let result = super::algo::levenshtein_algorithm(query, content);

        assert_eq!(4, result)
    }

    #[test]
    fn should_return_correct_search() {
        let query = "cat";
        let list = ["clipcat", "clipboard", "rust-app", "cat", "act"];
        let result = super::fuzzy_search(query, &list, Some(5));

        assert_eq!(vec!["cat", "act", "clipcat"], result.0)
    }
}
