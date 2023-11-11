use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FunctionWord {
    pub entry: String,
    pub part_of_speech: String,
    pub details: Vec<String>,
}

fn get_function_word_list() -> Vec<FunctionWord> {
    let rdr = csv::Reader::from_path("resources/function_word.csv");
    match rdr {
        Ok(mut rdr) => {
            let mut case_clitics = Vec::new();
            for result in rdr.deserialize() {
                if let Ok(result) = result {
                    let case_clitic: FunctionWord = result;
                    case_clitics.push(case_clitic);
                } else {
                    panic!("Validation Error")
                }
            }
            case_clitics
        }
        Err(_) => {
            panic!("Error reading function word csv")
        }
    }
}
