use num_cpus;
use reqwest::blocking::Client;
use url::Url;
use memory_stats::memory_stats;
use manic::threaded::Downloader;

pub fn run(_url: String){
    print!("{:} {_url}", num_cpus::get());
}

pub fn get_fsize(_url: String){
    print!("{:} {_url}", num_cpus::get());
}

pub fn get_url(_url: &str) {
    let url = parse_file_name(_url);
    print!("{url:?}");
}

pub fn parse_file_name(url: &str) -> String {
    let url = Url::parse(url).unwrap();
    url.path().rsplit_once('/').unwrap().1.into()
}

pub fn test(_url: String){
    let client = Client::new();
    // let rdlf = RDLF::new(&_url);
    let name = parse_file_name(&_url);
    // let name = rdlf.parse();
    let response = client.get(&_url)
        .send()
        .unwrap();
    let fname = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|h| h.to_str().ok())
        .or(Some(&name));
    print!("{fname:?}");
}

pub fn debug(_url: String){
    let client = Client::new();
    let response = client.get(&_url)
        .send()
        .unwrap();

    let content_disposition = response
        .headers();
        // .get(reqwest::header::CONTENT_DISPOSITION)
        // .unwrap();

    let total = response.content_length().unwrap();
    let thread_num = num_cpus::get();
    let mem_st = memory_stats().unwrap();
    let (p, v) = (mem_st.physical_mem, mem_st.virtual_mem) ;
    print!("{total} {thread_num} {p} {v} {content_disposition:?}");
}

pub fn down(_url: String) -> Result<(), manic::ManicError> {
    let workers: u8 = 10;
    let client = Downloader::new(&_url, workers)?;
    let _ = client.download()?;
    Ok(())
}