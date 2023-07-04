# <a href="https://www.rust-lang.org/">Rust</a> Cheatsheet <img width="50em" src="https://upload.wikimedia.org/wikipedia/commons/thumb/2/20/Rustacean-orig-noshadow.svg/1280px-Rustacean-orig-noshadow.svg.png">
<table> 
<td> 

* [File](#file) 

</td> 
</table> 


## Setup project 

### Create project
`cargo new <project-name>`

### Run file 
`cargo run` 

### Generate exe file 
- exe file is generated in `./target/debug`
`cargo build` 

## Hello world 
```rs 
fn main() {
    println!("Hello, world!");
}
``` 


## Variables 
```rs 
// automatic type 
let var = "String";
 
// set mutability 
let mut var = <value>;
 
// set type
let var: <type> = <value>;
 
/* 
Type:  
        bool           = true, false 
        i, 16, 32, 64  = number in range of x bits, can be negative 
        u, 16, 32, 64  = number in range of x bits, can't be negative 
        f, 64          = decimal numbers 
        String         = string 
*/ 
``` 
 
### Tuples
```rs 
// automatic type
let t = (20, 10, 30);

// set type
let t: (i32, i32, i32) = (20, 10, 30);

let (x, y, z)= t;

t.0 // 20
x // 20
``` 
 
### Arrays
```rs 
// automatic type
let a = [20, 10, 30];

// set type
let a: [i32, 3] = [20, 10, 30];

a[0] // 20
```

### Structs
```rs
struct Name {
    value1: <type>,
    value2: <type>,
}

impl Name {
    fn add(&self) -> i32 {
        self.value1 + self.value2
    }
}

let x = Name {
    value1 = 5,
    value2 = 10,
}

x.add() // 15
```


## Functions
```rs 
fn name() { 
    //... 
} 
 
// return 
fn name() -> <type> { return x } 
 
// parameters  
fn name(param1: <type>) { } 
``` 
 
 
## Logic Statements 
 
### If/else 
```rs 
if x {
    // ...
} else if {
    // ...
} else {
    // ...
}
``` 
 
 
## Loop 
 
### For-I 
```rs 
for i in 0..10 {
    // ...
}
``` 
 
### For-In 
```rs 
for x in y {
    // ...
}
``` 
 
### While 
```rs 
while x {
    // ...
}
``` 
 
 

## Project ideas 
* [name](link)
