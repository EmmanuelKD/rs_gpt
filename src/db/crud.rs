use std::time;

// write the crud for saving query, 
//updating and deleting, plus reading by
// created date or just read all of them
pub struct Prompt_and_res {
    prompt: String,
    res: String,
    error_message: String,
    created_at: std::time,
}

pub fn add_prompt_and_res() {
    let now = Instant::now();
    
}
