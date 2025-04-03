use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use crate::ini::{Ini, IniMode, IniSection};

const SIMPLE_TEST_INI: &str = r#"
; This is a comment.
# This is also a comment.

[My Section]
; Here's a value
some KEY=This is a value!

; Here's another value
anotherkey=This is yet another value!

;commented out=this value isn't real :(

[Another Section]
yourkey=This is a value!
some KEY=This, too, is a value!
anotherkey=//Wow Look At Me I'm A Value\\
"#;

const SIMPLE_TRIMMED_TEST_INI: &str = r#"
; This is a comment.
# This is also a comment.

[My Section]
; Here's a value
some KEY = This is a value!

; Here's another value
anotherkey = This is yet another value!

;commented out = this value isn't real :(

[Another Section]
yourkey = This is a value!
some KEY = This, too, is a value!
anotherkey = //Wow Look At Me I'm A Value\\
"#;

#[test]
fn simple_ini_parse_test() {
    let ini = Ini::parse(SIMPLE_TEST_INI, IniMode::Simple).unwrap();

    assert_eq!(Ini {
        sections: {
            let mut sections = BTreeMap::new();

            sections.insert("My Section".to_owned(), {
                let mut values = BTreeMap::new();

                values.insert("some KEY".to_owned(), "This is a value!".to_owned());
                values.insert("anotherkey".to_owned(), "This is yet another value!".to_owned());

                IniSection {
                    values
                }
            });

            sections.insert("Another Section".to_owned(), {
                let mut values = BTreeMap::new();

                values.insert("yourkey".to_owned(), "This is a value!".to_owned());
                values.insert("some KEY".to_owned(), "This, too, is a value!".to_owned());
                values.insert("anotherkey".to_owned(), r#"//Wow Look At Me I'm A Value\\"#.to_owned());

                IniSection {
                    values
                }
            });

            sections
        }
    }, ini);
}

#[test]
fn simple_ini_parse_trimmed_test() {
    let ini = Ini::parse(SIMPLE_TEST_INI, IniMode::Simple).unwrap();
    let ini_trimmed = Ini::parse(SIMPLE_TRIMMED_TEST_INI, IniMode::SimpleTrimmed).unwrap();
    assert_eq!(ini, ini_trimmed);
}

