//! Upstream tests/DocBlock/*Test.php. Data providers are exported verbatim to tests.json by
//! scripts/export-docblock-tests.php; each test below mirrors the PHP test method body.

use super::*;
use serde_json::Value;
use std::rc::Rc;
use std::sync::OnceLock;

fn data() -> &'static Value {
    static D: OnceLock<Value> = OnceLock::new();
    D.get_or_init(|| serde_json::from_str(include_str!("tests.json")).unwrap())
}

fn cases(name: &str) -> Vec<Vec<Value>> {
    data()[name].as_array().unwrap_or_else(|| panic!("no {name}")).iter().map(|c| c["args"].as_array().unwrap().clone()).collect()
}

fn st(class: &str, name: &str) -> Value {
    data()[format!("{class}::__static")][name].clone()
}

fn s(v: &Value) -> Vec<u8> {
    v.as_str().unwrap_or_else(|| panic!("not a string: {v}")).as_bytes().to_vec()
}

fn list(v: &Value) -> Vec<Vec<u8>> {
    v["items"].as_array().unwrap().iter().map(s).collect()
}

fn ctx(ns: Option<&Value>, uses: Option<&Value>) -> Rc<Context> {
    let namespace = ns.filter(|v| !v.is_null()).map(|v| Namespace { full_name: s(&v["fullName"]) });
    let namespace_uses = uses
        .map(|u| u["items"].as_array().unwrap().iter().map(|u| NamespaceUse { full_name: s(&u["fullName"]), short_name: s(&u["shortName"]) }).collect())
        .unwrap_or_default();
    Rc::new(Context { namespace, namespace_uses })
}

fn lossy(v: &[u8]) -> String {
    String::from_utf8_lossy(v).into_owned()
}

fn ann(line: &[u8]) -> Annotation {
    Annotation::new(vec![(0, Line::shared(line))], Rc::default())
}

fn panics(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(f);
    std::panic::set_hook(prev);
    match r {
        Ok(()) => panic!("expected panic"),
        Err(e) => e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default(),
    }
}

// ---- LineTest

#[test]
fn line_tests() {
    let sample = s(&st("LineTest", "sample"));
    let doc = DocBlock::new(&sample);
    for c in cases("LineTest::testPosAndContent") {
        let pos = c[0].as_u64().unwrap() as usize;
        assert_eq!(lossy(doc.get_line(pos).unwrap().borrow().get_content()), lossy(&s(&c[1])));
    }
    for c in cases("LineTest::testStartOrEndPos") {
        let pos = c[0].as_u64().unwrap() as usize;
        let line = doc.get_line(pos).unwrap().borrow();
        assert_eq!(line.is_the_start(), pos == 0);
        assert_eq!(line.is_the_end(), pos == 14);
    }
    for c in cases("LineTest::testUseful") {
        let pos = c[0].as_u64().unwrap() as usize;
        assert_eq!(doc.get_line(pos).unwrap().borrow().contains_useful_content(), c[1].as_bool().unwrap(), "line {pos}");
    }
    for c in cases("LineTest::testTag") {
        let pos = c[0].as_u64().unwrap() as usize;
        assert_eq!(doc.get_line(pos).unwrap().borrow().contains_a_tag(), c[1].as_bool().unwrap(), "line {pos}");
    }
}

#[test]
fn line_set_content() {
    let mut line = Line::new(&b"     * @param $foo Hi!\n"[..]);
    assert_eq!(line.get_content(), b"     * @param $foo Hi!\n");
    line.add_blank();
    assert_eq!(line.get_content(), b"     * @param $foo Hi!\n     *\n");
    line.set_content(&b"\t * test\r\n"[..]);
    assert_eq!(line.get_content(), b"\t * test\r\n");
    line.add_blank();
    assert_eq!(line.get_content(), b"\t * test\r\n\t *\r\n");
    line.remove();
    assert_eq!(line.get_content(), b"");
}

// ---- DocBlockTest

#[test]
fn docblock_tests() {
    let sample = s(&st("DocBlockTest", "sample"));
    let doc = DocBlock::new(&sample);
    assert_eq!(doc.get_content(), sample);
    assert_eq!(DocBlock::new(b"").get_content(), b"");
    assert_eq!(doc.get_lines().len(), 15);
    assert!(doc.get_line(15).is_none());
    assert_eq!(doc.get_annotations().len(), 5);
    assert!(doc.get_annotation(5).is_none());
    let params = doc.get_annotations_of_type(&["param"]);
    assert_eq!(params.len(), 3);
    assert_eq!(lossy(&params[0].get_content()), "     * @param string $hello\n");
    assert_eq!(lossy(&params[1].get_content()), "     * @param bool $test Description\n     *        extends over many lines\n");
    assert_eq!(lossy(&params[2].get_content()), "     * @param adkjbadjasbdand $asdnjkasd\n");
    let throws = doc.get_annotations_of_type(&["throws"]);
    assert_eq!(throws.len(), 1);
    assert_eq!(lossy(&throws[0].get_content()), "     * @throws \\Exception asdnjkasd\n     * asdasdasdasdasdasdasdasd\n     * kasdkasdkbasdasdasdjhbasdhbasjdbjasbdjhb\n");
    let ret = doc.get_annotations_of_type(&["return"]);
    assert_eq!(ret.len(), 1);
    assert_eq!(lossy(&ret[0].get_content()), "     * @return void\n");
    assert_eq!(doc.get_annotations_of_type(&["foo"]).len(), 0);
    assert!(doc.is_multi_line());
}

#[test]
fn docblock_make_multi_line() {
    for c in cases("DocBlockTest::testMakeMultiLine") {
        let input = s(&c[0]);
        let output = c.get(1).filter(|v| !v.is_null()).map(s).unwrap_or_else(|| input.clone());
        let indent = c.get(2).map(s).unwrap_or_default();
        let nl = c.get(3).map(s).unwrap_or(b"\n".to_vec());
        let mut doc = DocBlock::new(&input);
        doc.make_multi_line(&indent, &nl);
        assert_eq!(lossy(&doc.get_content()), lossy(&output));
    }
}

#[test]
fn docblock_make_single_line() {
    for c in cases("DocBlockTest::testMakeSingleLine") {
        let input = s(&c[0]);
        let output = c.get(1).filter(|v| !v.is_null()).map(s).unwrap_or_else(|| input.clone());
        let mut doc = DocBlock::new(&input);
        doc.make_single_line();
        assert_eq!(lossy(&doc.get_content()), lossy(&output));
    }
}

// ---- ShortDescriptionTest

#[test]
fn short_description_get_end() {
    for c in cases("ShortDescriptionTest::testGetEnd") {
        let doc = DocBlock::new(&s(&c[1]));
        assert_eq!(ShortDescription::new(&doc).get_end(), c[0].as_u64().map(|v| v as usize), "{:?}", c[1]);
    }
}

// ---- TagTest / TagComparatorTest

#[test]
fn tag_name() {
    for c in cases("TagTest::testName") {
        let (expected, new, input) = (s(&c[0]), s(&c[1]), s(&c[2]));
        let tag = Tag::new(Line::shared(input));
        assert_eq!(lossy(&tag.get_name()), lossy(&expected));
        if expected == b"other" {
            let msg = panics(std::panic::AssertUnwindSafe(|| tag.set_name(&new)));
            assert!(msg.contains("Cannot set name on unknown tag"));
            continue;
        }
        tag.set_name(&new);
        assert_eq!(tag.get_name(), new);
    }
    for c in cases("TagTest::testValid") {
        assert_eq!(Tag::new(Line::shared(s(&c[1]))).valid(), c[0].as_bool().unwrap(), "{:?}", c[1]);
    }
}

#[test]
fn tag_comparator() {
    for c in cases("TagComparatorTest::testComparatorTogether") {
        let t1 = Tag::new(Line::shared([b"* @", &s(&c[0])[..]].concat()));
        let t2 = Tag::new(Line::shared([b"* @", &s(&c[1])[..]].concat()));
        assert_eq!(tag_comparator::should_be_together(&t1, &t2, tag_comparator::DEFAULT_GROUPS), c[2].as_bool().unwrap());
    }
    for c in cases("TagComparatorTest::testComparatorTogetherWithDefinedGroups") {
        let groups: Vec<Vec<String>> =
            c[0]["items"].as_array().unwrap().iter().map(|g| g["items"].as_array().unwrap().iter().map(|t| t.as_str().unwrap().to_string()).collect()).collect();
        let groups: Vec<Vec<&str>> = groups.iter().map(|g| g.iter().map(String::as_str).collect()).collect();
        let groups: Vec<&[&str]> = groups.iter().map(Vec::as_slice).collect();
        let t1 = Tag::new(Line::shared([b"* @", &s(&c[1])[..]].concat()));
        let t2 = Tag::new(Line::shared([b"* @", &s(&c[2])[..]].concat()));
        assert_eq!(tag_comparator::should_be_together(&t1, &t2, &groups), c[3].as_bool().unwrap());
    }
}

// ---- AnnotationTest

#[test]
fn annotation_sample() {
    let sample = s(&st("AnnotationTest", "SAMPLE"));
    for c in cases("AnnotationTest::testGetContent") {
        let doc = DocBlock::new(&sample);
        let a = doc.get_annotation(c[0].as_u64().unwrap() as usize).unwrap();
        assert_eq!(lossy(&a.get_content()), lossy(&s(&c[1])));
    }
    for c in cases("AnnotationTest::testStart") {
        let doc = DocBlock::new(&sample);
        assert_eq!(doc.get_annotation(c[0].as_u64().unwrap() as usize).unwrap().get_start() as u64, c[1].as_u64().unwrap());
    }
    for c in cases("AnnotationTest::testEnd") {
        let doc = DocBlock::new(&sample);
        assert_eq!(doc.get_annotation(c[0].as_u64().unwrap() as usize).unwrap().get_end() as u64, c[1].as_u64().unwrap());
    }
    for c in cases("AnnotationTest::testGetTag") {
        let doc = DocBlock::new(&sample);
        assert_eq!(doc.get_annotation(c[0].as_u64().unwrap() as usize).unwrap().get_tag().get_name(), s(&c[1]));
    }
    for c in cases("AnnotationTest::testRemove") {
        let doc = DocBlock::new(&sample);
        let a = doc.get_annotation(c[0].as_u64().unwrap() as usize).unwrap();
        a.remove();
        assert_eq!(a.get_content(), b"");
        assert_eq!(doc.get_line(c[1].as_u64().unwrap() as usize).unwrap().borrow().get_content(), b"");
        assert_eq!(doc.get_line(c[2].as_u64().unwrap() as usize).unwrap().borrow().get_content(), b"");
    }
    for c in cases("AnnotationTest::testRemoveEdgeCases") {
        let doc = DocBlock::new(&s(&c[1]));
        doc.get_annotation(0).unwrap().remove();
        assert_eq!(lossy(&doc.get_content()), lossy(&s(&c[0])));
    }
}

#[test]
fn annotation_types() {
    for c in cases("AnnotationTest::testTypeParsing") {
        assert_eq!(ann(&s(&c[1])).get_types().iter().map(|t| lossy(t)).collect::<Vec<_>>(), list(&c[0]).iter().map(|t| lossy(t)).collect::<Vec<_>>(), "{:?}", c[1]);
    }
    for c in cases("AnnotationTest::testTypes") {
        let line = Line::shared(s(&c[2]));
        let tag = Annotation::new(vec![(0, line.clone())], Rc::default());
        assert_eq!(tag.get_types(), list(&c[0]));
        tag.set_types(&list(&c[1]));
        assert_eq!(tag.get_types(), list(&c[1]));
        assert_eq!(lossy(line.borrow().get_content()), lossy(&s(&c[3])));
    }
    for c in cases("AnnotationTest::testNormalizedTypes") {
        assert_eq!(ann(&s(&c[1])).get_normalized_types(), list(&c[0]), "{:?}", c[1]);
    }
    let msg = panics(|| {
        ann(b" * @deprecated since Symfony 1.2").get_types();
    });
    assert!(msg.contains("This tag does not support types"));
    let msg = panics(|| ann(b" * @author Chuck Norris").set_types(&[b"string".to_vec()]));
    assert!(msg.contains("This tag does not support types"));
    assert!(Annotation::get_tags_with_types().iter().all(|t| !t.is_empty()));
}

#[test]
fn annotation_type_expression_and_variable_name() {
    for c in cases("AnnotationTest::testGetTypeExpression") {
        let a = Annotation::new(vec![(0, Line::shared(s(&c[0])))], ctx(c.get(1), c.get(2)));
        assert_eq!(a.get_type_expression().unwrap().get_common_type(), c[3].as_str().map(|s| s.as_bytes().to_vec()));
    }
    for c in cases("AnnotationTest::testGetVariableName") {
        assert_eq!(ann(&s(&c[0])).get_variable_name().map(|v| lossy(&v)), c[1].as_str().map(String::from), "{:?}", c[0]);
    }
    let doc = DocBlock::new(b"<?php\n/**\n * @param array{\n *        a: Foo,\n *        b: Bar\n * } $x\n */");
    assert_eq!(doc.get_annotation(0).unwrap().get_variable_name().unwrap(), b"$x");
}

// ---- TypeExpressionTest

fn check_inner_start_index(t: &TypeExpression) -> Vec<(usize, Vec<u8>)> {
    let mut res = vec![];
    for (start, inner) in t.inner_type_expressions() {
        let v = inner.to_string();
        assert_eq!(lossy(&t.to_string()[*start..(*start + v.len()).min(t.to_string().len())]), lossy(v));
        res.push((*start, v.to_vec()));
        res.extend(check_inner_start_index(inner));
    }
    res
}

fn parse(v: &[u8]) -> TypeExpression {
    let t = TypeExpression::new(v, Rc::default());
    check_inner_start_index(&t);
    t
}

#[test]
fn type_expression_get_types() {
    for c in cases("TypeExpressionTest::testGetTypes") {
        let value = s(&c[0]);
        let expected = c.get(1).filter(|v| !v.is_null()).map(list).unwrap_or_else(|| vec![value.clone()]);
        let e = parse(&value);
        assert_eq!(e.get_types().iter().map(|t| lossy(t)).collect::<Vec<_>>(), expected.iter().map(|t| lossy(t)).collect::<Vec<_>>(), "{}", lossy(&value));
        let ns = b"__UnionTest__";
        let union = parse(&[&ns[..], b"\\A|", &value, b"|", ns, b"\\Z"].concat());
        if !e.is_composite_type() || e.is_union_type() {
            let mut want = vec![[&ns[..], b"\\A"].concat()];
            want.extend(expected);
            want.push([&ns[..], b"\\Z"].concat());
            assert_eq!(union.get_types(), want);
        }
    }
}

#[test]
fn type_expression_parse_invalid() {
    for c in cases("TypeExpressionTest::testParseInvalidException") {
        let v = s(&c[0]);
        let r = TypeExpression::try_new(&v, Rc::default());
        assert!(r.is_err(), "should fail: {}", lossy(&v));
        assert!(r.unwrap_err().to_string().contains("Unable to parse phpdoc type"));
    }
}

#[test]
fn type_expression_huge_type() {
    let types: Vec<Vec<u8>> = (0..2000).map(|i| format!("\\X\\Foo{i}").into_bytes()).collect();
    let mut str = types.join(&b'|');
    let e = TypeExpression::plain(&str);
    assert_eq!(e.get_types(), types);
    for i in 0..100 {
        str = [&b"array"[..], if i % 2 == 1 { b"{" } else { b"<" }, &str, if i % 2 == 1 { b"}" } else { b">" }].concat();
    }
    let left = b"\\Closure(A|B): void".to_vec();
    let right = [&b"\\Closure("[..], &left, b"): void"].concat();
    let e = TypeExpression::plain(&[&left[..], b"|(", &str, b")|", &right].concat());
    assert_eq!(e.get_types(), vec![left, [&b"("[..], &str, b")"].concat(), right]);
}

#[test]
fn type_expression_predicates() {
    for c in cases("TypeExpressionTest::testGetTypesGlue") {
        assert_eq!(TypeExpression::plain(&s(&c[1])).get_types_glue(), c[0].as_str().map(|g| g.as_bytes()[0]));
    }
    for c in cases("TypeExpressionTest::testIsCompositeType") {
        assert_eq!(TypeExpression::plain(&s(&c[1])).is_composite_type(), c[0].as_bool().unwrap());
    }
    for c in cases("TypeExpressionTest::testIsUnionType") {
        assert_eq!(TypeExpression::plain(&s(&c[1])).is_union_type(), c[0].as_bool().unwrap());
    }
    for c in cases("TypeExpressionTest::testIsIntersectionType") {
        assert_eq!(TypeExpression::plain(&s(&c[1])).is_intersection_type(), c[0].as_bool().unwrap());
    }
    for c in cases("TypeExpressionTest::testGetCommonType") {
        let e = TypeExpression::new(&s(&c[0]), ctx(c.get(2), c.get(3)));
        assert_eq!(e.get_common_type().map(|t| lossy(&t)), c[1].as_str().map(String::from), "{:?}", c[0]);
    }
    for c in cases("TypeExpressionTest::testAllowsNull") {
        assert_eq!(TypeExpression::plain(&s(&c[0])).allows_null(), c[1].as_bool().unwrap(), "{:?}", c[0]);
    }
}

#[test]
fn type_expression_map_and_walk_types() {
    let te = TypeExpression::plain(br"Foo|Bar|($v is \Closure(X, Y): Z ? U : (V&W))");
    let add = |t: TypeExpression| {
        let v = t.to_string();
        if !v.starts_with(b"\\") && !v.starts_with(b"(") { TypeExpression::plain(&[b"\\", v].concat()) } else { t }
    };
    let remove = |t: TypeExpression| if let Some(r) = t.to_string().strip_prefix(b"\\") { TypeExpression::plain(r) } else { t };
    let mut log = vec![];
    te.map_types(&mut |t| {
        log.push(lossy(t.to_string()));
        if t.to_string() == b"Y" { TypeExpression::plain(b"_y_") } else { t }
    });
    assert_eq!(log, [
        "Foo", "Bar", "\\Closure", "X", "Y", "Z", "\\Closure(X, _y_): Z", "U", "V", "W", "V&W", "(V&W)", "($v is \\Closure(X, _y_): Z ? U : (V&W))",
        "Foo|Bar|($v is \\Closure(X, _y_): Z ? U : (V&W))",
    ]);
    let mut te = te;
    for (f, want) in [
        (true, r"\Foo|\Bar|($v is \Closure(\X, \Y): \Z ? \U : (\V&\W))"),
        (true, r"\Foo|\Bar|($v is \Closure(\X, \Y): \Z ? \U : (\V&\W))"),
        (false, r"Foo|Bar|($v is Closure(X, Y): Z ? U : (V&W))"),
        (false, r"Foo|Bar|($v is Closure(X, Y): Z ? U : (V&W))"),
        (true, r"\Foo|\Bar|($v is \Closure(\X, \Y): \Z ? \U : (\V&\W))"),
    ] {
        te = if f { te.map_types(&mut |t| add(t)) } else { te.map_types(&mut |t| remove(t)) };
        check_inner_start_index(&te);
        assert_eq!(lossy(te.to_string()), want);
    }

    let te = TypeExpression::plain(br"Foo|Bar|($v is \Closure(X, Y): Z ? U : (V&W))");
    let mut log = vec![];
    te.walk_types(&mut |t| log.push(lossy(t.to_string())));
    assert_eq!(log, [
        "Foo", "Bar", "\\Closure", "X", "Y", "Z", "\\Closure(X, Y): Z", "U", "V", "W", "V&W", "(V&W)", "($v is \\Closure(X, Y): Z ? U : (V&W))",
        "Foo|Bar|($v is \\Closure(X, Y): Z ? U : (V&W))",
    ]);
}

fn crc32(s: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in s {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

#[test]
fn type_expression_sort_types() {
    use pun_tokens::php_compat::strcasecmp;
    let mut case_fx = |a: &TypeExpression, b: &TypeExpression| strcasecmp(a.to_string(), b.to_string()).cmp(&0);
    let mut crc_fx = |a: &TypeExpression, b: &TypeExpression| crc32(a.to_string()).cmp(&crc32(b.to_string()));
    for c in cases("TypeExpressionTest::testSortTypes") {
        let e = parse(&s(&c[0])).sort_types(&mut case_fx);
        check_inner_start_index(&e);
        assert_eq!(lossy(e.to_string()), lossy(&s(&c[1])), "{:?}", c[0]);
        let e = e.sort_types(&mut crc_fx);
        check_inner_start_index(&e);
        let e = e.sort_types(&mut case_fx);
        check_inner_start_index(&e);
        assert_eq!(lossy(e.to_string()), lossy(&s(&c[1])));
    }
}

#[test]
fn type_regex_does_not_have_unnamed_capturing_group() {
    // PHP drops trailing unset groups from $matches; count only up to the last set one like upstream does.
    let m = preg::captures(&format!("~{}~", type_expression::REGEX_TYPES), b"int").unwrap();
    let last = m.len() - 1;
    let re = pcre2::bytes::Regex::new(type_expression::REGEX_TYPES).unwrap();
    let named = re.capture_names()[..=last].iter().filter(|n| n.is_some()).count();
    assert_eq!(named, last);
}

/// A use whose short name differs from the type must be skipped (expected values from PHP-CS-Fixer's TypeExpression).
#[test]
fn type_expression_common_type_with_unrelated_use() {
    let uses = vec![NamespaceUse { full_name: b"Bar\\Baz".to_vec(), short_name: b"Baz".to_vec() }];
    let ctx = Rc::new(Context { namespace: None, namespace_uses: uses });
    assert_eq!(TypeExpression::new(b"array|Traversable", ctx.clone()).get_common_type(), Some(b"iterable".to_vec()));
    assert_eq!(TypeExpression::new(b"array|Baz", ctx).get_common_type(), None);
}

/// Annotation::getTypes caches: editing the line afterwards doesn't change the reported types (checked against PHP).
#[test]
fn annotation_get_types_is_cached() {
    let doc = DocBlock::new(b"/**\n * @param int $a\n */");
    let a = &doc.get_annotations()[0];
    assert_eq!(a.get_types(), [b"int".to_vec()]);
    doc.get_line(1).unwrap().borrow_mut().set_content(b" * @param string $a\n".to_vec());
    assert_eq!(a.get_types(), [b"int".to_vec()]);
    assert_eq!(a.get_content(), b" * @param string $a\n");
}

#[test]
fn ascii_fast_paths_match_php() {
    // Expected values: PHP 8.5 preg_split / preg_match_all / preg_match with the upstream patterns on the same input.
    let split = |c: &str| super::split_lines_ascii(c.as_bytes()).map(|v| v.into_iter().map(|l| String::from_utf8(l).unwrap()).collect::<Vec<_>>());
    assert_eq!(split("/**\n * a\x0bb\r\n\x0c\n * c\n */").unwrap(), ["/**\n", " * a\x0bb\r\n\x0c\n", " * c\n", " */"]);
    assert_eq!(split("/** x */").unwrap(), ["/** x */"]);
    assert_eq!(split("/**\r\r\n *\n\n\n */\n").unwrap(), ["/**\r\r\n", " *\n\n\n", " */\n"]);
    assert_eq!(split("/**\n * @a\x0c").unwrap(), ["/**\n", " * @a\x0c"]);
    assert_eq!(split("\n/** */"), None);
    assert_eq!(split("/** \u{e9} */"), None);
    let tag = |c: &str| super::tag::tag_name_ascii(c.as_bytes()).map(|v| String::from_utf8(v).unwrap());
    for (c, name) in [
        (" * @param int $a", "param"),
        (" * @a@b c", "b"),
        (" * @foo.bar @baz", "baz"),
        (" * @foo", "foo"),
        (" * @-x\t", "-x"),
        (" * email@example.com", "other"),
        (" * @", "other"),
        (" * @@ @x\n", "x"),
    ] {
        assert_eq!(tag(c).unwrap(), name, "{c:?}");
    }
    assert_eq!(tag(" * @\u{e9}"), None);
    // Non-ASCII lines take the regex: no `@name` followed by whitespace or the end is upstream's "other".
    assert_eq!(Tag::new(Line::shared(" * @caf\u{e9}")).get_name(), b"other");
    assert_eq!(Tag::new(Line::shared(" * caf\u{e9} @see\u{a0}x")).get_name(), b"see");
    for (c, has) in [(" * @param", true), (" *\t\x0b@x", true), (" * x @y", false), ("**@", true), (" * text", false), (" */", false), ("*a@", false)] {
        assert_eq!(Line::new(c).contains_a_tag(), has, "{c:?}");
    }
    assert!(Line::new("*\u{a0}@").contains_a_tag());
}
