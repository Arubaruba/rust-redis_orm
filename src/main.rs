extern crate rustc_serialize;
extern crate redis;

use rustc_serialize::{Encodable, Decodable};
use std::convert::Into;

trait Database {
    fn save<T>(&self, record: &T) where T: Indexable;
    
	/// Create an index for a field in the database
	fn index_field<T>(&self, name: &'static str, value: &T) where T: Encodable;

	/// Numeric fields are indexed differently
	fn index_num_field<T>(&self, name: &'static str, value: &T) where T: Encodable + Into<f64>;
}

impl Database for redis::Connection {
    fn save<T>(&self, record: &T) where T: Indexable {
        
    }   

	/// Create an index for a field in the database
	fn index_field<T>(&self, name: &'static str, value: &T) where T: Encodable {
	    
	}

	/// Numeric fields are indexed differently
	fn index_num_field<T>(&self, name: &'static str, value: &T) where T: Encodable + Into<f64> {
	    
	}
}

trait Indexable where Self: Encodable + Decodable + PartialEq {
	/// Creates indices for the record in the database that
	/// can later be used to search for it
    fn index<D>(&self, db: &D) where D: Database;
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct User {
    name: String,
    age: u16
}

impl Indexable for User {
    fn index<D>(&self, db: &D) where D: Database {
        db.index_field("name", &self.name);
        db.index_num_field("age", &self.age);
    }
}


fn main() {
    let a = "asdf";
    let b = 0b10100;

	println!("a: {}", 1);
}




/*
#![feature(custom_attribute)]
extern crate bincode;
use bincode::rustc_serialize::{encode, decode};

mod models;

use redis::Commands;
use redis::RedisResult;

struct ShadowQuery {
	filterFn: Option<usize>,
	sortFn: Option<usize>,
	sortReverse: bool,
	skip: Option<usize>,
	take: Option<usize>,
}

//enum ShadowOp{
//    Sort(ShadowAttr),
//    FilterValue(ShadowAttr, Shado)
//}

//#[derive(ShadowTable)]
struct Course {
//	#[shadow_index]
	name: String,
    teachers: Vec<User>,
    students: Vec<User>,
}


#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct User {
//	#[shadow_index]
    name: String,
//    courses: Vec<Course>,
	logins: HyperLogLog
}

struct Database {
    a: i32
}

impl Database {
    fn get<T>() {
        
    }
}

impl User {
    pub fn new() -> User {
        User { name: String::new() }
    }
}

// Will be autogenerated
struct UserIndices {
    name: Index<String>,
    age: NumericIndex<i32>,

    logins: Vec<Date>
}

struct Index<T: Encodable + Decodable>  {
    value: T

	// TODO implement PartialEq
}

// We need to make sure that the type can be represented as a float because
// redis only accepts floats as sorted set rankings
struct NumericIndex<T: Encodable + Decodable + std::convert::Into<f64>> {
    value: T

	// TODO implement PartialEq, Less than, Greater than
}


/**

MISSING FEATURES: Sort string fields, relations
COOL TRICKS TO USE: bitop, hyperloglogs 

1) SAVING DATA:

MULTI
HSET user:1 name Andy
HSET user:1 age 20

HSET user:2 name Jim
HSET user:2 age 24


2) INDEXING:

SADD user:name=Andy 1
SADD user:name=Jim 2

ZADD user:age 20 1
ZADD user:age 20 2

3) RETRIEVING

// Want users with name Andy younger than 22 sorted by age

MULTI
ZUNIONSTORE tmp 1 user:age
// Remove users 22 and older
ZREMRANGEBYSCORE tmp [22 +inf
ZINTERSTORE tmp 2 tmp user:name=Andy AGGREGATE MAX
SORT tmp BY user:*->age GET # GET user:*->name GET user:*->age
DEL tmp
EXEC
*/

fn main() {
    println!("got user: {}", get_user().unwrap().name);

	// Create a user
	// 

	// Get all users with more than 10 logins per day



//    // Queries
//    let users = User::load_from(&db)
//    				// TODO - change to macro
//    				// will be indexed 
//    				// SAMPLE 
//    				/*
//    				MULTI 
//    				
//    				SUNION User:name=Andy User:age:32
//
//    				*/
//                    .filter(|user| user.name == "Andy" && user.age > 32 && user.courses.len() > 32)
////                    .map(|user| (user.name, user.age, user.courses.len()))
//                    .sort_by_key(|user| user.courses.len())
//                    .reverse()
//                    .skip(60).take(30);
//	
//
//    for user in users {
//        // Method 1
//        user.courses.push(Course { name: "ECON 101" });
//
//        // Method 2
//        let course = Course::save_to(&db, Course::new());
//
//        println!("user.name = {}", user.name);
//    }
}


fn get_user() -> RedisResult<User> {
    let client = try!(redis::Client::open("redis://127.0.0.1"));
    let db = try!(client.get_connection());

    try!(db.hset("user",
                 "1",
                 encode(&User { name: "Andy".to_string() },
                        bincode::SizeLimit::Infinite)
                     .unwrap()));

    let encoded_user: Vec<u8> = try!(db.hget("user", "1"));

    Ok(decode(&encoded_user).unwrap())
}*/