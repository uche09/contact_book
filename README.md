# Contact Book
```
   ____ ___  _   _ _____  _    ____ _____   ____   ___   ___  _  __ 
  / ___/ _ \| \ | |_   _|/ \  / ___|_   _| | __ ) / _ \ / _ \| |/ /
 | |  | | | |  \| | | | / _ \| |     | |   |  _ \| | | | | | | ' / 
 | |__| |_| | |\  | | |/ ___ \ |___  | |   | |_) | |_| | |_| | . \ 
  \____\___/|_| \_| |_/_/   \_\____| |_|   |____/ \___/ \___/|_|\_\
                                                                         
 A Simple Rust CLI Contact Manager                                                                        
```

[![Rust Version](https://img.shields.io/badge/Rust-1.78+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
<!-- [![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)](LICENSE) -->

---

## Features
- Add a new contact with name phone number and email
- View all saved contacts
- Delete a contact by name
- Basic user input handling and validation

## What I Learned
Working on this project helped me practice and understand:
- **Structs**: Using structs to represent a `Contact` with name and phone fields.
- **Vectors**: Storing and managing a dynamic list of contacts.
- **Ownership and Borrowing**: Handling references properly when adding, and deleting contacts.
- **Pattern Matching**: Using `match` statements to control user options and handle possible errors.
- **Input/Output**: Reading user input from the terminal and processing it.
- **Error Handling**: Managing common issues like invalid input and empty searches.

## Challenges Encountered
- Managing **borrowing and ownership rules**, especially when modifying the list of contacts.
- Handling **mutable and immutable references** correctly without causing borrow checker errors.
- Keeping the program **interactive** and **user-friendly** while avoiding panics.

And I was able to over come these challenges to the best of my knowledge yet.

## Example Usage

```bash
CLI PHONE BOOK

Select your action:
1. Add a contact 
2. View all contacts 
3. Delete contact by name 
4. Exit
 1

Contact name:
Alice

contact number:
1234567890

Contact Email:
example@gmail.com

Do you want to save this contact?
1. Yes
2. No
1

Contact saved!

Select your action:
1. Add a contact 
2. View all contacts 
3. Delete contact by name 
4. Exit
2

YOUR CONTACTS

Name: Alice
Phone: 01234567890
Email: example@gmail.com


Select your action:
1. Add a contact 
2. View all contacts 
3. Delete contact by name 
4. Exit
4
Bye!

```

## How to Run

1. Make sure you have Rust installed. If not, install it from [here](https://www.rust-lang.org/tools/install)

2. Clone the repository:

```bash
git clone https://github.com/uche09/contact_book.git
```

3. Navigate to the project directory:

```bash
cd contact_book
```

4. Build and run the project:

```bash
cargo run
```


### If you are excited about seeing how far I go in my rust journey, give me a star ⭐