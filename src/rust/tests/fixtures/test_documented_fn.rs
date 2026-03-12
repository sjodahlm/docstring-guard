/// This function has a doc comment
pub fn documented_function() {
    println!("hello");
}

/** this function has outer doc block */
pub fn outer_doc_function() {
    println!("no doc comment here");
}

#[rustfmt::skip]
pub fn ignore_this_function() { //docstring-guard=ignore
    println!("this function is ignored");
}
