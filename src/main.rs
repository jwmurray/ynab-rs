use curl::easy::{Easy2, Handler, List, WriteError};
use json;

mod token;

// use token;

fn main() {
    struct Collector(Vec<u8>); // a byte vector type to hold curl get data

    /// Define the write handler for the curl get:
    impl Handler for Collector {
        fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            println!("Received {} bytes", data.len());
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));

    let url_base = "https://api.youneedabudget.com/v1/";

    let url_budgets = [url_base, "budgets"].join("");

    //     let url2 = "https://api.youneedabudget.com/v1/budgets/{budget_id}/transactions
    // List transactions"
    let bearer_token = token::get_token("bearer.token"); // store token in private file
    let token_header = ["Authorization: Bearer", bearer_token.as_str()].join(" ");
    let mut list = List::new();
    list.append(&token_header).unwrap(); // Create a list of header items
    easy.get(true).unwrap(); // this is a get
    easy.http_headers(list).unwrap(); // Add the heder list to the http header
    easy.url(&url_budgets).unwrap(); // load the url
    easy.perform().unwrap(); // Execute the http get

    assert_eq!(easy.response_code().unwrap(), 200); // Hope for a 200 rc
    let contents_json = easy.get_ref(); // Get the data from the http response

    // parse the json contents
    let parsed = json::parse(&String::from_utf8_lossy(&contents_json.0)).unwrap();

    println!("{:#}", parsed); // pretty print the json response

    println!("id: {}", parsed["data"]["budgets"][0]["id"]); // print the id

    let id = &parsed["data"]["budgets"][0]["id"];

    let url_transactions = [url_base, "budgets/", id.as_str().unwrap(), "/transactions"].join("");

    println!("url_transactions: {}", url_transactions);

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url(&url_transactions).unwrap(); // load the url
    let mut list2 = List::new();
    list2.append(&token_header).unwrap(); // Create a list of header items

    easy2.http_headers(list2).unwrap(); // Add the heder list to the http header

    easy2.get(true).unwrap();
    easy2.perform().unwrap(); // Execute the http get
    let contents_json2 = easy2.get_ref(); // Get the data from the http response
    let parsed_transactions2 = json::parse(&String::from_utf8_lossy(&contents_json2.0)).unwrap();
    println!("{:#}", parsed_transactions2); // pretty print the json response
}
