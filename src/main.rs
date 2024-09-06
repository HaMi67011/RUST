//struct First letter will be capital
//simple 
struct Student{
    //name :&str, error of lifetime
    name : String,
    id : i16,
}

impl Student {
    fn new (name:String,id:i16)-> Self{
        Self {name,id}
    }
    fn display_data(&self){
        println!("Name {} and id {}",self.name,self.id)
    }
    
}

//tuple struct
#[derive(Debug)]
struct Color(i32,f32);

//empty struct
#[derive(Debug)]
struct Empty;

//struct methods
#[derive(Debug)]
struct Circle{
    radius:f32,
}
impl Circle {
    //self for intance and Self for  type
    fn area(&self) ->f32{
        std::f32::consts::PI *self.radius*self.radius
    }
    
}

//struct with assoicated function
#[derive(Debug)]
struct Person{
    name : String,
    age : u32,
}
impl Person {
    fn new (name:&str,age:u32) -> Self{
        Person{
            name:name.to_string(),
            age,
        }
    }
}

//struct with lifetime 
#[derive(Debug)]
struct Wrapper<'a> {
    data: &'a str,
}

//struct Generic  
#[derive(Debug)]
struct  GenericStrcut<T>{
    x:T,
}

//nested struct 
#[derive(Debug)]
struct Dept {
    name: String,
}

//implement of dept
impl Dept {
    fn new(name:String) -> Self{
        Dept {name}
    }
    
}

struct Employee{
    name:String,
    age:u32,
    dept : Dept,
}

//implement of method of employee
impl Employee {
    fn new(name:String,age:u32,dept:Dept) -> Self{
        Employee {
            name,
            age,
            dept
        }
    }

    fn display_data(&self){
        println!("name is {} , age is {} and dept is {}",self.name,self.age,self.dept.name);
    }

    fn edit_age(&mut self,age:u32){
        self.age = age
    }
}

//mod
//simple
mod math{
    //private function so cannot access
    fn add(x:i32,y:i32) -> i32{
        x + y
    }

    pub fn sub(x:i32,y:i32)->i32{
        x-y
    }
}

//nested
//simple
mod shape{

    //private module so cannot access 
    mod circle{
        fn display(){
            println!("HELLO FROM CIRCLE");
        }
    }

    //module is public but function is private
    pub mod triangle{
        fn display(){
            println!("HELLO FROM TRIANGLE");
        }
    }

    //module is private but function is public
    mod rectangle{
        pub fn display(){
            println!("HELLO FROM RECTANGLE");
        }
    }

    //both module and function is public
    pub mod square{
        pub fn display(){
            println!("HELLO FROM SQUARE");
        }
    }

}


mod university {
    //public mod but struct is private but constructor is public
    pub mod student{
        struct Student{
            name : String,
            age:i32,
        }

        impl Student {
            pub fn new (name:String,age:i32)-> Self{
                Self {
                    name,
                    age
                }
            }
        }    
    }

    //public module and struct but constructor is private
    pub mod teacher{
        pub struct Teacher{
            name:String,
            age:i32,
        }

        impl Teacher {
            fn new (name:String,age:i32)-> Self{
                Self {
                    name,
                    age
                }
            }
        }
    }

    //public module and struct and it's entry is also public
    pub mod Dept{
        pub struct Depts{
            pub name:String,
        }
    }

    //public module and struct with public function but private datatypes
    pub mod office{
        pub struct Offices{
            name:String,
            addr : String,
        }

        impl Offices{
            pub fn new(name:String,addr:String) -> Self{
                Self { name, addr }
            }

            pub fn display_data(&self){
                println!("name {} and it's address is {}",self.name,self.addr);
            }
        }
    }

}

//exporting
//use university::student::Student; cannot access as it's a private 
//use university::teacher::Teacher;
//use university::Dept::dept;

//import from other files
mod cal;
//use cal::cal::advanced::square as sqrt; without dividing module
//use cal::square as sqrt; //with divide module


//testing
#[cfg(test)]
mod tests{
    use self::university::teacher;

    use super::*;
    
    #[test]
    //simple cal basic add function testing using assert equal (should equal to given value of test)
    fn test_add_equal(){
        assert_eq!(cal::basic::add(3,5),8);
    }


    #[test]
    //using assert not equal (value should not equal to given value of test)
    fn test_add_unequal(){
        assert_ne!(cal::basic::add(3,5),7);
    }

    #[test]
    //using assert that check for bools only
    fn test_add_assert(){
        assert!(cal::basic::check_value(40));
    }

    #[test]
    #[should_panic]
    //for checking it's handling an unexpected value or not
    fn test_panic(){
        assert!(cal::basic::check_value(30));
    }


}


//generics
//function
use std::fmt::Debug; // Import Debug trait
fn generic_print<T:Debug> (value:T) -> T{
    println!("{:?}",value);
    value
}

//struct
#[derive(Debug)]
struct Pair<T>{
    x: T,
    y:T,
}

impl<T> Pair<T> {
    fn new(x:T,y:T)->Pair<T>{
        Pair{x:x,y:y}
        }
}

//enum
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
    
}


//trait

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArt<T>{
    Name:String,
    content:T ,   
}

impl <T:Summary> Summary for NewsArt<T>{
 fn summarize(&self) -> String {
        format!("{} - {}", self.Name, self.content.summarize())
    }
}

struct Tweet{
    Name:String,
    content:String,    
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{} - {}", self.Name, self.content)
    }
}

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

struct Circles {
    radius: f64,
}

impl Shape for Circles {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn print_shape_info(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}


//trait bond

trait Greeting {
    fn greet(&self) -> String {
        "Hello from Rust!".to_string()
    }
}

fn print_greeting1<T:Greeting >(input: &T) {// Fix using trait bound
    println!("{}", input.greet());
}


fn print_greeting2(input: &impl Greeting) {// Fix using impl trait syntax 
    println!("{}", input.greet());
}


fn print_greeting3<T>(input: &T) 
// Fix by using the where clause
where 
T : Greeting
{
    println!("{}", input.greet());
}


struct Greeter;

impl Greeting for Greeter {}

//SUPER TRAIT

trait Size {
    fn compute_size(&self) -> u16;
}

trait Printable {
    fn size_to_str(&self) -> String;
}

trait Comparable:Size+Printable {
    fn print_greater(a: &Self, b: &Self) { 
    // Please note that Self on the line above means, that type which will be implementing the trait 

        let item1 = a.compute_size();
        let item2 = b.compute_size();
        if item1 > item2 {
            println!("{} is greater than {}", a.size_to_str(), b.size_to_str());
        } else if item2 > item1 {
            println!("{} is greater than {}", b.size_to_str(), a.size_to_str());
        } else {
            println!("Both sizes are {}", a.size_to_str());
        }
    }
}

struct Book {
    page: u16,
}

impl Size for Book {
    fn compute_size(&self) -> u16 {
        self.page
    }
}

impl Printable for Book {
    fn size_to_str(&self) -> String {
        format!("Book having {} pages", self.page)
    }
}

impl Comparable for Book {}

//trait objects

trait Animal {
    fn sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) {
        println!("Meow!");
    }
}

fn make_sound(animal: &dyn Animal) {
    animal.sound();
}

//associated type

trait Container {
    type Item;

    fn get_item(&self) -> Self::Item;
}

// Implement `Container` trait for `StringContainer`
struct StringContainer {
    item: String,
}

impl Container for StringContainer {
    type Item = String; // Specify the associated type

    fn get_item(&self) -> Self::Item {
        self.item.clone() // Return the associated type
    }
}

// Implement `Container` trait for `i32Container`
struct IntContainer {
    item: i32,
}

impl Container for IntContainer {
    type Item = i32; // Specify the associated type

    fn get_item(&self) -> Self::Item {
        self.item // Return the associated type
    }
}

//closure function

fn apply_function<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

//function pointer

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn square(x: u32) -> u32 {
    x * x
}

fn sum_of_squares(num: u32, sq: fn(u32) -> u32, add: fn(u32, u32) -> u32) -> u32 { 
    let mut result = 0;
    for i in 1..=num {
        result = add(result, sq(i));
    }
    result
}


//lifetime
//functions
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//struct
struct Important<'a> {
    part: &'a str,
}

impl<'a> Important<'a> {
    fn new(part: &'a str) -> Self {
        Important { part }
    }
}

//trait
trait Printer {
    fn print<'a>(&self, message: &'a str);
}

//BOX
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//RC import 
use List::{Cons, Nil};
use std::rc::Rc;
    
//Refcell import
use std::cell::RefCell;

//RC and REFCELL 
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

//singly linked list

#[derive(Debug)]
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Nodes {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Nodes>>;

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn add_to_head(&mut self, element: i32) {
        let new_nodes = Some(Box::new(Nodes {
            element,
            next: self.head.take(),
        }));
        self.head = new_nodes;
    }

    fn add_to_tail(&mut self, element: i32) {
        let new_nodes = Some(Box::new(Nodes {
            element,
            next: None,
        }));
        match self.head {
            None => {
                self.head = new_nodes;
            }
            Some(ref mut current) => {
                let mut tail = current;
                while let Some(ref mut next) = tail.next {
                    tail = next;
                }
                tail.next = new_nodes;
            }
        }
    }

    fn add_at_index(&mut self, index: usize, element: i32) {
        if index == 0 {
            self.add_to_head(element);
            return;
        }
        let mut current = &mut self.head;
        for _ in 0..index-1 {
            match current {
                None => return, // Index out of bounds
                Some(ref mut nodes) => current = &mut nodes.next,
            }
        }
        let new_nodes = Some(Box::new(Nodes {
            element,
            next: current.take().map(|nodes| nodes),
        }));
        *current = new_nodes;
    }

    fn remove_from_head(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(nodes) => {
                self.head = nodes.next;
                Some(nodes.element)
            }
            None => None,
        }
    }

    fn remove_from_tail(&mut self) -> Option<i32>  {
        let mut list_traversal = &self.head;
        while let Some(ref nodes) = list_traversal {
            if nodes.next.is_none(){
                let result = nodes.element;
                println!("{:?}",result);
                return Some(result)
                
            }
            list_traversal = &nodes.next;
        }
        None
    }

    fn remove_at_index(&mut self, index: usize) -> Option<i32> {
        if index == 0 {
            return self.remove_from_head();
        }
        let mut current = &mut self.head;
        for _ in 0..index-1 {
            match current {
                None => return None, // Index out of bounds
                Some(ref mut nodes) => current = &mut nodes.next,
            }
        }
        match current {
            None => None,
            Some(ref mut nodes) => match nodes.next.take() {
                Some(mut next_nodes) => {
                    let removed_element = next_nodes.element;
                    nodes.next = next_nodes.next.take();
                    Some(removed_element)
                }
                None => None,
            },
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while let Some(ref nodes) = list_traversal {
            print!("{} -> ", nodes.element);
            list_traversal = &nodes.next;
        }
        println!("None");
    }
}


// Define a macro that takes multiple arguments and prints them
macro_rules! print_values {
    ($($val:expr),*) => {
        $(
            println!("{}", $val);
        )*
    };
}

// Import the necessary crates
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

// Define the procedural macro
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap();
    
    // Build the output tokens
    impl_hello_macro(&ast)
}

// Generate the implementation of the HelloMacro trait
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Define the HelloMacro trait in the same or another crate
pub trait HelloMacro {
    fn hello_macro();
}

// In main.rs or another file
#[derive(HelloMacro)]
struct MyStruct;

//Propagating Errors with Result

use std::fs::File;
use std::io::{self, Read};

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;  // Open the file
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // Read the file content
    Ok(content)
}

//errors with option 
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn calculate() -> Option<f64> {
    let a = divide(10.0, 2.0)?;  // This will succeed
    let b = divide(a, 0.0)?;  // This will fail and return None
    Some(b)
}

//hashset
use std::collections::HashSet;

//trie tree

use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end_of_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }
}

//file handling 
use std::fs::File;
use std::io::{self, Read}; //for reading
use std::io::{self, Write}; //for writing and appending
use std::io::{self, BufRead, BufReader}; //for reading line by line


//todo mircro

macro_rules! todo_macro {
    ($( $task_name:ident => $task_description:expr, $completed:expr );* $(;)?) => {
        $(
            struct $task_name {
                description: &'static str,
                completed: bool,
            }

            impl $task_name {
                fn new() -> Self {
                    $task_name {
                        description: $task_description,
                        completed: $completed,
                    }
                }

                fn print_status(&self) {
                    println!("Task: {}, Status: {}", self.description, if self.completed { "Completed" } else { "Incomplete" });
                }
            }
        )*
    };
}

todo_macro! {
    Task1 => "Buy groceries", false;
    Task2 => "Clean the house", true;
    Task3 => "Finish homework", false;
}


//threads
//scope threads
use crossbeam::thread;

//thread parking    
use std::thread;
use std::time::Duration;

//async await
use std::time::Duration;
use tokio::time::sleep;

async fn do_something() {
    println!("Doing something...");
    sleep(Duration::from_secs(2)).await; // Simulate asynchronous work
    println!("Done doing something!");
}



fn main(){
    
    // //declaring

    // //integers 
    // //signed  integers: i32, i64, isize (platform dependent)
    // let x: i16 = 10;
    // let y: i32 = 100;
    // let z: i64 = -1000;
    // println!("x is {} , y is {}, and z is {}", x,y,z);

    // //unsign integers
    // let  a: u8 = 1;
    // let  b: u16 = 10;
    // let  c: u32 = 100;
    // //let  d: u64 = -100; error because of negative value
    // println!{"a is {},b is {},c is {}",a,b,c};

    // //float 
    // let d : f32 = 10.9;
    // let e : f64 = -199.5;
    // println!("d is {},e is {}",d,e);

    // //string
    // let name :&str = "Hamza";
    // let fullname :String =  String::from("Muhammad Hamza");
    // let char_name :char = 'H';
    // println!("My Name Is {} and Fullname is {} and char is {}",name,fullname ,char_name);

    // //bool
    // let bool_true = true;
    // let bool_false:bool = false;
    // println!("ture bool is {} and false bool is {}",bool_true,bool_false);

    // //array 
    // let number_array :[i32;5] = [1,2,3,4,5];
    // let vowels :[char;5] = ['a','e','i','o','u'];
    // let vec_numbers : Vec<i32> = vec![1,2,3,4,5];
    // println!("Array elements are {:?},vowels are{:?} and vec are {:?}",number_array,vowels,vec_numbers); // ? for numbers in array

    // //tuples
    // let number_tuples : (i32,&str,f32,char) = (10,"HAMZA",10.9,'H');
    // println!("tuple is {:?}",number_tuples);

    // //enum
    // #[derive(Debug)]
    // enum Direction{
    //     Left,
    //     Right,
    //     Up,
    //     Down,
    // }
    // println!("enum is {:?}",Direction::Up);

    // //structs
    // //simples
    // let st1 = Student{name : String::from("Hamza"),id :78};
    // println!("Struct simple student {:?},{:?}",st1.name,st1.id);
    
    // //tuple struct
    // let color  = Color (3,6.5);
    // println!("tuple struct color is {:?}",color);
    

    // //empty stuct
    // let empty :Empty = Empty;
    // println!("empty struct is {:?}",empty);


    // //struct method 
    // let circle :Circle = Circle {radius:50.2};
    // let area :f32 = circle.area();
    // println!("The area of the circle is {} ",area);

    // //struct with assoicated function
    // let person : Person = Person::new("hamza", 23);
    // println!("person name {} and age {}",person.name,person.age);

    // //struct with lifetime 
    // let wrapper:Wrapper =  Wrapper{data : "Hamza"};
    // println!("the data in wrapper is {:?}",wrapper.data);

    // //Struct Generic 
    // let c1:GenericStrcut<i32> = GenericStrcut{x:10};
    // println!("Generic Struct value is {:?}",c1.x);


    // //functions

    // //user input from number
    // println!("Enter the Number");
    // let mut number = String::new();
    // std::io::stdin().read_line(&mut number)
    //      .expect("Failed to read line");
    // let number : usize = number.trim().parse()
    //     .expect("please enter a valid");




    // //add two unsign integer
    // fn add_two_unsign_int(a:u32 , b: u32) -> u32 {
    //     a+b
    // }
    // println!("function add_two_unsign_int {}",add_two_unsign_int(c, c));

    // //add two sign integer
    // fn add_two_sign_int(a: i32, b: i32) -> i32 {
    //     a+b
    // }
    // println!("function add_two_sign_int {}",add_two_sign_int(y,y));

    // //add two float 
    // fn add_two_float(a:f32,b:f64) -> f64{
    //     let a = a as f64;
    //     a+b
    // }
    // println!("function add two float{}",add_two_float(d,e));

    // //string function
    // //using of as
    // fn lenth_of_str_using_as(a:&str) -> f32{
    //     a.len() as f32
    // }
    // println!("Lenght of &str {}",lenth_of_str_using_as(name));

    // //using of usize
    // fn lenth_of_str_using_usize(a:&str) -> usize {
    //     a.len()
    // }
    // println!("Lenght of &str {}",lenth_of_str_using_usize(name));

    // //indexing 
    // fn indexing_str(a:&str,b:usize) -> char {
    //     a.chars().nth(b).unwrap_or('\0')
    // }
    // println!("Indexing Str using function {}",indexing_str(name,3)) ;

    // //without function
    // let indexing = &name[number-1..number];

    // println!("indexing {} from {} to {}",indexing,number-1,number);

    // //slicing with function
    // fn slicing_with_func(a:&str , b:usize,c:usize) -> &str{
    //     &a[b..c]
    // }
    // println!("Slicing with function {}",slicing_with_func(name,1,3));

    // //string Concatenation
    // println!("ENTER THE STRING 1");
    // let mut string_1 = String::new();
    // std::io::stdin().read_line(&mut string_1)
    //     .expect("Failed to read line");

    // string_1 = string_1.trim().to_string();

    // println!("String 1 is {}",string_1);

    // println!("ENTER THE STRING 2");
    // let mut string_2 = String::new();
    // std::io::stdin().read_line(&mut string_2)
    //     .expect("Failed to read line");

    // string_2 = string_2.trim().to_string();

    // println!("String 2 is {}",string_1);

    // //function concatention
    // fn concatenation(a:&String,b:&String) -> String {
    //     format!("{}{}",a,b)
    // }
    // println!("string {} and {} and their concatention is {} ", &string_1,&string_2,concatenation(&string_1,&string_2)); //passing reference becaue of ownership


    // //without function
    // let combine_without_function = string_1+&string_2;
    // println!("combined without function {}",combine_without_function);

    // //whitespace and split
    // //without function
    // let whitespace_string = "hello world    ";
    // let word:Vec<&str> = whitespace_string.split_whitespace().collect();
    // println!("String without whitespaces {:?}",word);

    // //with function
    // fn whitespace_string_function(a:&str) -> Vec<&str>{
    //     a.split_whitespace().collect()
    // }

    // println!("white space function {:?}",whitespace_string_function(whitespace_string));

    // //contain start and end of string matching
    // let contain = whitespace_string.contains("o");
    // let start_with = whitespace_string.starts_with("H");
    // let end_with = whitespace_string.ends_with(" ");
    // println!("Contains {} , start_with {} and end with {}",contain,start_with,end_with);


    // //array
    // let arr1:[i32;10] = [1,3,4,5,10,2,6,8,7,9];

    // //length function 
    // fn lenght_array(array:&[i32;10]) -> i32 {
    //     if array.len() < 0 {
    //         println!("ERROR OF LENGTH IS LESS THEN ZERO");
    //         return -1;
    //     }else {
    //         //array.len() as i32
    //         let mut count = 0;
    //         for &num in array.iter(){
    //             count += 1;
    //         }
    //         return count;
    //     }
    // }

    // println!("Length of array using length {}",lenght_array(&arr1));

    // //sort function 
    // fn sort_array(array : &[i32;10]) -> Result<[i32;10],&str> {
    //     if lenght_array(&array) > 0{
    //          let mut arr = *array;
    //         // arr.sort();
    //         // Ok(arr)

    //         for i in 0..arr.len()-1 {
    //             for j in 0..arr.len()-1-i{
    //                 if arr[j] > arr[j+1] {
    //                     let temp = arr[j];
    //                     arr[j] = arr[j+1];
    //                     arr[j+1] = temp;
    //                 }
    //             }
    //         }

    //         Ok(arr)

    //     }else {
    //         Err("ERROR")
    //     }
    // }

    // println!("Sort array {:?}",sort_array(&arr1).unwrap());


    // //array contains a number
    // fn array_contains(array : &[i32;10],b:i32) -> bool {
    //     //array.contains(&3)
    //     for i in array.iter(){
    //         if *i == b{
    //             return true;
    //         }
    //     }
    //     false
    // }

    // println!("array contain 3 {}",array_contains(&arr1,3));

    // //searching for array position
    // fn array_position(array: &[i32; 10],b:i32) -> i32 {
    //     //array.iter().position(|&x| x==3)
    //     let mut position = 0;
    //     for i in array.iter(){
    //         if *i == b{
    //             return position;
    //         }
    //         position+=1;
    //     }
    //     position
    // }

    // println!("searching position (index) of 3 in array {}",array_position(&arr1,3));

    // //filtering elements
    // fn array_filtering(array: &[i32; 10],b:i32) -> [i32;10] {
    //     //let filtered_arr: Vec<_> = arr.iter().filter(|&&x| x > 2).collect();
    //     let mut filter_array  =  [0; 10]; //for initalizing array from 0 to 10
    //     let mut index = 0;

    //     for i in 0..array.len(){
    //         if array[i] > b{
    //             filter_array[index] += array[i];
    //             index+=1;
    //         }
    //     }
    //     filter_array
    // }

    // println!("array filtering of greater then 5 {:?}",array_filtering(&arr1,5));

    // //vec 
    // //declare a vector
    // let mut vec : Vec<i32> = Vec::new();
    // println!("vec {:?}",vec);

    // let mut vec1 : Vec<i32> = vec![1,2,3,4,5];
    // println!("vec1 {:?}",vec1);

    // //inserting elements
    // fn vec_insert (vec :&mut Vec<i32>,b:i32) {
    //     vec.push(b)
    // }   

    // vec_insert(&mut vec, 3);
    // println!("VEC AFTER INSERTING {:?}",vec);

    // //popping elements 
    // fn vec_pop(vec :&mut Vec<i32> ) -> i32{
    //     let element = vec.pop().expect("cannot pop empty vec"); //give error of option
    //     element
    // }
    // println!("Popping element {} and vec {:?}",vec_pop(&mut vec1),vec1);

    // //filter 
    // let mut filtered_vec: Vec<_> = vec.into_iter().filter(|&x| x > 2).collect();

    // //resize
    // filtered_vec.resize(10, 0); // Resize the vector to a specific length, padding with zeros

    // //searching and sort as array

    // let index_of_3 = filtered_vec.iter().position(|&x| x == 3);
    // filtered_vec.sort(); // Sort the elements in ascending order
    // let is_empty = filtered_vec.is_empty(); // Check if the vector is empty
    // filtered_vec.clear(); // Remove all elements from the vector

    // //enums
    // //with associated data
    // enum Message{
    //     Quit,
    //     Move {x:i32,y:i32},
    //     Write(String),
    //     ChangeColor(i32,i32,i32),
    // }

    // //method of enum
    // impl Message {
    //     fn processing_msg(&self){
    //         match self{
    //             Message::Move { x, y } => println!("Move to {x} and {y}"),
    //             Message::ChangeColor(r, g, b) =>  println!("Color changed to {r},{g} and {b}"),
    //             Message::Quit => println!("QUITING.."),
    //             Message::Write(text) => println!("Write {text}"),

    //         }
    //     }

    //     //error of returning a enum with multiple feilds
    //     // fn new() -> Self {
    //     //     Message::Move { x: (0), y: (0) };
    //     //     Message::ChangeColor((0), (0), (0));
    //     //     Message::Write("HELLO".to_string());
    //     //     return Message;
    //     // }

    //     fn new () -> Self {
    //         Message::Move { x: 0, y: 0 }
    //     }

    //     fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    //         for (index, &element) in arr.iter().enumerate() {
    //             if element == target {
    //                 return Some(index);
    //             }
    //         }
    //         None
    //     }
    //     fn find_elementenum(arr: &[i32], target: i32) -> Option<usize> {
    //         for (index, &element) in arr.iter().enumerate() {
    //             if element == target {
    //                 return Some(index);
    //             }
    //         }
    //         None
    //     }
                
    // }

    // let message = Message::Quit;
    // message.processing_msg();

    // let message = Message::Move { x: 10, y: 20 };
    // message.processing_msg();

    // let message = Message::Write(String::from("Hello"));
    // message.processing_msg();

    // let message = Message::ChangeColor(255, 0, 128);
    // message.processing_msg();

    // let msg = Message::new();

    // let numbers = [10, 20, 30, 40, 50];
    // let target = 30;

    // match Message::find_elementenum(&numbers, target) {
    //     Some(index) => println!("Found {} at index {}", target, index),
    //     None => println!("{} not found in the array", target),
    // }

    // let std :Student = Student::new("Hamza".to_string(), 1);
    // std.display_data();

    // //mut instance 
    // let mut std2 = Student::new("ali".to_string(), 3);
    // std2.display_data();

    // std2.id+=1;


    // println!("mutable std2 instance ");
    // std2.display_data();

    // //nested struct
    // let dept_1 = Dept::new("CS".to_string());
    // let emp :Employee = Employee::new(String::from("Hamza"), 23, dept_1);
    // emp.display_data();

    // //nested struct passing other struct in new
    // let mut emp2 : Employee = Employee::new("hamza".to_string(), 23, Dept::new("CS".to_string()));
    // println!("nested struct passing other struct in new");
    // emp2.display_data();

    // //calling edit funciton
    // emp2.edit_age(24);
    // println!("Calling edit function as mut reference ");
    // emp2.display_data();

    // //mods
    // //println!("mod add {:?} is ",math::add(3, 5)); private function so not access
    // println!("mod sub {:?} is ",math::sub(3, 5));

    // //nested mod
    // //shape::circle::display(); cannot as a private module
    // //shape::triangle::display(); cannot access display as its private
    // //shape::rectangle::display(); cannot access due to private module
    // shape::square::display(); //access due to public module and function


    // //let Tech_1 = Teacher::new("Hamza".to_string(),23); struct is public but constructor is private so cannot

    
    // //without use or exporting
    // let dept_mod = university::Dept::dept{
    //     name:String::from("Computer Science"),
    // };

    // println!("Module dept is {}",dept_mod.name);

    // //with exporting
    // let dept_mod_1 = dept{
    //     name:String::from("Computer Science 1"),
    // };

    // println!("Module dept1 is {}",dept_mod_1.name);

    // let off = university::office::Offices::new(String::from("account"), String::from("main building"));
    // off.display_data();

    // println!("imported square form cal.rs { }",sqrt(3));


    // //generic
    // //functions
    // let num=println!("generic function {}",generic_print(1));
    // let string = println!("generic function {} ",generic_print('1'));

    // //structures
    // let x = Pair{x:10, y:2};
    // println!("pair has value of {},{}",x.x,x.y);

    // //enum
    // let enum_val = Option::Some(3);
    // let enum_val = Option::Some(3.0);

    // //method
    // let pair_val = Pair::new(1, 1);

    // //trait

    // let tweet = Tweet {
    //     Name: String::from("user123"),
    //     content: String::from("Hello, Rust!"),
    // };

    // let news_article = NewsArt {
    //     Name: String::from("Rust News"),
    //     content: tweet,
    // };

    // println!("{}", news_article.summarize());

    // let rectangle = Rectangle { width: 5.0, height: 3.0 };
    // let circle = Circles { radius: 2.5 };

    // println!("Rectangle:");
    // print_shape_info(&rectangle);

    // println!("Circle:");
    // print_shape_info(&circle);

    // let greeter_instance = Greeter;

    // print_greeting1(&greeter_instance);
    // print_greeting2(&greeter_instance);
    // print_greeting3(&greeter_instance);

    // let book_1 = Book { page: 50 };
    // let book_2 = Book { page: 450 };
    // Comparable::print_greater(&book_1, &book_2);

    // let dog = Dog;
    // let cat = Cat;

    // make_sound(&dog as &dyn Animal); // Explicit casting to a trait object
    // make_sound(&cat as &dyn Animal); // Explicit casting to a trait object

    // let string_container = StringContainer { item: String::from("Hello") };
    // let int_container = IntContainer { item: 42 };

    // // Call the method using trait object
    // println!("String container item: {}", string_container.get_item());
    // println!("Int container item: {}", int_container.get_item());


    // //closure
    // let add_one = |x| x + 1;

    // println!("Result: {}", add_one(5));

    // let double = |x| x * 2;
    // let result = apply_function(double, 5);

    // println!("Result: {}", result);


    // //Fn: Captures variables by immutable reference, allowing only read access.
    // //FnOnce: Consumes variables it captures, taking ownership of them, and can't be called more than once.
    // //FnMut: Captures variables by mutable reference, allowing both read and write access.

    // //fn
    // let data = vec![1, 2, 3];
    // let closure = |index| println!("{}", data[index]);
    // closure(1);

    // //fnone
    // let data = vec![1, 2, 3];
    // let closure = move |index| println!("{}", data[index]);
    // closure(1);
    // // closure(2); // This line would cause a compilation error because closure has consumed `data`

    // //fnmut
    // let mut data = vec![1, 2, 3];
    // let mut closure = |index| {
    //     data[index] *= 2;
    //     println!("{}", data[index]);
    // };
    // closure(1);
    // closure(2);

    
    // let num = 4;
    // let sum = sum_of_squares(num, square, add);
    // println!("Sum of squares from 1 to {} = {}", num, sum);


    // //functional pointer
    // let func_pointer : fn(u32,u32) -> u32;

    // func_pointer = add;

    // println!("functional pointer add {}",func_pointer(3,4));

    // //box
    // let list = Cons(1, 
    //     Box::new(Cons(2, 
    //         Box::new(Cons(3, 
    //             Box::new(Nil))))));
    
    // println!("list {:?}",list);

    // //RC
    // let shared_value = Rc::new(5);
    // let shared_value_clone = Rc::clone(&shared_value);

    // println!("Value: {}", *shared_value);
    // println!("Reference count: {}", Rc::strong_count(&shared_value));

    // //REFCELL
    // let data = RefCell::new(5);

    // {
    //     let mut val = data.borrow_mut();
    //     *val += 1;
    // }

    // {
    //     let val = data.borrow();
    //     println!("Value: {}", val);
    // }

    // println!("data: {:?}", data);

    // let data = RefCell::new(5);

    // let _val1 = data.borrow_mut();
    // let _val2 = data.borrow_mut(); //error here as we cannot borrow same object twice

    // println!("Val1 {_val1} and Val2 {_val2}");

    // //Rc<Refcel> problem
    // let node1 = Rc::new(RefCell::new(Node {
    //     value: 1,
    //     children: vec![],
    // }));
    
    // let node2 = Rc::new(RefCell::new(Node {
    //     value: 2,
    //     children: vec![],
    // }));
    
    // let node3 = Rc::new(RefCell::new(Node {
    //     value: 3,
    //     children: vec![],
    // }));
    
    // // Modify children of node1
    // node1.borrow_mut().children.push(Rc::clone(&node2));
    // node1.borrow_mut().children.push(Rc::clone(&node3));
    
    // // Modify children of node2
    // //A -> B -> C -> A
    // node2.borrow_mut().children.push(Rc::clone(&node1)); // creating a cycle for demonstration
    
    // // Inspecting the graph structure
    // println!("Node1: {:?}", node1);
    // println!("Node2: {:?}", node2);
    // println!("Node3: {:?}", node3);

    // // Modifying a node's value
    // node1.borrow_mut().value = 10;
    // println!("Modified Node1: {:?}", node1);


    //linked list singly

    let mut list = LinkedList::new();
    list.add_to_head(5);
    list.add_to_head(7);
    list.add_to_tail(10);
    list.add_at_index(1, 15);
    list.add_at_index(0, 20);

    list.print();

    println!("Removed from head: {}", list.remove_from_head().unwrap());
    list.print();

    // println!("Removed from tail: {}", list.remove_from_tail().unwrap());
    // list.print();

    println!("Removed at index 1: {}", list.remove_at_index(1).unwrap());
    list.print();


    // Use the macro
    print_values!(1, 2, 3, 4, 5); // This will print each number on a new line

    MyStruct::hello_macro(); // This will print "Hello, Macro! My name is MyStruct!"

    //capture type
    let x = 10;
    
    // Capture by reference
    let by_ref = || println!("x by reference: {}", x);
    by_ref();
    
    // Capture by mutable reference
    let mut y = 20;
    let mut by_mut_ref = || y += 10;
    by_mut_ref();
    println!("y after mutation: {}", y);
    
    // Capture by value
    let z = 30;
    let by_value = move || println!("z by value: {}", z);
    by_value();
    // println!("z after move: {}", z); // This line would cause a compile error because z has been moved

    //errors with result
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    //errors with option 
    match calculate() {
        Some(result) => println!("Calculation result: {}", result),
        None => println!("Error in calculation"),
    }

    //hashset
    // Create a new, empty HashSet
    let mut fruits = HashSet::new();

    // Add some elements to the HashSet
    fruits.insert("Apple");
    fruits.insert("Banana");
    fruits.insert("Orange");

    // Attempt to add a duplicate element
    fruits.insert("Apple");

    // Check if an element is in the HashSet
    if fruits.contains("Banana") {
        println!("Banana is in the set");
    }

    // Remove an element from the HashSet
    fruits.remove("Orange");

    // Print the elements of the HashSet
    println!("Fruits in the set:");
    for fruit in &fruits {
        println!("{}", fruit);
    }

    // Check the length of the HashSet
    println!("The number of fruits in the set: {}", fruits.len());

    //trie tree
    let mut trie = Trie::new();

    trie.insert("hello");
    trie.insert("hell");
    trie.insert("heaven");
    trie.insert("heavy");

    println!("Search 'hello': {}", trie.search("hello")); // true
    println!("Search 'hell': {}", trie.search("hell"));   // true
    println!("Search 'heaven': {}", trie.search("heaven")); // true
    println!("Search 'heav': {}", trie.search("heav"));   // false

    println!("Starts with 'he': {}", trie.starts_with("he")); // true
    println!("Starts with 'hea': {}", trie.starts_with("hea")); // true
    println!("Starts with 'hez': {}", trie.starts_with("hez")); // false

    //file handling
    // Open the file in read-only mode (returns `Result<File, Error>`)
    let mut file = File::open("example.txt")?;

    // Create a string to store the file content
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Print the file contents
    println!("File contents:\n{}", contents);

    Ok(())

    // Create a new file, truncating it if it already exists
    let mut file = File::create("output.txt")?;

    // Write some text to the file
    file.write_all(b"Hello, world!")?;

    println!("Data written to file successfully.");

    Ok(())

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.txt")?;

    // Append some text to the file
    file.write_all(b"\nAppending some text.")?;

    println!("Data appended to file successfully.");

    Ok(())

    // Open the file in read-only mode for line by line reading
    let file = File::open("example.txt")?;

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        // Print each line
        println!("{}", line?);
    }

    Ok(())

    //todo micro
    let task1 = Task1::new();
    let task2 = Task2::new();
    let task3 = Task3::new();

    task1.print_status();
    task2.print_status();
    task3.print_status();


    //scope threads
    let numbers = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        // Creating a scoped thread that borrows `numbers`
        s.spawn(|_| {
            for n in &numbers {
                println!("Thread 1: {}", n);
            }
        });

    }

    //thread parking 
    let parked_thread = thread::spawn(|| {
        println!("Thread is about to park.");
        thread::park();  // This parks the thread
        println!("Thread is unparked and running again.");
    });

    // Sleep for 2 seconds to simulate some work in the main thread
    thread::sleep(Duration::from_secs(2));

    // Unpark the thread after some work
    println!("Unparking the thread...");
    parked_thread.thread().unpark();

    // Wait for the parked_thread to finish
    parked_thread.join().unwrap();   


    //async await
    println!("Starting...");
    do_something().await; // Await the future returned by do_something()
    println!("Finished!");

}
