use std::io;


fn main() {    
    println!("Starting...");
    let mut candidate_1_votes: i32 = 0;
    let mut candidate_2_votes: i32 = 0;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail to get input");
        let vote = input.trim();

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

        if vote != ""{
            println!("Candidate 1: {}", candidate_1_votes);
            println!("Candidate 2: {}", candidate_2_votes);
        }
    }
}
