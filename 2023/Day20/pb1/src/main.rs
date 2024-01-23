use std::collections::{HashMap, VecDeque};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::alpha1,
    multi::separated_list0,
    branch::alt,
   };
use crate::Module::*;

const NUMBER_OF_BUTTON_ACTIVATIONS: usize = 1000; 

#[derive(PartialEq, Eq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone)]
enum Module {
    Broadcast,
    FlipFlop(String),
    Conjunction(String),
    Output,
}

#[derive(Clone)]
struct Sender {
    module: Module,
    destination: Vec<String>,
    on: bool,
    source_map: HashMap<String, Pulse>,
    low_sent: usize,
    high_sent: usize,
}

impl Sender {
    fn new(module: Module, destination: Vec<String>) -> Self {
        Self { module, destination, on: false, source_map: HashMap::new(), low_sent: 0, high_sent: 0 }    
    }

    fn output() -> Self {
        Self::new(Output, Vec::new())
    }

    fn name(&self) -> String {
        match &self.module {
            Module::Broadcast => "broadcaster",
            Module::FlipFlop(n) | Module::Conjunction(n) => n,
            Module::Output => "output",
        }.to_string()
    }

    fn is_init(&self) -> bool {
        !self.on && self.source_map.values().all(|p| *p == Pulse::Low)
    }

    fn add_source(&mut self, source: String) {
        if let Conjunction(_) = self.module { 
            self.source_map.insert(source, Pulse::Low); 
        }
    }

    fn call(&mut self, pulse: Pulse, from: String) -> Option<(String, Pulse, Vec<String>)> {
        //println!("{} -{}-> {}", from, match pulse { Pulse::High => "high", Pulse::Low => "low" }, self.name());
        let p = match self.module {
            Broadcast => pulse,
            FlipFlop(_) => match pulse {
                Pulse::High => return None,
                Pulse::Low => {
                    if self.on {
                        self.on = false;
                        Pulse::Low
                    } else {
                        self.on = true;
                        Pulse::High
                    }
                }
            },
            Conjunction(_) => {
                self.source_map.insert(from, pulse);
                let mut values = self.source_map.values();
                if values.all(|p| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            },
            Output => return None,
        };
        Some((self.name(), p, self.destination.clone()))
    }

    fn send(&mut self, pulse: Pulse, dest: &mut [Sender]) -> Vec<(String, Pulse, Vec<String>)> {
        let mut res: Vec<(String, Pulse, Vec<String>)> = Vec::new();
        for destination in dest.iter_mut() {
            match pulse {
                Pulse::High => self.high_sent += 1,
                Pulse::Low => self.low_sent += 1,
            };
            if let Some(tuple) = destination.call(pulse, self.name()) {
                res.push(tuple);
            }
        }
        //println!("{} -{}-> {:?} // ({}, {})", self.name(), match pulse { Pulse::High => "high", Pulse::Low => "low" }, dest.iter().map(|s| s.name()).collect::<Vec<String>>(), self.low_sent, self.high_sent);
        res
    }
    
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut sender_map = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, sender) = parse_line(&line).unwrap();
            sender_map.insert(sender.name(), sender);
        }
    }


    let mut source_dest: Vec<(String, Vec<String>)> = Vec::new();
    for sender_name in sender_map.keys() {
        let destinations = sender_map.get(sender_name).unwrap().destination.clone();
        source_dest.push((sender_name.clone(), destinations));
    }
    for (source, dest) in source_dest {
        for d in dest {
            if let Some(obj) = sender_map.get_mut(&d) {
                obj.add_source(source.clone());
            }
        }
    }

    let mut button_activation_count = 0;
    let mut low_high_for_button: Vec<(usize, usize)> = Vec::new();
    low_high_for_button.push((0, 0));
    // computation
    loop {
        button_activation_count += 1;
        let mut broadcaster = sender_map.get_mut("broadcaster").unwrap().clone();
        let mut to_send: VecDeque<(String, Pulse, Vec<String>)> = VecDeque::from([broadcaster.call(Pulse::Low, "button".to_string()).unwrap()]);

        while !to_send.is_empty() {
            let (name, pulse, dest) = to_send.pop_front().unwrap();
            let mut dest_obj = Vec::new();
            for d in dest.iter() {
                match sender_map.get(d) {
                    Some(obj) => dest_obj.push(obj.clone()),
                    None => dest_obj.push(Sender::output()),
                }
            }
            let sender = sender_map.get_mut(&name).unwrap();
            let res = sender.send(pulse, &mut dest_obj);
            for dest in dest_obj {
                sender_map.insert(dest.name(), dest);
            }
            to_send.extend(res);
        }

        low_high_for_button.push(
            (button_activation_count + sender_map.values().map(|s| s.low_sent).sum::<usize>(), 
            sender_map.values().map(|s| s.high_sent).sum::<usize>())
        );

        println!("{} button activations", button_activation_count);

        // end loop
        if sender_map.values().all(|s| s.is_init()) {
            break;
        }
        if button_activation_count == NUMBER_OF_BUTTON_ACTIVATIONS {
            println!("All done");
            break;
        }
    }

    let size_of_loop = button_activation_count;
    let number_of_loops = NUMBER_OF_BUTTON_ACTIVATIONS / size_of_loop;
    let remainder = NUMBER_OF_BUTTON_ACTIVATIONS % size_of_loop;

    let total_low_count = low_high_for_button.last().unwrap().0 * number_of_loops + low_high_for_button[remainder].0;
    let total_high_count = low_high_for_button.last().unwrap().1 * number_of_loops + low_high_for_button[remainder].1;
    let res = total_low_count * total_high_count;

    println!("{total_low_count}*{total_high_count}={res}, loop after {button_activation_count} button activations");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parsing
fn parse_recipients(input: &str) -> IResult<&str, Vec<String>> {
    let (input, recipients) = separated_list0(tag(", "), alpha1)(input)?;
    Ok((input, recipients.into_iter().map(|s| s.to_string()).collect()))
}

fn parse_broadcast(input: &str) -> IResult<&str, Sender> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, recipients) = parse_recipients(input)?;
    Ok((input, Sender::new(Module::Broadcast, recipients)))
}

fn parse_flipflop(input: &str) -> IResult<&str, Sender> {
    let (input, _) = tag("%")(input)?;
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, recipients) = parse_recipients(input)?;
    Ok((input, Sender::new(Module::FlipFlop(id.to_string()), recipients)))
}

fn parse_conjunction(input: &str) -> IResult<&str, Sender> {
    let (input, _) = tag("&")(input)?;
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, recipients) = parse_recipients(input)?;
    Ok((input, Sender::new(Module::Conjunction(id.to_string()), recipients)))
}
 
fn parse_line(input: &str) -> IResult<&str, Sender> {
    alt((parse_broadcast, parse_flipflop, parse_conjunction))(input)
}
   
