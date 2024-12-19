/*
1
Ownership is a set of rules that the compiler checks for to ensure the program
will be free of memory errors
Memory refers to the area of your computer that is responsible for storing
the information your programs use
Its ideal to free memory when it is no longer in use
Programming languages implement different strategies for memory management

When a program requests memory its called allocation
and When it gives memory back to the computer its called deallocation

Ownership is a set of rules on how rust manages your computers memory
The rust compiler does not compile the program if an ownership rule is violated
The speed of a language like C and with less room for error like python

The owner is who/what is responsible for cleaning up a piece of data when it is no longer
in use
Every value in a Rust program has one owner.
The owner can change over the course of the program, but there is only 1 owner
for a value at a time
The owner is usually a name : A variable can be an owner, A parameter can be an owner
Ownership also extends to composite types that own their elements: A tuple and array own their values
*/

/*
    2: THE STACK AND THE HEAP
    The stack and the heap are two different parts/regions of the computer's memory
    The stack and heap read and write date in different ways that offer advantages and
    disadvantages
    (The stack is generally faster but it only supports data of a fixed, predictable constant
    size and also that size must be known at compile time.
    The heap is generally slower, but it supports dynamic data that can change in size over
    the program's execution.)
    {Compile-time is the time at which the source code is converted into an executable code while
    the run time is the time at which the executable code is started running.}

--The Stack
• A stack stores values in the sequential order it receives them.
• A stack is last in, first out (LIFO). The last item added is the first one removed.
• The technical terminology for adding data is pushing onto the stack.
• The technical terminology for removing data is popping off the stack.

The Stack II
• All stack data has a fixed, consistent size that is known at compile time.
• Data types like integers, floating- points, Booleans, characters, and arrays have a fixed size.
Rust stores them on the stack at runtime.
• The pieces of data on the stack will not grow or shrink in size as the program runs.
• All stack data has a fixed, consistent size that is known at compile time.
• Data types like integers, floating- points, Booleans, characters, and arrays have a fixed size.
Rust stores them on the stack at runtime.

The Heap
• The heap is a large area of storage space. Think of it like a warehouse.
• The heap is for data whose size is not known at compile time (user input,
a file's contents, etc).
• When the Rust program needs dynamic space, it requests it from the heap.
A program called the memory allocator finds an empty spot that is large enough to store the data.
References
• The memory allocator returns a reference, which is an address.
• The reference points to the memory address of the data.
• Think of a parking lot giving you a reference (spot "H25") when they park your car.
• We can store a reference in a variable in a Rust program. References have a fixed size,
so Rust stores them on the stack.
The Heap II
• Allocating on the heap is slower than pushing to the stack. The memory allocator has
to spend time searching for an open spot large enough to fit the data.
• Accessing data is faster on the stack than the heap as well. With a heap,
ng on the heap is slower than pushing to the stack. The memory allocator has to spend time searching for an open spot large enough to fit the data.
• Accessing data is faster on the stack than the heap as well. With a heap, the program has to follow the pointer to find the memory address.
• A stack sthe program has to follow the pointer to find the memory address.
• A stack stores the data in sequence, so there is less "jumping around" from point to point.
Ownership
• The purpose of ownership is to assign responsibility for deallocating memory
(primarily heap memory).
• Ownership is a compiler feature for reducing duplicate heap data and cleaning
up heap data that is no longer needed.
*/

/*
3
when we asign a value to a variable, the variable becomes the owner of the value
In this example, 'age' is the owner of the value 33.
The owner is who is responsible for cleaning up the data. And how does the owner know when to clean
up the data from either the stack or the heap?
The answer is the owner knows to clean up the data when the variable goes out of scope, which is when
the block ends.
When the variable goes out of scope, it is the owner who deallocates the memory for that value.
It either pops the stack entry or it cleans up the heap data
*/

/*
4
THE COPY TRAIT
the copy trait mandates that a type can be copied which means thata full duplicate can be created from it.
Rust's primitive data types, or the fixed size ones that we store on the stack like integers, floats,
Booleans, characters, and more.They all implement the Copy trait, and this means that full copies of those values will be created
*/

/*
5
THE STRING TYPE
the '&str" type not stored on either the stack or the heap, Rather, it is embedded directly into the binary executable that the Rust compiler produces.
And the reason that is done is because the value is already known at compile time.
String -> when we need dynamic string type
*/

/*
6
THE push_str METHOD ON A STRING TYPE
pust_str method to concatenate to the end of the string
*/

/*
7
MOVES AND OWNERSHIP
a move is the transfer of ownership from one owner to another
When we assign 'person' to 'genius', Rust moves ownership from the 'person' variable to the 'genius' variable.
'genius' becomes the owner, which means that 'person' actually becomes invalid.
It goes out of scope.
Stack memory doesn't work with the drop function
*/

/*
8
THE DROP FUNCTION
*/

/*
9
THE CLONE METHOD
The 'clone' method is actually a requirement of a trait called cCone.
*/

/*
10
REFERENCES AND BORROWING
A reference allows the program to use a value without moving ownership.
Creating a reference -> Borrowing
Borrowing means using something without taking ownership of it.
References must never outlive their referent
*/

/*
11
THE DEREFERENCES OPERATOR
*(dereferences operator)
To dereference means to access the data at the memory address that the
reference points to.
*/

/*
12
STRING, &STRING,STR AND &STR
String - a dynamic piece of text stored on the heap at runtime
&String ("ref String") - A reference to a heap String
str - A hardcoded, read-only piece of text encoded in the binary
&str ("ref str") - A reference to the text in the memory that has loaded the binary file
*/

/*
13
THE COPY TRAIT WITH REFERENCES
*/

/*
14
OWNERSHIP AND FUNCTION PARAMETERS
*/

/*
15
MUTABLE PARAMETERS
Just like variables, function parameters are immutable by default.
*/

/*
16, 17
RETURN VALUES

*/

use std::backtrace;

fn main() {
    //3: SCOPE AND OWNERSHIP
    let age = 18;
    {
        let is_ok = false; //this variable exist here
    } //is_ok doesn't exist here

    //4: THE COPY TRAIT
    let time = 2025;
    let year = time;
    println!("The {time} and the {year}.");

    //5: THE STRING TYPE
    let text = String::new(); //its not a method but its a plain function
    let candy = String::from("KitKat"); // this is going to create a Rust on the heap, a "String", but based on this original data.

    //6: THE push_str METHOD ON A STRING TYPE
    let mut name = String::from("Leon");
    println!("Name before: {name}");
    //The stack entry will hold three pieces of data. 1. reference to the String(address), 2.length of the string, 3.capacity and what the capacity
    // is the amount of bytes capacity and what the capacity is,
    name.push_str(" Islam");
    println!("Name after: {name}");

    //7: MOVES AND OWNERSHIP
    let person = String::from("Leon");
    let genius = person;
    //A heap allocated String does not implement the copy trait, and therefore rust will not make a copy
    // println!("This is an error {person}");
    println!("This will run {genius}");

    //8: THE DROP FUNCTION
    let a = String::from("hi");
    drop(a);
    // let b = a; //error cause a is already dropped

    //THE COPY TRAIT
    let cln = String::from("Clone");
    let cln2 = cln.clone();
    println!("This is cloned {cln}");
    //This time it wont show any error

    //REFERENCES AND BORROWING
    let my_stack_value = 2;
    let my_stack_reference = &my_stack_value;
    println!("{}", my_stack_reference);
    //rust implements the  display trait for references
    println!("{}", *my_stack_reference);

    let my_heap_value = String::from("String");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference);
    println!("{}", *my_heap_reference);

    //STRING, &STRING,STR AND &STR

    //THE COPY TRAIT WITH REFERENCES

    //OWNERSHIP AND FUNCTION PARAMETERS
    let example = 23;
    print_my_i_val(example); // like "let value = example"
    println!("{example} is still valid.");

    let example_str = String::from("This is a string.");
    print_my_s_val(example_str);
    // println!("{example_str} is not valid.");

    //MUTABLE PARAMETERS
    let ex_mp = String::from("Berger");
    mut_par(ex_mp);

    //RETURN VALUES
    let rt_ex = bake_cake();
    println!("I have a {rt_ex} cake.")
}

//OWNERSHIP AND FUNCTION PARAMETERS
fn print_my_i_val(value: i32) {
    println!("The value is {value}")
}
fn print_my_s_val(value: String) {
    println!("The value is {value}")
}

//MUTABLE PARAMETERS
fn mut_par(mut meal: String) {
    meal.push_str("and Fries");
    println!("Example of mutable parameters: {meal}")
}

//RETURN VALUES
fn bake_cake() -> String {
    let cake = String::from("Chocolate");
    cake
}
