use super::ast::structs;

named!(identifier<&str, String>,
    do_parse!(
        value: re_find_static!("[a-zA-Z_][a-zA-Z_\\-0-9]*") >>
        (value.to_string())
    )
);

named!(pub parse_module<&str, structs::Module>,
    do_parse!(
        space0 >>
        tag!("module") >>
        space1 >>
        name: identifier >>
        space1 >>
        content: delimited!(tag!("{"), module_content, tag!("}")) >>
        (structs::Module{
            name: name,
            ..content
        })
    )
);

named!(module_content<&str, structs::Module>,
    fold_many0!(
        alt!(
            parse_prefix => { |val: String| structs::Node::Prefix(val) } |
            parse_namespace => { |val: String| structs::Node::Namespace(val) } |
            parse_anyxml => { |val: String| structs::Node::Unknown(val) } |
            parse_augment => { |val: String| structs::Node::Unknown(val) } |
            parse_choice => { |val: String| structs::Node::Unknown(val) } |
            parse_contact => { |val: String| structs::Node::Contact(val) } |
            parse_container => { |val: String| structs::Node::Unknown(val) } |
            parse_description => { |val: String| structs::Node::Description(val) } |
            parse_deviation => { |val: String| structs::Node::Unknown(val) } |
            parse_extension => { |val: String| structs::Node::Unknown(val) } |
            parse_feature => { |val: String| structs::Node::Unknown(val) } |
            parse_grouping => { |val: String| structs::Node::Unknown(val) } |
            parse_identity => { |val: String| structs::Node::Unknown(val) } |
            parse_import => { |val: String| structs::Node::Unknown(val) } |
            parse_include => { |val: String| structs::Node::Unknown(val) } |
            parse_leaf => { |val: String| structs::Node::Unknown(val) } |
            parse_leaf_list => { |val: String| structs::Node::Unknown(val) } |
            parse_list => { |val: String| structs::Node::Unknown(val) } |
            parse_notification => { |val: String| structs::Node::Unknown(val) } |
            parse_organization => { |val: String| structs::Node::Organization(val) } |
            parse_reference => { |val: String| structs::Node::Reference(val) } |
            parse_revision => { |val: String| structs::Node::Unknown(val) } |
            parse_rpc => { |val: String| structs::Node::Unknown(val) } |
            parse_typedef => { |val: String| structs::Node::Unknown(val) } |
            parse_uses => { |val: String| structs::Node::Unknown(val) } |
            parse_yang_version => { |val: String| structs::Node::YangVersion(val) }
        ),
        structs::Module::new(),
        |mut m: structs::Module, item: structs::Node| {
            match item {
                structs::Node::Contact(v) => {
                    m.contact = v;
                }
                structs::Node::Description(v) => {
                    m.description = v;
                }
                structs::Node::Namespace(v) => {
                    m.namespace = v;
                }
                structs::Node::Organization(v) => {
                    m.organization = v;
                }
                structs::Node::Prefix(v) => {
                    m.prefix = v;
                }
                structs::Node::Reference(v) => {
                    m.reference = v;
                }
                structs::Node::YangVersion(v) => {
                    m.yang_version = v;
                }
                _ => {}
            }
            m
        }
));

named!(parse_prefix<&str, String>, do_parse!(
    space0 >>
    tag!("prefix") >>
    space1 >>
    prefix: identifier >>
    space0 >>
    tag!(";") >>
    space0 >>
    (prefix)
));

named!(space0<&str, ()>, do_parse!(
    re_find_static!("^\\s*") >>
    ()
));

named!(space1<&str, ()>, do_parse!(
    re_find_static!("^\\s+") >>
    ()
));

fn parse_tag<'a, 'b>(input: &'a str, tag_name: &'b str) -> nom::IResult<&'a str, String> {
    do_parse!(input,
        space0 >>
        tag!(tag_name) >>
        space1 >>
        value: value >>
        space0 >>
        tag!(";") >>
        space0 >>
        (value)
    )
}

fn parse_namespace<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    parse_tag(input, "namespace")
}

//named!(parse_namespace<&str, String>, do_parse!(
    //space0 >>
    //tag!("namespace") >>
    //space1 >>
    //value: value >>
    //space0 >>
    //tag!(";") >>
    //space0 >>
    //(value)
//));

named!(value<&str, String>, do_parse!(
    value: delimited!(tag!("\""), re_find_static!("[^\"]*"), tag!("\"")) >>
    (value.to_string())
));

named!(parse_anyxml<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_augment<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_choice<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

fn parse_contact<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    parse_tag(input, "contact")
}

named!(parse_container<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

fn parse_description<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    parse_tag(input, "description")
}

named!(parse_deviation<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_extension<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_feature<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_grouping<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_identity<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_import<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_include<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_leaf<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_leaf_list<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_list<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_notification<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

fn parse_organization<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    parse_tag(input, "organization")
}

fn parse_reference<'a>(input: &'a str) -> nom::IResult<&'a str, String> {
    parse_tag(input, "reference")
}

named!(parse_revision<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_rpc<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_typedef<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_uses<&str, String>, do_parse!(
    space1 >>
    (String::new())
));

named!(parse_yang_version<&str, String>, do_parse!(
    space0 >>
    tag!("yang-version") >>
    space1 >>
    value: value >>
    space0 >>
    tag!(";") >>
    space0 >>
    (value)
));

named!(boolean<&str, bool>, do_parse!(
    space0 >>
    value: alt!(tag!("true") | tag!("false")) >>
    space0 >>
    (value.parse::<bool>().unwrap())  
));

#[test]
fn test_parse_minimal_module() {
    assert_eq!(
        parse_module("module test {
        namespace \"my-namespace\";
        prefix myprefix;
        }"),
        Ok(("", structs::Module {
            name: "test".to_string(),
            namespace: "my-namespace".to_string(),
            prefix: "myprefix".to_string(),
            ..structs::Module::new()
        }))
    );
}

#[test]
fn test_parse_complete_module() {
    assert_eq!(
        parse_module("module test {
        contact \"FooBar\";
        description \"Larodi\";
        namespace \"my-namespace\";
        organization \"org\";
        prefix myprefix;
        reference \"ref\";
        yang-version \"1\";
        }"),
        Ok(("", structs::Module {
            contact: "FooBar".to_string(),
            description: "Larodi".to_string(),
            name: "test".to_string(),
            namespace: "my-namespace".to_string(),
            organization: "org".to_string(),
            prefix: "myprefix".to_string(),
            reference: "ref".to_string(),
            yang_version: "1".to_string(),
            ..structs::Module::new()
        }))
    );
}

#[test]
fn test_parse_boolean() {
    let result = boolean("true").unwrap();
    assert_eq!(result.1, true);

    let result = boolean("false").unwrap();
    assert_eq!(result.1, false);

    let result = boolean("foobar");
    assert_eq!(result.is_err(), true);
}
