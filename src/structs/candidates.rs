pub struct Candidate{
    pub name: String,
    pub key: String,
    pub vote_count: u16,    
}

pub struct Candidates {
    pub items: Vec<Candidate>
}

impl Candidates {
    pub fn get_by_key(&mut self, key: &str) -> Option<&mut Candidate> {
        return self.items.iter_mut().find(|f| f.key == key);
    }
}