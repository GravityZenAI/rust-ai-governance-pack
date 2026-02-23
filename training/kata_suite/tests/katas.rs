use kata_suite::*;

#[test]
fn kata01_first_word() {
    assert_eq!(kata01_borrowing::first_word("hello world"), "hello");
    assert_eq!(kata01_borrowing::first_word("single"), "single");
    assert_eq!(kata01_borrowing::first_word("  leading space"), "leading");
}

#[test]
fn kata01_count_words() {
    assert_eq!(kata01_borrowing::count_words("a b  c"), 3);
    assert_eq!(kata01_borrowing::count_words(""), 0);
}

#[test]
fn kata02_push_suffix() {
    let s = kata02_ownership::push_suffix("hi".to_string(), "!");
    assert_eq!(s, "hi!");
}

#[test]
fn kata02_append_suffix_in_place() {
    let mut s = "hi".to_string();
    kata02_ownership::append_suffix_in_place(&mut s, "!");
    assert_eq!(s, "hi!");
}

#[test]
fn kata03_parse_positive() {
    assert_eq!(kata03_result_option::parse_positive_i32("7").unwrap(), 7);
    assert!(kata03_result_option::parse_positive_i32("0").is_err());
    assert!(kata03_result_option::parse_positive_i32("-1").is_err());
    assert!(kata03_result_option::parse_positive_i32("nope").is_err());
}

#[test]
fn kata03_safe_div() {
    assert_eq!(kata03_result_option::safe_div(10, 2), Some(5));
    assert_eq!(kata03_result_option::safe_div(10, 0), None);
}

#[test]
fn kata04_counter_methods() {
    let mut c = kata04_struct_methods::Counter::new();
    assert_eq!(c.get(), 0);
    c.inc();
    assert_eq!(c.get(), 1);
    c.add(41);
    assert_eq!(c.get(), 42);
}

#[test]
fn kata05_area() {
    use kata05_traits::{Area, Circle, Rectangle};
    let r = Rectangle { w: 2.0, h: 3.0 };
    assert_eq!(r.area(), 6.0);

    let c = Circle { r: 2.0 };
    let a = c.area();
    assert!((a - 12.56637).abs() < 1e-3);
}

#[test]
fn kata06_max_of() {
    assert_eq!(kata06_generics::max_of(1, 2), 2);
    assert_eq!(kata06_generics::max_of('a', 'z'), 'z');
}

#[test]
fn kata06_dedup_sorted() {
    assert_eq!(
        kata06_generics::dedup_sorted(&[1, 1, 2, 2, 2, 3]),
        vec![1, 2, 3]
    );
    assert_eq!(kata06_generics::dedup_sorted::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn kata07_longest() {
    assert_eq!(kata07_lifetimes::longest("abc", "z"), "abc");
    assert_eq!(kata07_lifetimes::longest("a", "zzz"), "zzz");
}

#[test]
fn kata08_iterators() {
    assert_eq!(kata08_iterators::squares(&[1, 2, 3]), vec![1, 4, 9]);
    assert_eq!(kata08_iterators::sum_even(&[1, 2, 3, 4]), 6);
}

#[test]
fn kata09_error_propagation() {
    assert_eq!(kata09_error_propagation::read_number("10").unwrap(), 10);
    assert!(kata09_error_propagation::read_number("x").is_err());

    assert_eq!(
        kata09_error_propagation::parse_and_add("10", "32").unwrap(),
        42
    );
    assert!(kata09_error_propagation::parse_and_add("10", "x").is_err());
}

#[test]
fn kata10_modules() {
    assert_eq!(kata10_modules::public_api(4), 12);
}

#[test]
fn kata11_enums_match() {
    use kata11_enums_match::Command::*;
    let cmds = [Add(2), Add(3), Sub(1), Reset, Add(5)];
    assert_eq!(kata11_enums_match::apply(&cmds), 5);
}

#[test]
fn kata12_hashmap() {
    let m = kata12_collections_hashmap::word_frequencies("Hello hello rust");
    assert_eq!(m.get("hello").copied(), Some(2));
    assert_eq!(m.get("rust").copied(), Some(1));
}

#[test]
fn kata13_strings() {
    assert_eq!(kata13_slices_strings::trim_prefix("foobar", "foo"), "bar");
    assert_eq!(
        kata13_slices_strings::trim_prefix("foobar", "baz"),
        "foobar"
    );

    assert!(kata13_slices_strings::is_ascii_palindrome(
        "A man, a plan, a canal: Panama"
    ));
    assert!(!kata13_slices_strings::is_ascii_palindrome("rust"));
}

#[test]
fn kata14_parsing() {
    assert_eq!(
        kata14_parsing::parse_csv_line(" a, b ,c "),
        vec!["a", "b", "c"]
    );
    assert_eq!(kata14_parsing::parse_pair("10:32").unwrap(), (10, 32));
    assert!(kata14_parsing::parse_pair("10:").is_err());
}

#[test]
fn kata15_refcell() {
    let b = kata15_refcell_basics::Bag::new();
    b.push(1);
    b.push(2);
    b.push(3);
    assert_eq!(b.sum(), 6);
}

#[test]
fn kata16_split_borrow() {
    let mut v = vec![10, 20, 30];
    let s = kata16_split_borrow::sum_and_bump_two(&mut v, 0, 2).unwrap();
    assert_eq!(s, 40);
    assert_eq!(v, vec![11, 20, 31]);

    assert!(kata16_split_borrow::sum_and_bump_two(&mut v, 1, 1).is_none());
    assert!(kata16_split_borrow::sum_and_bump_two(&mut v, 0, 99).is_none());
}

#[test]
fn kata17_from_into() {
    use kata17_into_from::{Kilometers, Meters};
    let m: Meters = Kilometers(3).into();
    assert_eq!(m, Meters(3000));
}

#[test]
fn kata18_builder() {
    use kata18_builder_pattern::UserBuilder;

    let u = UserBuilder::new("moy")
        .age(42)
        .email("moy@example.com")
        .build();

    assert_eq!(u.name, "moy");
    assert_eq!(u.age, Some(42));
    assert_eq!(u.email.as_deref(), Some("moy@example.com"));
}

#[test]
fn kata19_threads() {
    let v: Vec<i32> = (1..=100).collect();
    let s = kata19_threads::parallel_sum(v, 4);
    assert_eq!(s, 5050);
}

#[test]
fn kata20_small_parser() {
    assert_eq!(kata20_small_parser::sum_expr("1 + 2 + 3").unwrap(), 6);
    assert!(kata20_small_parser::sum_expr("").is_err());
    assert!(kata20_small_parser::sum_expr("1 + x").is_err());
    assert!(kata20_small_parser::sum_expr("1 +").is_err());
}
