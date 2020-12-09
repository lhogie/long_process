use rand::Rng;
use std::env;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;

pub struct ProgressMonitorBuilder {
    stack: Vec<&ProgressMonitor>
}

pub const pm_builder :  ProgressMonitorBuilder::new();

impl ProgressMonitorBuilder {
    pub fn create(&mut self, name: String, n: u64) -> &ProgressMonitor {
        let pm =ProgressMonitor::new(name, n);
        self.stack.push(pm);
        pm
    }

    pub fn new(){
        ProgressMonitorBuilder {
            stack : Vec::new()
        }        
    }
}

pub struct ProgressMonitor {
    startInstant: Instant,
    name: String,
    target: u64,
    progress: u64,
//    h: std::thread::JoinHandle<bool>,
}

impl Drop for ProgressMonitor {
    fn drop(&mut self) {
        println!("destructor");
        self.stop();
    }
}


impl ProgressMonitor {
    pub fn new(name: String, n: u64) -> Self {
        print!("> START {}", name);

        let (tx, rx) = channel();

        let pm : ProgressMonitor {
            startInstant: Instant::now(),
            name: name,
            target: n,
            progress: 0,
        };
        start(pm);
        pm
    }

    fn thread_code(pm: &ProgressMonitor) {
        while !ProgressMonitor::completed(pm) {
            println!("progress {}", ProgressMonitor::ratio(pm));
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn completed(&self) -> bool {
        self.progress == self.target
    }

    pub fn stop(&self) {
        print!(
            "> END {}, duration: {}s",
            self.name,
            self.startInstant.elapsed().as_secs()
        );
//        self.h.join().expect("join failed");
    }

    pub fn ratio(&self) -> f64 {
        self.progress as f64 / self.target as f64
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[0].parse::<u32>().unwrap();
    let mut array: Vec<u32> = Vec::with_capacity(n as usize);
    let rng = rand::thread_rng();

    println!("Generating array");

    for _ in 0..n {
        array.push(rng.gen());
    }

    println!("{:?}", &array[..10]);

    let mut lp = ProgressMonitor::new("Bubble-sorting".to_string(), n as u64);
    h: thread::spawn(|| {
        ProgressMonitor::thread_code(self);
        true
    }),


    for i in 0..array.len() {
        lp.progress += 1;

        for j in 0..array.len() {
            if array[i] < array[j] {
                let tmp = array[i];
                array[i] = array[j];
                array[j] = tmp;
            }
        }
    }

    lp.stop();

    println!("{:?}", &array[..10]);
}
