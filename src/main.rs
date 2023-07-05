const END_PASSWORD: &str = "stop";

struct Voter{
    key: String,
    voted: bool    
}

impl Voter{
    fn has_voted(&self) -> bool{
        return self.voted
    }

    fn set_voted(&mut self){
        self.voted = true;
    }
}

struct Candidate{
    name: String,
    key: String,
    vote_count: u16,    
}

struct Voters{
    items: Vec<Voter>,
}

impl Voters {
    fn get_by_key(&mut self, key: &str) -> Option<&mut Voter> {
        return self.items.iter_mut().find(|f| f.key == key);
    }

    fn has_voters_left(&self) -> bool{
        return self.items.iter().any(|i| i.voted == false);
    }

}

struct Candidates {
    items: Vec<Candidate>
}

impl Candidates {
    fn get_by_key(&mut self, key: &str) -> Option<&mut Candidate> {
        return self.items.iter_mut().find(|f| f.key == key);
    }
}

fn main() {    
    let mut voters = Voters{
        items: vec![
        Voter{
            key: String::from("111"),
            voted: false
        },
        Voter{
            key: String::from("222"),
            voted: false
        },
        Voter{
            key: String::from("333"),
            voted: false
        }
        ]
    };

    let mut candidates =  Candidates{
        items: vec![
            Candidate{
                name: String::from("Candidate 1"),
                key: String::from("1"),
                vote_count: 0
            },
            Candidate{
                name: String::from("Candidate 2"),
                key: String::from("2"),
                vote_count: 0
            },
        ]
    };

    println!("Starting...");

    loop{ 

        if !voters.has_voters_left(){
            let mut adm_input = String::new();
            println!("Insert End Password: ");
            std::io::stdin().read_line(&mut adm_input).expect("Fail to get key input");

            if adm_input.trim() != END_PASSWORD{
                println!("Bad Password");
                continue;
            }
            
            println!("Results:");
            for candidate in candidates.items  {
                println!("{}: {}", candidate.name, candidate.vote_count);
            }
            println!("Vote Ended");            
            std::process::exit(0)
        }


        let mut key_input = String::new();
        println!("Insert Key: ");
        std::io::stdin().read_line(&mut key_input).expect("Fail to get key input");
        let key = key_input.trim();

        if key == END_PASSWORD{
            println!("Results:");
            for candidate in candidates.items {
                println!("{}: {}", candidate.name, candidate.vote_count);
            }
            println!("Vote Ended");            
            std::process::exit(0)
        }
          
        let voter = voters.get_by_key(key);

        if voter.is_none(){
            println!("Invalid Key");
            continue;
        }

        
        if voter.as_ref().unwrap().has_voted(){
            println!("Already voted!");
            continue;
        }
        
        
        let mut vote_input = String::new();
        println!("Insert Vote: ");
        std::io::stdin().read_line(&mut vote_input).expect("Fail to get vote input");
        let vote = candidates.get_by_key(vote_input.trim());
        
        if vote.is_none(){
            println!("Invalid Vote, try again!");
            continue;
        }        

        voter.map(|v| v.set_voted());
        
        vote.map(|v| {
            v.vote_count += 1
        });


    }
}
