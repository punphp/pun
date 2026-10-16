//! Port of Mago's `final-controller`: `class UserController` → `final class UserController`.
//! Opt-in only (`Pun/final_controller`). Risky: a final class can no longer be extended or mocked.
//! Deviation from Mago: a class named exactly `Controller` is skipped, because Laravel <= 10 ships the base
//! controller as a concrete `class Controller extends BaseController` that every other controller extends;
//! so is a controller extended elsewhere in the same file.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct FinalController;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(FinalController)
}

impl Fixer for FinalController {
    fn name(&self) -> &'static str {
        "Pun/final_controller"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CLASS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // Classes extended in this file can't be final (other files can't be seen: hence risky).
        let extended: Vec<Vec<u8>> = (0..tokens.count())
            .filter(|&k| tokens[k].is_kind(T_EXTENDS))
            .filter_map(|k| {
                let mut n = tokens.get_next_meaningful_token(k)?;
                while let Some(m) = tokens.get_next_meaningful_token(n).filter(|&m| tokens[m].is_given_kind(&[T_NS_SEPARATOR, T_STRING])) {
                    n = m;
                }
                Some(tokens[n].get_content().to_vec())
            })
            .collect();
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_CLASS) {
                continue;
            }
            let Some(name) = tokens.get_next_meaningful_token(i).filter(|&n| tokens[n].is_kind(T_STRING)) else { continue };
            let n = tokens[name].get_content();
            if !n.ends_with(b"Controller") || n == b"Controller" || extended.iter().any(|e| e == n) {
                continue;
            }
            let mut start = i;
            let mut skip = false;
            while let Some(p) = tokens.get_prev_meaningful_token(start).filter(|&p| tokens[p].is_given_kind(&[T_ABSTRACT, T_FINAL, T_READONLY])) {
                skip |= !tokens[p].is_kind(T_READONLY);
                start = p;
            }
            if skip || tokens.get_prev_meaningful_token(start).is_some_and(|p| tokens[p].is_kind(T_NEW)) {
                continue;
            }
            tokens.insert_at(start, vec![Token::new(T_FINAL, "final"), Token::new(T_WHITESPACE, " ")]);
        }
    }
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        let mut tokens = pun_tokens::Tokens::from_code(code.as_bytes());
        super::fixer().fix(std::path::Path::new("t.php"), &mut tokens);
        String::from_utf8(tokens.generate_code()).unwrap()
    }

    #[test]
    fn mago_cases() {
        // adds_final_to_plain_controller, adds_final_before_existing_modifier, adds_final_after_attributes
        assert_eq!(fix("<?php\n\nclass UserController\n{\n}\n"), "<?php\n\nfinal class UserController\n{\n}\n");
        assert_eq!(fix("<?php\n\nreadonly class CachedController\n{\n}\n"), "<?php\n\nfinal readonly class CachedController\n{\n}\n");
        assert_eq!(fix("<?php\n\n#[Route]\nclass OrderController\n{\n}\n"), "<?php\n\n#[Route]\nfinal class OrderController\n{\n}\n");
        // skips_abstract_controller
        let same = "<?php\n\nabstract class BaseController\n{\n}\n";
        assert_eq!(fix(same), same);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php\nnamespace App\\Http\\Controllers;\n\n/** Doc */\nclass PostController extends Controller {}\n"),
            "<?php\nnamespace App\\Http\\Controllers;\n\n/** Doc */\nfinal class PostController extends Controller {}\n"
        );
        for same in [
            "<?php final class UserController {}",
            "<?php readonly final class UserController {}",
            "<?php abstract readonly class UserController {}",
            "<?php class Controller extends BaseController {}",
            "<?php class UserService {}",
            "<?php class Usercontroller {}",
            "<?php interface UserController {}",
            "<?php $x = UserController::class;",
            "<?php $c = new class extends Controller {};",
            "<?php class AController {}\nfinal class BController extends \\App\\AController {}\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
