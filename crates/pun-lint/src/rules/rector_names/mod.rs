//! Rector 2.6.1's name handling, shared by the rules that rename or add classes: php-parser's view of a file's names
//! ([`view`]), how a doc comment's names resolve ([`resolve`]) and are rewritten ([`doc`]), the post rectors that
//! import names and add and remove `use` statements ([`post`]), and php-parser's printer of a changed statement list
//! ([`differ`]).

pub(crate) mod differ;
pub(crate) mod doc;
pub(crate) mod post;
pub(crate) mod resolve;
pub(crate) mod view;
