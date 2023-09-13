use num_cpus;
use url::Url;
use memory_stats::memory_stats;

struct RDLF{
    url: String,
    mem: usize,
    n_th: usize,
}

impl RDLF {
    fn new(_url: &str)-> Self{
        let url = _url.to_owned();
        let mem_st = memory_stats().unwrap();
        let mem = mem_st.virtual_mem;
        let n_th = num_cpus::get();
        Self { url, mem, n_th }
    }

    fn parse(self) -> String{
        let url = Url::parse(&self.url).unwrap();
        url.path().rsplit_once('/').unwrap().1.into()
    }

}