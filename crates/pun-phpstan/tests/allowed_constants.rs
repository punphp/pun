//! ParameterAllowedConstantsTest.php's reflection contract, from PHPStan 2.3.1.

use pun_phpstan::symbols::{ConstantReflection, PhpVersion, symbols};

#[test]
fn reflected_policies_preserve_groups_and_are_not_invented_for_unmapped_parameters() {
    for version in [PhpVersion::Php84, PhpVersion::Php85] {
        let symbols = symbols(version);
        let html = symbols.functions["htmlspecialchars"].variants[0].params[1]
            .allowed_constants
            .as_ref()
            .unwrap();
        assert!(html.bitmask);
        assert_eq!(
            html.exclusive_groups,
            [
                vec!["ENT_COMPAT", "ENT_QUOTES", "ENT_NOQUOTES"],
                vec!["ENT_HTML401", "ENT_XML1", "ENT_XHTML", "ENT_HTML5"],
            ]
        );
        let constants: Vec<_> = ["ENT_QUOTES", "ENT_NOQUOTES", "ENT_HTML401", "ENT_HTML5"]
            .into_iter()
            .map(|name| ConstantReflection {
                name: name.into(),
                builtin: true,
            })
            .collect();
        let result = html.check(&constants);
        assert!(result.disallowed.is_empty());
        assert_eq!(
            result.violated_groups,
            [
                vec!["ENT_QUOTES", "ENT_NOQUOTES"],
                vec!["ENT_HTML401", "ENT_HTML5"]
            ]
        );
        assert!(!result.bitmask_not_allowed);
        let single = symbols.functions["array_unique"].variants[0].params[1]
            .allowed_constants
            .as_ref()
            .unwrap();
        assert!(!single.bitmask);
        let constants: Vec<_> = ["SORT_REGULAR", "SORT_NUMERIC"]
            .into_iter()
            .map(|name| ConstantReflection {
                name: name.into(),
                builtin: true,
            })
            .collect();
        let result = single.check(&constants[..1]);
        assert!(result.disallowed.is_empty());
        assert_eq!(result.violated_groups, [] as [std::vec::Vec<&str>; 0]);
        assert!(!result.bitmask_not_allowed);
        let result = single.check(&constants);
        assert!(result.disallowed.is_empty());
        assert_eq!(result.violated_groups, [] as [std::vec::Vec<&str>; 0]);
        assert!(result.bitmask_not_allowed);
        let mut reordered = html.clone();
        reordered.constants.swap(0, 1);
        assert_ne!(*html, reordered);
        reordered = html.clone();
        reordered.exclusive_groups.swap(0, 1);
        assert_ne!(*html, reordered);
        reordered = html.clone();
        reordered.bitmask = false;
        assert_ne!(*html, reordered);
        assert!(
            symbols.functions["strlen"].variants[0].params[0]
                .allowed_constants
                .is_none()
        );
        assert!(
            symbols.classes["attribute"].methods["__construct"].variants[0].params[0]
                .allowed_constants
                .is_none()
        );
    }
}
