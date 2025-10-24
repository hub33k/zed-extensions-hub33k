// use lazy_static::lazy_static;
// use regex::Regex;

// lazy_static! {
//     static ref UNORDERED_RE: Regex =
//         Regex::new(r"^(?P<indent>\s*)(?P<marker>[-*+])\s+(?P<rest>.*)$").unwrap();
//     static ref TASK_RE: Regex =
//         Regex::new(r"^(?P<indent>\s*)(?P<marker>[-*+])\s+\[(?P<checked>[ xX]?)\]\s*(?P<rest>.*)$")
//             .unwrap();
//     static ref ORDERED_RE: Regex =
//         Regex::new(r"^(?P<indent>\s*)(?P<num>\d+)\.\s+(?P<rest>.*)$").unwrap();
// }

// /// Decide whether to handle Enter and what to insert.
// /// `line_text` is the full text of the current line where caret is (before Enter).
// /// `caret_at_end_of_line` indicates if caret is at end (if false, we may still continue if it's within list).
// ///
// /// Returns `Some((replace_range_start_offset, replace_range_end_offset, insert_text))`
// /// meaning: replace the range with (usually zero-length insertion) and insert `insert_text`.
// /// For simplicity this function returns the string to insert (leading newline included).
// pub fn new_line_for(line_text: &str) -> Option<String> {
//     // skip empty
//     if line_text.trim().is_empty() {
//         return None;
//     }

//     // Task list first
//     if let Some(caps) = TASK_RE.captures(line_text) {
//         let indent = caps.name("indent").map(|m| m.as_str()).unwrap_or("");
//         let marker = caps.name("marker").unwrap().as_str();
//         // If the current line only contains the marker and maybe the task box but no text,
//         // pressing Enter should remove the list marker (i.e., produce a blank line).
//         let rest = caps.name("rest").map(|m| m.as_str()).unwrap_or("");
//         if rest.trim().is_empty() {
//             // produce newline + indent (i.e. blank line without marker)
//             return Some(format!("\n{}", indent));
//         } else {
//             // continue task list
//             return Some(format!("\n{}{} [ ] ", indent, marker));
//         }
//     }

//     if let Some(caps) = UNORDERED_RE.captures(line_text) {
//         let indent = caps.name("indent").map(|m| m.as_str()).unwrap_or("");
//         let marker = caps.name("marker").unwrap().as_str();
//         let rest = caps.name("rest").map(|m| m.as_str()).unwrap_or("");
//         if rest.trim().is_empty() {
//             // remove marker on blank list item
//             return Some(format!("\n{}", indent));
//         } else {
//             return Some(format!("\n{}{} ", indent, marker));
//         }
//     }

//     if let Some(caps) = ORDERED_RE.captures(line_text) {
//         let indent = caps.name("indent").map(|m| m.as_str()).unwrap_or("");
//         let num_str = caps.name("num").unwrap().as_str();
//         let rest = caps.name("rest").map(|m| m.as_str()).unwrap_or("");
//         if rest.trim().is_empty() {
//             // blank ordered item -> remove numbering
//             return Some(format!("\n{}", indent));
//         } else {
//             // increment number
//             if let Ok(n) = num_str.parse::<usize>() {
//                 let next = n + 1;
//                 return Some(format!("\n{}{}. ", indent, next));
//             } else {
//                 return Some(format!("\n{}1. ", indent));
//             }
//         }
//     }

//     None
// }
