use std::collections::HashMap;

pub struct MockData {
    seed: u64,
}

impl MockData {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    fn next_seed(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }

    pub fn integer(&mut self, min: i64, max: i64) -> i64 {
        if max <= min {
            return min; // Avoid division by zero
        }
        min + (self.next_seed() % ((max - min + 1) as u64)) as i64
    }

    pub fn string(&mut self, length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        (0..length)
            .map(|_| {
                let index = self.integer(0, CHARSET.len() as i64 - 1) as usize;
                CHARSET[index] as char
            })
            .collect()
    }

    pub fn name(&mut self) -> String {
        let first_names = ["Alice", "Bob", "Charlie", "David", "Eve"];
        let last_names = ["Smith", "Johnson", "Williams", "Brown", "Davis"];

        let first = first_names[self.integer(0, first_names.len() as i64 - 1) as usize];
        let last = last_names[self.integer(0, last_names.len() as i64 - 1) as usize];

        format!("{} {}", first, last)
    }

    pub fn email(&mut self) -> String {
        let domains = ["example.com", "mail.com", "test.org"];
        let domain = domains[self.integer(0, domains.len() as i64 - 1) as usize];

        format!("{}@{}", self.string(8), domain)
    }

    pub fn phone(&mut self) -> String {
        format!(
            "+{}-{}-{}",
            self.integer(1, 99),
            self.integer(100, 999),
            self.integer(1000, 9999)
        )
    }

    pub fn address(&mut self) -> String {
        format!(
            "{} {}, {}",
            self.integer(100, 999),
            self.string(8),
            self.string(6)
        )
    }

    pub fn country(&mut self) -> String {
        let countries = ["USA", "Canada", "UK", "Germany", "France"];
        countries[self.integer(0, countries.len() as i64 - 1) as usize].to_string()
    }

    pub fn job_title(&mut self) -> String {
        let jobs = ["Software Engineer", "Data Scientist", "Doctor", "Artist", "Teacher"];
        jobs[self.integer(0, jobs.len() as i64 - 1) as usize].to_string()
    }

    pub fn credit_card(&mut self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.integer(1000, 9999),
            self.integer(1000, 9999),
            self.integer(1000, 9999),
            self.integer(1000, 9999)
        )
    }

    pub fn date(&mut self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.integer(1900, 2023),
            self.integer(1, 12),
            self.integer(1, 28)
        )
    }

    pub fn time(&mut self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.integer(0, 23),
            self.integer(0, 59),
            self.integer(0, 59)
        )
    }

    pub fn paragraph(&mut self, sentences: usize) -> String {
        let sentence_lengths: Vec<usize> = (0..sentences)
            .map(|_| self.integer(5, 15) as usize)
            .collect();

        sentence_lengths
            .into_iter()
            .map(|len| self.sentence(len))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn sentence(&mut self, words: usize) -> String {
        let word_lengths: Vec<usize> = (0..words)
            .map(|_| self.integer(3, 10) as usize)
            .collect();

        let mut sentence = word_lengths
            .into_iter()
            .map(|len| self.string(len))
            .collect::<Vec<String>>()
            .join(" ");
        
        sentence.push('.');
        sentence
    }

    pub fn uuid(&mut self) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.integer(0, u32::MAX as i64),
            self.integer(0, u16::MAX as i64),
            self.integer(0, u16::MAX as i64),
            self.integer(0, u16::MAX as i64),
            self.integer(0, u64::MAX as i64) & 0xFFFFFFFFFFFF
        )
    }

    pub fn json(&mut self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("uuid".to_string(), self.uuid());
        map.insert("name".to_string(), self.name());
        map.insert("email".to_string(), self.email());
        map.insert("phone".to_string(), self.phone());
        map.insert("address".to_string(), self.address());
        map.insert("country".to_string(), self.country());
        map.insert("job_title".to_string(), self.job_title());
        map.insert("credit_card".to_string(), self.credit_card());
        map.insert("date".to_string(), self.date());
        map.insert("time".to_string(), self.time());
        map.insert("bio".to_string(), self.paragraph(3));
        map
    }
  }
