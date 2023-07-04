const END_PASSWORD: &str = "stop";
const VOTER_1_KEY: &str = "111";
const VOTER_2_KEY: &str = "222";
const VOTER_3_KEY: &str = "333";

fn main() {    

    println!("Starting...");
    let mut candidate_1_votes: i32 = 0;
    let mut candidate_2_votes: i32 = 0;
    let mut voter_1_voted: bool = false;
    let mut voter_2_voted: bool = false;
    let mut voter_3_voted: bool = false;
    let mut current_voter: i8 = 0;

    loop{ 
        if voter_1_voted && voter_2_voted && voter_3_voted{
            let mut adm_input = String::new();
            println!("Insert End Password: ");
            std::io::stdin().read_line(&mut adm_input).expect("Fail to get key input");

            if adm_input.trim() != END_PASSWORD{
                println!("Bad Password");
                continue;
            }

            println!("Vote Ended");
            println!("Candidate 1: {}", candidate_1_votes);
            println!("Candidate 2: {}", candidate_2_votes);
            std::process::exit(0)
        }


        let mut key_input = String::new();
        println!("Insert Key: ");
        std::io::stdin().read_line(&mut key_input).expect("Fail to get key input");
        let key = key_input.trim();

        if key == END_PASSWORD{
            println!("Vote Ended");
            println!("Candidate 1: {}", candidate_1_votes);
            println!("Candidate 2: {}", candidate_2_votes);
            std::process::exit(0)
        }

        if key == VOTER_1_KEY{
            current_voter = 1;
        }else if key == VOTER_2_KEY {
            current_voter = 2;
        }else if key == VOTER_3_KEY {
            current_voter = 3;
        }else {
            println!("Bad voter key");
            continue;
        }

        if current_voter == 1 && voter_1_voted || current_voter == 2 && voter_2_voted || current_voter == 3 && voter_3_voted{
            println!("Already voted!");
            continue;
        }


        let mut vote_input = String::new();
        println!("Insert Vote: ");
        std::io::stdin().read_line(&mut vote_input).expect("Fail to get vote input");
        let vote = vote_input.trim();

        if vote != "1" && vote != "2"{
            println!("Invalid Vote, try again!");
            continue;
        }

        if vote == "1"{
            candidate_1_votes = candidate_1_votes + 1;
            println!("Voted in Candidate 1");
        }
              
        if vote == "2"{
            candidate_2_votes = candidate_2_votes + 1;
            println!("Voted in Candidate 2");
        }
        
        voter_1_voted = current_voter == 1 || voter_1_voted;
        voter_2_voted = current_voter == 2 || voter_2_voted;
        voter_3_voted = current_voter == 3 || voter_3_voted;
    }
}
