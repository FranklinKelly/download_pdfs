use csv;
use clap::Parser;
use reqwest::blocking;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    links_path: std::path::PathBuf,
    download_path: std::path::PathBuf
}

struct PDF {
    name: String,
    url: String
}

fn read_csv(file_path: &PathBuf) -> Result<Vec<PDF>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .expect("Failed to create file reader");

    // let mut elements: Vec<String> = Vec::new();
    let mut pdfs: Vec<PDF> = Vec::new();
    for result in rdr.records() {
        let result = result.unwrap();
        let name: &str = csv::StringRecord::get(&result, 0).unwrap().trim();
        let url: &str = csv::StringRecord::get(&result, 1).unwrap().trim();

        let pdf = PDF {
            name: String::from(name),
            url: String::from(url)
        };

        pdfs.insert(pdfs.len(), pdf);
    }

    Ok(pdfs)

}


fn download_file(url: &str, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = blocking::get(url)
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let mut dest = File::create(destination)
        .map_err(|e| format!("Failed to create file! {}", e))?;
    
    let content = response.bytes()
        .map_err(|e| format!("Failed to read response bytes! {}", e))?;
    
    copy(&mut content.as_ref(), &mut dest)
        .map_err(|e| format!("Failed to copy content! {}", e))?;
    
    Ok(())
}

fn download_files(pdfs: Vec<PDF>, folder: PathBuf) {
    for pdf in pdfs {
        let name = pdf.name;
        let folder = folder.to_str().unwrap();

        println!("Downloading file {name} to {folder}");
        
        let dest = format!("{folder}{name}.pdf");
        let dest = Path::new(&dest);
        let _ = download_file(&pdf.url, &dest).map_err(|e| format!("Failed to download file {name}: {e}"));
    }
    println!("Finished downloading all files!");
}

fn main() {
    let args = Cli::parse();

    println!("Getting links from {:?} and downloading to {:?}", args.links_path, args.download_path);

    let pdfs = read_csv(&args.links_path).expect("Failed to understand the csv file");
    download_files(pdfs, args.download_path);
    
}