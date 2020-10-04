/*************************
The state machine:

State::sleep:
    transition: if it's day time -> idle state
    transition: if battery low -> feint state

State::idle:
    transition: if battery is low -> feint state
    transition: if it's night time -> sleep state
    transition: if dizzyness is high -> dizzy state
    transition: if mouse is in long range and out of range -> chase state
    transition: if boredom is too high -> wander state

    function: if mouse is clicked -> do a trick
    function: if cpu is high, increase dizzyness quickly, 
        otherwise decrease it slowly
    function: increase boredom

State::feint:
    transition: if battery is not low and dizzyness isn't high -> sleep state

    function: if cpu is low, decrease dizzyness quickly

State::dizzy:
    transition: if it's night time -> sleep state (reset dizzyness)
    transition: if battery is low -> feint state
    transition: if dizzyness has subsided -> idle state
    transition: if dizzyness grows too high -> feint state

    function: if cpu is low, decrease dizzyness slowly,
            otherwise, increase dizzyness slowly

State::chase:
    transition: if battery is low or dizzyness is high -> slow_chase state
    transition: if I've reached the target -> idle state
    
    function: always set target to mouse pos
    function: if mouse is clicked, pounce
    function: if cpu is high, increase dizzyness quickly,
            otherwise, decrease it slowly

State::slow_chase:
    ***ordered by priority, speed will be reduced to 0 before 
            any other state changes take place***
    transition: if my speed is > 0, reduce my speed -> slow_chase state
    transition: if the battery is low -> feint state
    transition: if dizzyness is high -> state dizzy
    transition: else -> idle state

    function: always set target to mouse pos
    function: if cpu is high, increase dizzyness quickly, 
        otherwise decrease it slowly

State::wander:
    transition: if it's night time -> sleep state
    transition: if battery is low -> feint state
    transition: if dizzyness is high -> dizzy state
    transition: if mouse is in range and I'm not bored -> chase state
    transition: if I've reached my target,
        if I'm still bored, find a new point -> wander state
        otherwise -> idle state

    function: decrease boredom
    function: if cpu is high, increase dizzyness quickly, 
        otherwise decrease it slowly
************************/
#![allow(dead_code,non_camel_case_types,non_snake_case)]

use std::time::{Duration,SystemTime,UNIX_EPOCH};

const LOW_BATT_LVL:u8 = 20;
const BED_TIME:u8 = 4;
const MORNING:u8 = 12;
const HIGH_CPU:u8 = 230; // 90% of 8 bit max value. approx 230
const HIGH_RAM:u8 = 230; // again 90%

const BORED_THRESH:i32 = 40;
const DIZZY_THRESH_HIGH:i32 = 15;
const DIZZY_THRESH_LOW:i32 = 10;
const TOO_DIZZY_THRESH:i32 = 30;
const DIZZY_SLOW:i32 = 1;
const DIZZY_QUICK:i32 = 2;
const PET_RANGE:i32 = 10;
const PET_LONG_RANGE:i32 = 100;

struct Time {
    hours:u8,
    minutes:u8,
    seconds:u8,
    millis:u32,
    micros:u32,
    nanos:u32
}

fn get_time() -> Time {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)
                            .expect("Time has gone backward.")
                            .as_nanos();
    let u = n / 1_000;
    let m = u / 1_000;
    let sec = m / 1_000;
    let min = sec / 60;
    let h = (min / 60) - 6; // 6 hours diff from UTC to MST
    
    return Time {
        hours: (h % 24) as u8,
        minutes: (min % 60) as u8,
        seconds: (sec % 60) as u8,
        millis: (m % 1000) as u32,
        micros: (u % 1000) as u32,
        nanos: (n % 1000) as u32
    };
}

fn is_day_time(hour:u8) -> bool {
    if MORNING > BED_TIME {
        return !(hour > BED_TIME && hour < MORNING);
    } else {
        return hour > MORNING && hour < BED_TIME;
    }
}



// unsafety: mutation of a static variable can cause data races in 
// multi-thread situations. That's ok here since it just makes it
// more random.
fn sudo_rand(max:u32) -> u32{
    unsafe{
        static mut last:u32 = 0;
        let n = SystemTime::now().duration_since(UNIX_EPOCH)
                                .expect("Time has gone backward.")
                                .as_micros()
                                * (last + 1) as u128;
        last = ((n as i128).abs() % max as i128) as u32;
        return last;
    }
}

fn main() {
    let mut pet1 = Pet::new();
    pet1.print();
    
    let mut info = Specs{
        time: get_time(),
        mouse_pos: (50,40),
        battery: 150,
        mouse_clicked: false,
        cpu: 128,
        ram: 200,
    };

    while info.battery > 0{
        // update the pet
        pet1.update(&info);

        if info.battery >= 1 {
            info.battery -= 1;
        } else {
            info.battery = 0;
        }
        
        // print system specs
        info.print();
        println!();
    }
}

enum States {
    sleep, idle, chase, slow_chase, feint, wander, dizzy
}

fn get_state_str(state:&States) -> &'static str{
    match *state {
        States::sleep => "Sleeping",
        States::idle => "Idle", 
        States::chase => "Chasing", 
        States::slow_chase => "Getting tired", 
        States::feint => "Feinted",
        States::wander => "Wandering",
        States::dizzy => "Dizzy",
    }
}

struct Specs {
    time:Time,
    mouse_pos:(i32,i32),
    battery:u8,
    mouse_clicked:bool,
    cpu:u8,
    ram:u8
}

impl Specs {
    fn print(&self) {
        println!("Time: {}, Mouse: ({},{}) clicked? {}, Batt: {}, CPU: {}, RAM: {}",
        self.time.hours, self.mouse_pos.0, self.mouse_pos.0,self.mouse_clicked, self.battery, self.cpu, self.ram);
    }
}


struct Pet {
    state:States,
    pos:(i32,i32),
    target:(i32,i32),
    range:i32,
    long_range:i32,
    speed:i32,
    boredom:i32,
    bored_thresh: i32,
    dizzyness:i32
}

impl Pet {
    fn new() -> Pet {
        Pet { 
            state:States::sleep, 
            pos:(0,0), 
            target:(0,0), 
            range: PET_RANGE,
            long_range: PET_LONG_RANGE,
            speed:15, 
            boredom:0, 
            bored_thresh:10,
            dizzyness: 0
        }
    }

    fn update(&mut self, info:&Specs) {
        // Some useful calculations
        let target_dx = self.target.0 - self.pos.0;
        let target_dy = self.target.1 - self.pos.1;
        let target_D2 = (target_dx * target_dx) + (target_dy * target_dy);
        let target_dist = isqrt(target_D2 as u32) as i32;

        let mouse_dx = info.mouse_pos.0 - self.pos.0;
        let mouse_dy = info.mouse_pos.1 - self.pos.1;
        let mouse_dist_2 = (mouse_dx * mouse_dx) + (mouse_dy * mouse_dy);
        let mouse_dist = isqrt(mouse_dist_2 as u32) as i32;
        
        // useful cpu macro that is not called in all states
        macro_rules! checkCPU {
            () => {
                if info.cpu > HIGH_CPU {
                    self.dizzyness += DIZZY_QUICK;
                } else if self.dizzyness > 0{
                    self.dizzyness -= DIZZY_SLOW;
                }
            };
        }

        //Move to target
        if target_dist > self.range {
            self.pos.0 += ((target_dx as f32 / target_dist as f32) * self.speed as f32) as i32;
            self.pos.1 += ((target_dy as f32 / target_dist as f32) * self.speed as f32) as i32;
        }
        
        match self.state {
            States::sleep => {
                if is_day_time(info.time.hours) {
                    self.state = States::idle
                } else if info.battery < LOW_BATT_LVL { 
                    self.state = States::feint;
                }
            },
            States::idle => {
                self.boredom += 1;
                if info.battery < LOW_BATT_LVL { 
                    self.state = States::feint;
                    self.boredom = 0;
                } else if !is_day_time(info.time.hours) { 
                    self.state = States::sleep;
                    self.boredom = 0;
                } else if self.dizzyness > DIZZY_THRESH_HIGH {
                    self.state = States::dizzy;
                }else if mouse_dist > self.range && mouse_dist < self.long_range {
                    self.target = info.mouse_pos;
                    self.state = States::chase;
                } else if self.boredom >= self.bored_thresh{
                    self.state = States::wander;
                    //TODO: set to random values
                    self.target = (sudo_rand(500) as i32,sudo_rand(500) as i32);
                    self.bored_thresh = (sudo_rand(20) + 10) as i32;
                }else if info.mouse_clicked {
                    if self.boredom >= 5{
                        self.boredom -= 5;
                    } else {
                        self.boredom = 0;
                    }
                    //do a trick
                }
                checkCPU!();
            }, 
            States::chase => {
                self.target = info.mouse_pos;
                if info.battery < LOW_BATT_LVL || self.dizzyness > DIZZY_THRESH_HIGH { 
                    self.state = States::slow_chase;
                } else if target_dist <= self.range {
                    self.state = States::idle;
                } else if info.mouse_clicked {
                    // pounce
                }
                checkCPU!();
            }, 
            States::slow_chase => {
                self.target = info.mouse_pos;
                if self.speed > 0 {
                    self.speed -= 2;
                } else if info.battery < LOW_BATT_LVL {
                    self.target = self.pos;
                    self.state = States::feint;
                    self.speed = 15;
                } else if self.dizzyness > DIZZY_THRESH_HIGH {
                    self.target = self.pos;
                    self.state = States::dizzy;
                    self.speed = 15;
                } else {
                    self.target = self.pos;
                    self.speed = 15;
                    self.state = States::idle;
                }

                checkCPU!();
            }, 
            States::feint => {
                if info.battery > LOW_BATT_LVL && self.dizzyness < DIZZY_THRESH_LOW {
                    self.state = States::sleep;
                }
                if self.dizzyness >= DIZZY_THRESH_LOW && info.cpu < HIGH_CPU  {
                    self.dizzyness -= DIZZY_QUICK;
                } 
            },
            States::wander => {
                if !is_day_time(info.time.hours){ // it's bedtime
                    self.target = self.pos; // stop moving
                    self.state = States::sleep; // go to sleep
                    self.boredom = 0;
                } else if info.battery < LOW_BATT_LVL { 
                    self.target = self.pos;
                    self.state = States::feint;
                } else if self.dizzyness > DIZZY_THRESH_HIGH{
                    self.target = self.pos;
                    self.state = States::dizzy;
                }else if mouse_dist <= self.range && self.boredom <= 0 {
                    self.target = info.mouse_pos;
                    self.state = States::chase; // chase mouse if it goes by and not bored
                } else if target_dist < self.range{
                    if self.boredom > 0 {
                        //TODO: set random point when I learn how
                        self.target = (sudo_rand(500) as i32,sudo_rand(500) as i32)
                    } else {
                        self.state = States::idle;
                    }
                }
                if self.boredom > 0{
                    self.boredom -= 1;
                }
                checkCPU!();
            },
            States::dizzy => {
                if !is_day_time(info.time.hours) {
                    self.state = States::sleep;
                    self.dizzyness = 0;
                }else if info.battery < LOW_BATT_LVL{
                    self.state = States::feint;
                }else if self.dizzyness <= DIZZY_THRESH_LOW {
                    self.state = States::idle;
                    self.dizzyness = 0;
                }else if self.dizzyness > TOO_DIZZY_THRESH {
                    self.state = States::feint;
                }
                
                if info.cpu < HIGH_CPU {
                    self.dizzyness -= DIZZY_SLOW;
                } else {
                    self.dizzyness += DIZZY_SLOW;
                }
            },
        }
        self.print();
    }

    fn print(&self) {
        println!(
        "State: {}, pos: ({},{}), range: {}, speed: {} target: ({},{}), boredom: {}, 
        bored_tresh: {}, dizzyness: {} --------------------------------------------",
            get_state_str(&self.state), self.pos.0, self.pos.1, self.range, self.speed,
            self.target.0, self.target.1, self.boredom, self.bored_thresh, self.dizzyness
        );
    }
}

// Finds the closest integer to the square root of the argument 
// using binary math for a speedy sqrt().
// Pulled this function from Wikipedia and translated from C to Rust: 
// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)
fn isqrt(mut num:u32) -> u32 {
    let mut res:u32 = 0;
    let mut bit:u32 = 1 << 30; // The second-to-top bit is set.
                           // Same as ((unsigned) INT32_MAX + 1) / 2.

    // My own code to quit early if it's 0
    if num == 0 {
        return 0;
    }

    // "bit" starts at the highest power of four that is <= the argument.
    while bit > num {
        bit >>= 2; // Same as bit /= 4
    }

    while bit != 0 {
        if num >= res + bit {
            num -= res + bit;
            res = (res >> 1) + bit;
        } else {
            res >>= 1; // Same as res /= 2
        }
        bit >>= 2; 
    }
    return res;
}