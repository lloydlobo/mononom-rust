use std::{collections::HashMap, vec};

// region:      --- hash_maps ---

pub fn phone_book_hash_map() {
    let mut phone_book: HashMap<i64, &str> = HashMap::new();

    let phone_book_table = format!("{:<10} {:<10}", "Phone", "Name");
    println!("{}", phone_book_table);

    let mut entries_list: Vec<(i64, &str)> = vec![
        (9090909090, "John Doe"),
        (9090909091, "Jane Doe"),
        (9090909092, "John Smith"),
        (9090909093, "Jane Smith"),
    ];

    entries_list.push((2342352352, "Jose Doe"));

    // Add new entry to the phone book
    let new_entry = vec![(9899094094, "Jon Doe")];
    insert_new_phone_book_entry(&mut phone_book, new_entry);
    insert_new_phone_book_entry(&mut phone_book, vec![(9064909095, "Lola")]);

    // print_phone_entries(&entries_list, &mut phone_book);
    let _ = &entries_list.push((9909343494, "Ziggy"));
    insert_new_phone_book_entry(&mut phone_book, vec![(9093343494, "Claudia")]);
    print_phone_entries(&entries_list, &mut phone_book);

    // Find length of hash map (number of phone_book entries count)
    let phone_book_entries_count = &mut phone_book.len(); // 4 but why isn't it 5? Because it's a reference to the hash map
    println!("Phone book entries count is: {}", phone_book_entries_count);

    // Find if a value exists in the hash map
    match_phone_book_number_with_name(&phone_book, 9090909090);
    match_phone_book_number_with_name(&phone_book, 9090909094);
}

// endregion:   --- hash_maps ---

// region:      --- iterating_over_hash_maps ---

/// Prints the phone book entries
/// returns: nothing
/// params:  phone_book_entries_list: &[(&str, i64)], phone_book: &mut HashMap<&str, i64>
/// # Functions
/// - print_phone_entries
/// # Examples
/// ```
/// print_phone_entries(entries_list, &mut phone_book);
/// ```
fn print_phone_entries<'a>(
    entries_list: &Vec<(i64, &'a str)>,
    phone_book: &mut HashMap<i64, &'a str>,
) {
    for (number, name) in entries_list {
        phone_book.insert(*number, *name);
    }

    let mut len: i64 = 0;
    for (number, name) in phone_book {
        println!("{} {} {}", len, number, name);
        len += 1;
    }
    // println!("{:?}", phone_book);
}

// this 'a is a lifetime
/// Inserts a new phone_book entry
/// returns: nothing
/// params:  phone_book: &mut HashMap<&str, i64>, name: &str, number: i64
/// # Functions
/// - insert_phone_book_entry
/// # Examples
/// ```
/// insert_phone_book_entry(&mut phone_book, "John Doe", 9090909090);
/// ```
/// # Errors
/// - none
fn insert_new_phone_book_entry<'a>(
    phone_book: &mut HashMap<i64, &'a str>,
    phone_book_entries: Vec<(i64, &'a str)>,
) {
    for (number, name) in phone_book_entries {
        phone_book.insert(number, name);
    }
}

// endregion:   --- iterating_over_hash_maps ---

// region:      --- match_phone_book_number_with_name ---

/// Find if a value exists in the hash map
/// returns: nothing
/// params:  phone_book: HashMap<&str, i64>, name: &str
/// # Functions
/// - match_phone_book_number_with_name
/// # Examples
/// ```
/// match_phone_book_number_with_name(&phone_book, "Jane Doe");
/// ```
/// # Errors
/// - none
/// # Notes
/// - none
/// # See Also
/// - none
/// # References
/// - none
fn match_phone_book_number_with_name<'a>(phone_book: &'a HashMap<i64, &str>, number: i64) {
    match phone_book.get(&number) {
        Some(name) => println!("The phone number {} is of: {}", number, name),
        None => println!("No entry found for number: {}", number),
    }
}

// endregion:   --- match_phone_book_number_with_name ---

/* TEST SUITE
'########:'########::'######::'########:::::'######::'##::::'##:'####:'########:'########:
... ##..:: ##.....::'##... ##:... ##..:::::'##... ##: ##:::: ##:. ##::... ##..:: ##.....::
::: ##:::: ##::::::: ##:::..::::: ##::::::: ##:::..:: ##:::: ##:: ##::::: ##:::: ##:::::::
::: ##:::: ######:::. ######::::: ##:::::::. ######:: ##:::: ##:: ##::::: ##:::: ######:::
::: ##:::: ##...:::::..... ##:::: ##::::::::..... ##: ##:::: ##:: ##::::: ##:::: ##...::::
::: ##:::: ##:::::::'##::: ##:::: ##:::::::'##::: ##: ##:::: ##:: ##::::: ##:::: ##:::::::
::: ##:::: ########:. ######::::: ##:::::::. ######::. #######::'####:::: ##:::: ########:
:::..:::::........:::......::::::..:::::::::......::::.......:::....:::::..:::::........::
*/

// region:      --- TEST ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_book_hash_map_runs() {
        phone_book_hash_map();
    }

    #[test]
    fn test_match_phone_book_number_with_name_runs() {
        let mut phone_book: HashMap<i64, &str> = HashMap::new();
        phone_book.insert(9090909090, "John Doe");
        phone_book.insert(9090909091, "Jane Doe");
        phone_book.insert(9090909092, "John Smith");
        phone_book.insert(9090909093, "Jane Smith");
        match_phone_book_number_with_name(&phone_book, 9090909091);

        let mut phone_book: HashMap<i64, &str> = HashMap::new();
        phone_book.insert(9090909090, "John Doe");
        phone_book.insert(9090909091, "Jane Doe");
        phone_book.insert(9090909092, "John Smith");
        phone_book.insert(9090909093, "Jane Smith");
        match_phone_book_number_with_name(&phone_book, 9090909094);
    }

    #[test]
    fn test_entries_list_runs() {
        let mut entries_list: Vec<(i64, &str)> = vec![
            (9090909090, "John Doe"),
            (9090909091, "Jane Doe"),
            (9090909092, "John Smith"),
            (9090909093, "Jane Smith"),
        ];
        entries_list.push((2342352352, "John Doe"));
        assert!(entries_list.len() == 5);
        entries_list.push((989909094, "Jon Doe"));
        assert!(entries_list.len() == 6);
        entries_list.push((9064909095, "Lola"));
        assert!(entries_list.len() == 7);
    }
}

// endregion:   --- TEST ---
