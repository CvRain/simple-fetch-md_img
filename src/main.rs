use regex::Regex;
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few argument at least one");
        return Ok(());
    }

    println!("Get url: {}", &args[1]);
    let body = get(&args[1])?.text()?;
    if body.len() <= 0 {
        println!("Could not fetch file");
        return Ok(());
    }
    println!("Get data: {} bytes", body.len());

    let re = Regex::new(r"!\[*\]\((.+?)\)")?;
    for captures in re.captures_iter(&body) {
        let url = captures.get(1).map_or("", |m| m.as_str());
        let file_imge = Url::parse(&url).unwrap();
        let file_name = file_imge.path_segments().unwrap().last().unwrap();

        println!("Get image:{}\n\turl:{}", &file_name, &url);
        if !download_file(url, &file_name).is_ok() {
            println!("Download failed!")
        } else {
            println!("Download successfully kfc_v_me_50");
        }
    }
    Ok(())
}

fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let mut out = File::create(path)?;
    copy(&mut response, &mut out)?;
    Ok(())
}
