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

    pub fn first_name(&mut self) -> String {
        let first_names = ["Alice", "Bob", "Charlie", "David", "Eve"];
        first_names[self.integer(0, first_names.len() as i64 - 1) as usize].to_string()
    }

    pub fn last_name(&mut self) -> String {
        let last_names = ["Smith", "Johnson", "Williams", "Brown", "Davis"];
        last_names[self.integer(0, last_names.len() as i64 - 1) as usize].to_string()
    }

    pub fn name(&mut self) -> String {
        let first = self.first_name();
        let last = self.last_name();

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

    pub fn adjective(&mut self) -> String {
        let words = ["other", "new", "good", "high", "old", "great", "big", "small", "large", "national", "young", "different", "black", "long", "little", "important", "political", "bad", "white", "real", "best", "right", "social", "only", "public", "sure", "low", "early", "able", "human", "local", "late", "hard", "major", "better", "economic", "strong", "possible", "whole", "free", "military", "federal", "international", "full", "special", "easy", "clear", "recent", "certain", "personal", "open", "red", "difficult", "available", "likely", "short", "single", "medical", "current", "wrong", "private", "past", "foreign", "fine", "common", "poor", "natural", "significant", "similar", "hot", "dead", "central", "happy", "serious", "ready", "simple", "left", "physical", "general", "environmental", "financial", "blue", "democratic", "dark", "various", "entire", "close", "legal", "religious", "cold", "final", "main", "green", "nice", "huge", "popular", "traditional", "cultural"];
        words[self.integer(0, words.len() as i64 - 1) as usize].to_string()
    }

    pub fn noun(&mut self) -> String {
        let words = ["time", "year", "people", "way", "day", "man", "thing", "woman", "life", "child", "world", "school", "state", "family", "student", "group", "country", "problem", "hand", "part", "place", "case", "week", "company", "system", "program", "question", "work", "government", "number", "night", "point", "home", "water", "room", "mother", "area", "money", "story", "fact", "month", "lot", "right", "study", "book", "eye", "job", "word", "business", "issue", "side", "kind", "head", "house", "service", "friend", "father", "power", "hour", "game", "line", "end", "member", "law", "car", "city", "community", "name", "president", "team", "minute", "idea", "kid", "body", "information", "back", "parent", "face", "others", "level", "office", "door", "health", "person", "art", "war", "history", "party", "result", "change", "morning", "reason", "research", "girl", "guy", "moment", "air", "teacher", "force", "education"];
        words[self.integer(0, words.len() as i64 - 1) as usize].to_string()
    }

    pub fn color(&mut self) -> String {
        let words = ["red", "orange", "yellow", "green", "blue", "purple", "white", "black", "cyan", "magenta"];
        words[self.integer(0, words.len() as i64 - 1) as usize].to_string()
    }

    pub fn word(&mut self) -> String {
        let i = self.integer(0, 2);
        println!("Generated random integer {}", i);
        match i {
            0 => self.adjective(),
            1 => self.noun(),
            2 => self.color(),
            _ => "the".to_string(),
        }
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
        let mut sentence = (0..words)
            .map(|_| self.word())
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
