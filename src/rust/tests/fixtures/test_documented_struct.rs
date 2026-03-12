/// This struct has a doc comment
pub struct DocumentedStruct {
    name: String,
}

#[rustfmt::skip]
pub struct IgnoredStruct { //docstring-guard=ignore
    name: String,
}
