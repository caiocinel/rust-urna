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
    fn get_by_key(&self, key: String) -> Option<&Voter>{
        return self.items.iter().find(|f| f.key == key);
    }

    fn has_voters_left(&self) -> bool{
        return self.items.iter().any(|i| i.voted == false);
    }

}

fn main() {    
    let voters = Voters{
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

    let candidates: Vec<Candidate> = vec![
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
    ];

    println!("Starting...");
    let mut current_voter: i8;

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
            for candidate in &candidates  {
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
            for candidate in &candidates {
                println!("{}: {}", candidate.name, candidate.vote_count);
            }
            println!("Vote Ended");            
            std::process::exit(0)
        }
          
        let voter = &voters.get_by_key(String::from(key));

        if voter.is_none(){
            println!("Invalid Key");
            continue;
        }

        println!("{}", voter.unwrap().has_voted());
        voter.unwrap().set_voted();  // Stopped Here ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
        println!("{}", voters.get_by_key(String::from(key)).unwrap().has_voted());
       
        // if current_voter == 1 && voter_1_voted || current_voter == 2 && voter_2_voted || current_voter == 3 && voter_3_voted{
        //     println!("Already voted!");
        //     continue;
        // }


        // let mut vote_input = String::new();
        // println!("Insert Vote: ");
        // std::io::stdin().read_line(&mut vote_input).expect("Fail to get vote input");
        // let vote = vote_input.trim();

        // if vote != "1" && vote != "2"{
        //     println!("Invalid Vote, try again!");
        //     continue;
        // }

        // if vote == "1"{
        //     candidate_1_votes = candidate_1_votes + 1;
        //     println!("Voted in Candidate 1");
        // }
              
        // if vote == "2"{
        //     candidate_2_votes = candidate_2_votes + 1;
        //     println!("Voted in Candidate 2");
        // }
        
        // voter_1_voted = current_voter == 1 || voter_1_voted;
        // voter_2_voted = current_voter == 2 || voter_2_voted;
        // voter_3_voted = current_voter == 3 || voter_3_voted;
    }
}
