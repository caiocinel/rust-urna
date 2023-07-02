use std::io;


fn main() {    
    println!("Starting...");
    let mut candidate_1_votes: i32 = 0;
    let mut candidate_2_votes: i32 = 0;

    loop{
        let mut voted_candidate = String::new();
        io::stdin().read_line(&mut voted_candidate).expect("Failed to read line");

        if voted_candidate.trim() == "1"{
            candidate_1_votes = candidate_1_votes + 1;
            println!("Voted in Candidate 1");
        }
              
        if voted_candidate.trim() == "2"{
            candidate_2_votes = candidate_2_votes + 1;
            println!("Voted in Candidate 2");
        }

        if voted_candidate.trim() != ""{
            println!("Candidate 1: {}", candidate_1_votes);
            println!("Candidate 2: {}", candidate_2_votes);
        }
    }
}
