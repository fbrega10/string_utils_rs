#[derive(Debug)]
pub struct StringBuilder {
    content: String,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder {
            content: String::new(),
        }
    }
    pub fn to_string(&self) -> String {
        self.content.clone()
    }

    pub fn append(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn trim(&mut self) {
        //trimming a string removing prefix/suffix spaces
        if self.content.len() < 2 {
            println!("string lenght must be at least of two characters long. Exit 1");
            return;
        }
        let mut start = 0;
        let mut end: usize = self.content.len() as usize - 1;
        for (i, c) in self.content.char_indices() {
            match c != ' ' {
                true => {
                    start = i;
                    println!("the value of start is {}", start);
                    break;
                }
                _ => (),
            }
        }
        for (i, c) in self.content.chars().rev().enumerate() {
            match c != ' ' {
                true => {
                    end = end - i;
                    println!("the value of start is {}", end);
                    break;
                }
                _ => (),
            }
        }
        self.content = self.content.chars().skip(start).take(end).collect()
    }

    pub fn substring(&self, start_index: usize, end_index: usize) -> Option<String> {
        if start_index < end_index || self.content.len() - 1 < end_index {
            println!(
                "invalid indexes, please choose the correct indexes for the string size : {}",
                self.content.len()
            );
            return None;
        }
        Some(
            self.content
                .chars()
                .skip(start_index)
                .take(end_index - start_index)
                .collect(),
        )
    }

    pub fn left(&self, elements: usize) -> Option<String> {
        let start: usize = 0;
        if self.content.len() - 1 < elements {
            println!("index out of bounds");
            return None;
        }
        Some(self.content.chars().skip(start).take(elements).collect())
    }

    pub fn right(&self, elements: usize) -> Option<String> {
        if self.content.len() - 1 < elements {
            println!("index out of bounds");
            return None;
        }
        Some(
            self.content
                .chars()
                .skip(self.content.len() - elements)
                .take(elements)
                .collect(),
        )
    }
}
