pub struct Voter{
    pub key: String,
    pub voted: bool    
}

impl Voter{
    pub fn has_voted(&self) -> bool{
        return self.voted
    }

    pub fn set_voted(&mut self){
        self.voted = true;
    }
}

pub struct Voters{
    pub items: Vec<Voter>,
}

impl Voters {
    pub fn get_by_key(&mut self, key: &str) -> Option<&mut Voter> {
        return self.items.iter_mut().find(|f| f.key == key);
    }

    pub fn has_voters_left(&self) -> bool{
        return self.items.iter().any(|i| i.voted == false);
    }

}