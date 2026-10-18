//! FuzzyRelativePathHelper over ParentDirectoryRelativePathHelper, the expected paths printed by PHPStan 2.2.7's
//! own helpers on the same working directory, analysed paths and files.

use pun_analyse::RelativePath;

#[test]
fn relative_paths_match_phpstan() {
    let cases: &[(&str, &[&str], &str, &str)] = &[
        (
            "/a/b",
            &["/a/b/src", "/a/b/tests"],
            "/a/b/src/X.php",
            "src/X.php",
        ),
        (
            "/a/b",
            &["/a/b/src", "/a/b/tests"],
            "/a/b/tests/Y.php",
            "tests/Y.php",
        ),
        (
            "/a/b",
            &["/a/b/src", "/a/b/tests"],
            "/a/c/Z.php",
            "../c/Z.php",
        ),
        ("/a/b", &["/a/b/src", "/a/b/tests"], "/x/y.php", "/x/y.php"),
        ("/a/b", &["/a/b/src/One.php"], "/a/b/src/One.php", "One.php"),
        (
            "/a/b",
            &["/a/b/src/One.php"],
            "/a/b/src/sub/Two.php",
            "sub/Two.php",
        ),
        (
            "/a/b",
            &["/a/b/src/One.php"],
            "/a/b/other/T.php",
            "other/T.php",
        ),
        (
            "/a/b",
            &["/c/d/e.php", "/c/d/f/g.php"],
            "/c/d/e.php",
            "/c/d/e.php",
        ),
        (
            "/a/b",
            &["/c/d/e.php", "/c/d/f/g.php"],
            "/c/d/f/g.php",
            "/c/d/f/g.php",
        ),
        (
            "/a/b",
            &["/c/d/e.php", "/c/d/f/g.php"],
            "/a/b/k.php",
            "k.php",
        ),
        ("/", &["/p/q/r.php"], "/p/q/r.php", "r.php"),
        ("/", &["/p/q/r.php"], "/p/s.php", "/p/s.php"),
        ("/a/b", &["/a/b"], "/a/b/x.php", "x.php"),
        ("/a/b", &["/a/b"], "/a/bc/x.php", "c/x.php"),
        ("/a/b", &[], "/a/b/x.php", "x.php"),
        ("/a/b", &[], "phar:///a/x.php", "../x.php"),
        ("/a/b", &[], "rel/x.php", "rel/x.php"),
    ];
    for (cwd, paths, file, expected) in cases {
        let paths: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
        assert_eq!(
            RelativePath::new(cwd, &paths).get(file),
            *expected,
            "{cwd} {paths:?} {file}"
        );
    }
}
