//! `PhpCsFixer\DocBlock\TypeExpression`.

use crate::analyzers::preg::{Captures, Preg};
use std::sync::LazyLock;

pub const REGEX_IDENTIFIER: &str = crate::__regex_identifier!();
pub const REGEX_TYPES: &str = crate::__regex_types!();
const REGEX_TYPE: &str = crate::__regex_type!();
/// parse()'s and parseCommaSeparatedInnerTypes()'s patterns, built once (they run for every type in every docblock).
static PARSE_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r"{{\G{REGEX_TYPE}(?<glue_raw>\h*(?<glue>[|&])\h*(?!$)|$)}}"));
static COMMA_SEPARATED_PATTERN: LazyLock<String> =
    LazyLock::new(|| format!(r"{{\G{REGEX_TYPES}(?:\h*,[\h\v*]*|$)}}"));

#[derive(Debug)]
pub struct ParseError(pub Vec<u8>);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // `'Unable to parse phpdoc type '.var_export($this->value, true)`
        write!(
            f,
            "Unable to parse phpdoc type {}",
            String::from_utf8_lossy(&pun_tokens::php_compat::var_export_string(&self.0))
        )
    }
}

#[derive(Clone, Debug)]
pub struct TypeExpression {
    value: Vec<u8>,
    is_composite_type: bool,
    types_glue: Option<u8>,
    inner_type_expressions: Vec<(usize, TypeExpression)>,
}

impl TypeExpression {
    /// Panics like upstream throws on an unparsable type; see `try_new`.
    pub fn new(value: &[u8]) -> Self {
        Self::try_new(value).unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn try_new(value: &[u8]) -> Result<Self, ParseError> {
        let mut t = TypeExpression {
            value: value.to_vec(),
            is_composite_type: false,
            types_glue: None,
            inner_type_expressions: vec![],
        };
        t.parse()?;
        Ok(t)
    }

    pub fn to_string(&self) -> &[u8] {
        &self.value
    }

    pub fn get_types(&self) -> Vec<Vec<u8>> {
        if self.is_composite_type {
            return self
                .inner_type_expressions
                .iter()
                .map(|(_, e)| e.value.clone())
                .collect();
        }
        vec![self.value.clone()]
    }

    pub fn is_composite_type(&self) -> bool {
        self.is_composite_type
    }

    pub fn is_union_type(&self) -> bool {
        self.is_composite_type && self.types_glue == Some(b'|')
    }

    pub fn get_types_glue(&self) -> Option<u8> {
        self.types_glue
    }

    pub fn map_types(
        &self,
        callback: &mut dyn FnMut(TypeExpression) -> TypeExpression,
    ) -> TypeExpression {
        let mut value = self.value.clone();
        let mut start_index_offset: isize = 0;
        for (start_index_orig, inner) in &self.inner_type_expressions {
            let inner_value_orig = &inner.value;
            let inner = inner.map_types(callback);
            if &inner.value != inner_value_orig {
                let at = (*start_index_orig as isize + start_index_offset) as usize;
                value.splice(at..at + inner_value_orig.len(), inner.value.iter().copied());
                start_index_offset += inner.value.len() as isize - inner_value_orig.len() as isize;
            }
        }
        let t = if value == self.value {
            self.clone()
        } else {
            TypeExpression::new(&value)
        };
        callback(t)
    }

    fn parse(&mut self) -> Result<(), ParseError> {
        struct InnerValue {
            start_index: usize,
            value: Vec<u8>,
            next_glue: Option<Vec<u8>>,
            next_glue_raw: Option<Vec<u8>>,
        }
        let pattern: &str = &PARSE_PATTERN;
        let mut seen_glues: Option<[bool; 2]> = None; // ['|', '&'] in insertion order
        let mut inner_values: Vec<InnerValue> = vec![];
        let value = self.value.clone();
        let mut index = 0;
        let matches = loop {
            let Some(m) = Preg::match_at(pattern, &value, 0, index) else {
                return Err(ParseError(value));
            };
            let glue = m.name_str("glue");
            if seen_glues.is_none() {
                if glue.is_empty() {
                    break m;
                }
                seen_glues = Some([false, false]);
            }
            if !glue.is_empty() {
                seen_glues.as_mut().unwrap()[(glue == b"&") as usize] = true;
            }
            inner_values.push(InnerValue {
                start_index: index,
                value: m.name_str("type").to_vec(),
                next_glue: m.name_exists("glue").then(|| glue.to_vec()),
                next_glue_raw: m
                    .name_exists("glue_raw")
                    .then(|| m.name_str("glue_raw").to_vec()),
            });
            index += m.str(0).len();
            if value.len() <= index {
                let seen = seen_glues.unwrap();
                self.is_composite_type = true;
                self.types_glue = Some(if seen[0] { b'|' } else { b'&' });
                let glue = self.types_glue.unwrap();
                if seen[0] != seen[1] {
                    for iv in &inner_values {
                        let e = TypeExpression::try_new(&iv.value)?;
                        self.inner_type_expressions.push((iv.start_index, e));
                    }
                } else {
                    let mut i = 0;
                    while i < inner_values.len() {
                        let inner_start_index = inner_values[i].start_index;
                        let mut inner_value = vec![];
                        loop {
                            inner_value.extend_from_slice(&inner_values[i].value);
                            if inner_values[i].next_glue.as_deref().unwrap_or(&[glue]) == [glue] {
                                break;
                            }
                            inner_value.extend_from_slice(
                                inner_values[i].next_glue_raw.as_deref().unwrap_or(b""),
                            );
                            i += 1;
                        }
                        let e = TypeExpression::try_new(&inner_value)?;
                        self.inner_type_expressions.push((inner_start_index, e));
                        i += 1;
                    }
                }
                return Ok(());
            }
        };

        self.is_composite_type = false;
        let s = |n: &str| matches.name_str(n);
        let at0 = |n: &str| !matches.name_str(n).is_empty() && matches.name_offset(n) == 0;
        let typ = s("type");
        if !s("nullable").is_empty() {
            let n = s("nullable").len();
            self.push(n, &typ[n..])?;
        } else if !s("array").is_empty() {
            self.push(0, &typ[..typ.len() - s("array").len()])?;
        } else if at0("generic") {
            self.push(0, s("generic_name"))?;
            let start = s("generic_name").len() + s("generic_start").len();
            self.parse_comma_separated_inner_types(start, s("generic_types"))?;
        } else if at0("callable") {
            self.push(0, s("callable_name"))?;
            let start = s("callable_name").len() + s("callable_template_start").len();
            self.parse_callable_template_inner_types(start, s("callable_template_inners"))?;
            let start =
                s("callable_name").len() + s("callable_template").len() + s("callable_start").len();
            self.parse_callable_argument_types(start, s("callable_arguments"))?;
            let ret = s("callable_return");
            if !ret.is_empty() {
                self.push(self.value.len() - ret.len(), ret)?;
            }
        } else if at0("array_shape") {
            self.push(0, s("array_shape_name"))?;
            let mut next_index = s("array_shape_name").len() + s("array_shape_start").len();
            self.parse_array_shape_inner_types(next_index, s("array_shape_inners"))?;
            if !s("array_shape_unsealed_type").is_empty() {
                next_index += s("array_shape_inners").len()
                    + s("array_shape_unsealed_variadic").len()
                    + s("array_shape_unsealed_type_start").len();
                self.push(next_index, s("array_shape_unsealed_type_a"))?;
                if !s("array_shape_unsealed_type_b").is_empty() {
                    next_index += s("array_shape_unsealed_type_a").len()
                        + s("array_shape_unsealed_type_comma").len();
                    self.push(next_index, s("array_shape_unsealed_type_b"))?;
                }
            }
        } else if at0("parenthesized") {
            let mut index = s("parenthesized_start").len();
            if !s("conditional").is_empty() {
                if !s("conditional_cond_left_types").is_empty() {
                    self.push(index, s("conditional_cond_left_types"))?;
                }
                index += s("conditional_cond_left").len() + s("conditional_cond_middle").len();
                self.push(index, s("conditional_cond_right_types"))?;
                index +=
                    s("conditional_cond_right_types").len() + s("conditional_true_start").len();
                self.push(index, s("conditional_true_types"))?;
                index += s("conditional_true_types").len() + s("conditional_false_start").len();
                self.push(index, s("conditional_false_types"))?;
            } else {
                self.push(index, s("parenthesized_types"))?;
            }
        } else if !s("class_constant").is_empty() {
            self.push(0, s("class_constant_name"))?;
        }
        Ok(())
    }

    fn push(&mut self, start_index: usize, value: &[u8]) -> Result<(), ParseError> {
        let e = TypeExpression::try_new(value)?;
        self.inner_type_expressions.push((start_index, e));
        Ok(())
    }

    fn parse_comma_separated_inner_types(
        &mut self,
        start_index: usize,
        value: &[u8],
    ) -> Result<(), ParseError> {
        let pattern: &str = &COMMA_SEPARATED_PATTERN;
        let mut index = 0;
        while value.len() != index {
            let m = Preg::match_at(pattern, value, 0, index)
                .ok_or_else(|| ParseError(value.to_vec()))?;
            self.push(start_index + index, m.name_str("types"))?;
            index += m.str(0).len();
        }
        Ok(())
    }

    /// Shared loop of parseCallableTemplateInnerTypes / parseCallableArgumentTypes / parseArrayShapeInnerTypes.
    #[expect(
        clippy::too_many_arguments,
        reason = "the three upstream loops differ only in these regex pieces"
    )]
    fn parse_prefixed(
        &mut self,
        start_index: usize,
        value: &[u8],
        group: &str,
        sep: &str,
        prefix: &[u8],
        suffix: &[u8],
        mut each: impl FnMut(&Captures, usize, &mut Vec<(usize, Vec<u8>)>),
    ) -> Result<(), ParseError> {
        let pre = format!(r"{{\G(?:(?=1)0{REGEX_TYPES}|(?<_{group}>(?&{group})){sep})}}");
        let full = format!(r"{{^{REGEX_TYPES}$}}");
        let mut index = 0;
        while value.len() != index {
            let pm =
                Preg::match_at(&pre, value, 0, index).ok_or_else(|| ParseError(value.to_vec()))?;
            let consumed_value = pm.name_str(&format!("_{group}"));
            let consumed_value_length = consumed_value.len();
            let consumed_comma_length = pm.str(0).len() - consumed_value_length;
            let subject = [prefix, consumed_value, suffix].concat();
            let m =
                Preg::match_groups(&full, &subject, 0).ok_or_else(|| ParseError(value.to_vec()))?;
            let mut out = vec![];
            each(&m, start_index + index, &mut out);
            for (i, v) in out {
                self.push(i, &v)?;
            }
            index += consumed_value_length + consumed_comma_length;
        }
        Ok(())
    }

    fn parse_callable_template_inner_types(
        &mut self,
        start_index: usize,
        value: &[u8],
    ) -> Result<(), ParseError> {
        let p = b"Closure<".len();
        self.parse_prefixed(
            start_index,
            value,
            "callable_template_inner",
            r"(?:\h*,\h*|$)",
            b"Closure<",
            b">(): void",
            |m, base, out| {
                if !m.name_str("callable_template_inner_b").is_empty() {
                    let s = m.name_offset("callable_template_inner_b_types") as usize;
                    out.push((
                        base + s - p,
                        m.name_str("callable_template_inner_b_types").to_vec(),
                    ));
                }
                if !m.name_str("callable_template_inner_d").is_empty() {
                    let s = m.name_offset("callable_template_inner_d_types") as usize;
                    out.push((
                        base + s - p,
                        m.name_str("callable_template_inner_d_types").to_vec(),
                    ));
                }
            },
        )
    }

    fn parse_callable_argument_types(
        &mut self,
        start_index: usize,
        value: &[u8],
    ) -> Result<(), ParseError> {
        self.parse_prefixed(
            start_index,
            value,
            "callable_argument",
            r"(?:\h*,\h*|$)",
            b"Closure(",
            b"): void",
            |m, base, out| {
                out.push((base, m.name_str("callable_argument_type").to_vec()));
            },
        )
    }

    fn parse_array_shape_inner_types(
        &mut self,
        start_index: usize,
        value: &[u8],
    ) -> Result<(), ParseError> {
        let p = b"array{".len();
        self.parse_prefixed(
            start_index,
            value,
            "array_shape_inner",
            r"(?:\h*,[\h\v*]*|$)",
            b"array{",
            b"}",
            |m, base, out| {
                let s = m.name_offset("array_shape_inner_value") as usize;
                out.push((base + s - p, m.name_str("array_shape_inner_value").to_vec()));
            },
        )
    }

    #[cfg(test)]
    pub(crate) fn inner_type_expressions(&self) -> &[(usize, TypeExpression)] {
        &self.inner_type_expressions
    }
}
