fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];

    println!("Checking {}...", url);

    let response = ureq::get(url).call();

    match response {
        Ok(response) => {
            if response.status() == 200 {
                println!("{} is up!", url);
            } else {
                println!("{} might be down (Status code: {})", url, response.status());
            }
        }
        Err(ureq::Error::Status(code, _response)) => {
            println!("{} might be down (Status code: {})", url, code);
        }
        Err(_) => {
            println!("Failed to reach {}", url);
        }
    }

    Ok(())
}
