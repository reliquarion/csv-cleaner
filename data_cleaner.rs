use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::{Read, Write};

fn i8_caster(text: &str) -> bool {
    text.parse::<i8>().is_ok()
}

fn i16_caster(text: &str) -> bool {
    text.parse::<i16>().is_ok()
}

fn i32_caster(text: &str) -> bool {
    text.parse::<i32>().is_ok()
}

fn i64_caster(text: &str) -> bool {
    text.parse::<i64>().is_ok()
}

fn u8_caster(text: &str) -> bool {
    text.parse::<u8>().is_ok()
}

fn u16_caster(text: &str) -> bool {
    text.parse::<u16>().is_ok()
}

fn u32_caster(text: &str) -> bool {
    text.parse::<u32>().is_ok()
}

fn u64_caster(text: &str) -> bool {
    text.parse::<u64>().is_ok()
}

fn f32_caster(text: &str) -> bool {
    text.parse::<f32>().is_ok()
}

fn f64_caster(text: &str) -> bool {
    text.parse::<f64>().is_ok()
}

fn bool_caster(text: &str) -> bool {
    text == "True" || text == "False"
}

fn main() {
    // HashMap that connect the string representation of a pandas dtype into a function
    // which confirms valid casting of a string representation of an item of that type
    // This is missing a few possible Pandas dtypes such as datetime and sparse
    // since I have never seen them and am thus unable to predict whether it's possible to cast
    // an item to that type.
    // The full list of Pandas dtypes is here: https://pandas.pydata.org/pandas-docs/stable/user_guide/basics.html#basics-dtypes
    let type_converter: HashMap<&'static str, fn(&str) -> bool> = HashMap::from([
        ("int8", i8_caster as fn(&str) -> bool),
        ("int16", i16_caster as fn(&str) -> bool),
        ("int32", i32_caster as fn(&str) -> bool),
        ("int64", i64_caster as fn(&str) -> bool),
        ("uint8", u8_caster as fn(&str) -> bool),
        ("uint16", u16_caster as fn(&str) -> bool),
        ("uint32", u32_caster as fn(&str) -> bool),
        ("uint64", u64_caster as fn(&str) -> bool),
        ("float32", f32_caster as fn(&str) -> bool),
        ("float64", f64_caster as fn(&str) -> bool),
        ("boolean", bool_caster as fn(&str) -> bool),
        ("string", |_| -> bool { true }),
        ("category", |_| -> bool { true }),
        ("object", |_| -> bool { true })
    ]);

    // Command line arguments in syntax of [datafile path] *[data type, space-seperated]
    let mut types: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    // Remove the datafile path from the list of dtypes
    let filename = types.remove(0);
    // Try to open the data file
    let mut data = File::open(filename.clone()).expect("Could not open file to read");
    // Create a String buffer in which to store the data
    let mut text = String::new();
    // Read the data
    data.read_to_string(&mut text).expect("Could not read file data");

    // The corrected version of the file data
    let mut newtext = String::new();
    // The number of columns in the current line
    // This must be externall tracked because the Split<'_, char> from line.split(',')
    // Does not have a len() method
    let mut len;
    // The index of the current line, starts at -1 so the first
    // Numbered line begins at 0
    let mut i = -1;
    'lineiter: for mut line in text.lines() {
        if i == -1 {
            // Write the column names without changes
            newtext.push_str(line);
            i += 1;
        } else {
            // Remove the index from the current line
            line = line.split_once(',').expect("Empty line found in datafile").1;

            len = 0;
            // For every cell in the current line
            for (n, val) in line.split(',').enumerate() {
                if n >= types.len() // If the line has too many columns
                || !type_converter[types[n].as_str()](val) // Or if the current cell cannot be cast to its intended type
                {
                    // Go to the next line
                    continue 'lineiter;
                }
                len += 1;
            }
            // Checks if the line has too few columns
            if len < types.len() {
                continue 'lineiter;
            }

            // Write the line if all items can be cast to the proper types
            newtext.push('\n');
            newtext.push_str(&i.to_string()); // Add the index at the start
            newtext.push(',');
            newtext.push_str(line); // Add the data
            i += 1;
        }
    }

    // Re-open the file for writing
    data = File::create(filename).expect("Could not open file");
    // Write the data
    data.write_all(newtext.as_bytes()).expect("Could not write file data");
}