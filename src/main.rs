extern crate sensehat;
extern crate rusqlite;
extern crate chrono;

use sensehat::*;
use std::{thread, time};
use rusqlite::Connection;
use chrono::prelude::*;

struct SenseData {
    humidity: f64,
    temperature: f64,
    p_temperature: f64, 
    h_temperature: f64, 
    pressure: f64
}

fn main() {

    let wait = time::Duration::from_millis(10000);
    let mut sense = SenseHat::new().unwrap();

    sense.init();
    
    //~ let conn = Connection::open_in_memory().unwrap();
    let conn = Connection::open("SenseData.sql").unwrap();
    
    

    conn.execute("CREATE TABLE sense_data (
                  time_log TIMESTAMP DEFAULT (DATETIME('now','localtime')),
                  humidity  FLOAT,
                  temperature   FLOAT,
                  p_temperature FLOAT,
                  h_temperature FLOAT,
                  pressure  FLOAT
                  )", &[]).unwrap();
    
    loop {
        let local: DateTime<Local> = Local::now(); 
        println!("{:?}", local);
        let res = SenseData {
            humidity: sense.get_humidity() as f64,
            temperature: sense.get_temperature() as f64,
            p_temperature: sense.get_temperature_from_pressure() as f64,
            h_temperature: sense.get_temperature_from_humidity() as f64,
            pressure: sense.get_pressure() as f64
        };
        conn.execute("INSERT INTO sense_data (
                time_log,
                humidity, 
                temperature, 
                p_temperature, 
                h_temperature, 
                pressure)
            VALUES (DATETIME('now','localtime'), ?1, ?2, ?3, ?4, ?5)",
                 &[&res.humidity, &res.temperature, &res.p_temperature, 
                 &res.h_temperature, &res.pressure]).unwrap();
        thread::sleep(wait);
    }
}
