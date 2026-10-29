//! PHPStan 2.3.1 ExpressionResult: own raw returns and effective value reads are separate.

use std::rc::Rc;

use mago_codex::ttype::{atomic::TAtomic, union::TUnion};
use mago_span::HasSpan;
use mago_syntax::cst::{Call, Expression};

use super::{AnalysisArtifacts, get_expression_range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnExpressionType {
    pub raw: Option<Rc<TUnion>>,
    pub raw_native: Option<Rc<TUnion>>,
    pub projects_void: bool,
    pub holder: Option<Rc<TUnion>>,
    pub native_holder: Option<Rc<TUnion>>,
    pub resolved: bool,
    pub overridden: bool,
}

/// An owned result from one evaluation, suitable for transport without re-running its effects.
/// Native types are independent and partial: None must not be replaced with the full type.
#[derive(Debug, Clone)]
pub struct ExpressionResult {
    raw: Rc<TUnion>,
    value: Rc<TUnion>,
    raw_native: Option<Rc<TUnion>>,
    native: Option<Rc<TUnion>>,
    projects_void: bool,
    resolved: bool,
    overridden: bool,
    holder: Option<Rc<TUnion>>,
    native_holder: Option<Rc<TUnion>>,
}

impl ExpressionResult {
    /// A synthetic non-call state read has no independent native result or call-return provenance.
    pub(crate) fn from_value(value: TUnion) -> Self {
        let value = Rc::new(value);
        Self {
            raw: Rc::clone(&value),
            value,
            raw_native: None,
            native: None,
            projects_void: false,
            resolved: false,
            overridden: false,
            holder: None,
            native_holder: None,
        }
    }

    pub fn is_eager(&self) -> bool {
        !self.resolved
    }

    pub fn has_value_override(&self) -> bool {
        self.overridden
    }

    pub fn get_type(&self) -> &TUnion {
        &self.value
    }

    pub fn get_raw_type(&self) -> &TUnion {
        &self.raw
    }

    /// ExpressionResult::getKeepVoidType, not Scope's containsNull-gated wrapper.
    pub fn get_keep_void_type(&self) -> &TUnion {
        if may_be_void(&self.raw) { &self.raw } else { &self.value }
    }

    pub fn get_native_type(&self) -> Option<&TUnion> {
        self.native.as_deref()
    }

    pub fn get_raw_native_type(&self) -> Option<&TUnion> {
        self.raw_native.as_deref()
    }

    pub fn get_keep_void_native_type(&self) -> Option<&TUnion> {
        let raw = self.get_raw_native_type()?;
        if may_be_void(raw) { Some(raw) } else { self.get_native_type() }
    }

    /// ExpressionResult::getTypeOnScope. Callers supply the asking scope's authoritative
    /// holder and resolver-extension answer (not the cached override at the original position).
    /// Eager results ignore holders. Unlike get_type(), even the eager fallback is projected.
    pub fn get_type_on_scope(&self, holder: Option<&TUnion>, extension: Option<&TUnion>) -> Rc<TUnion> {
        if let Some(extension) = extension {
            return Rc::new(extension.clone());
        }
        let raw = if self.resolved {
            holder.map_or_else(|| Rc::clone(&self.raw), |ty| Rc::new(ty.clone()))
        } else {
            Rc::clone(&self.raw)
        };
        project_void(raw, self.projects_void)
    }

    pub fn get_native_type_on_scope(&self, holder: Option<&TUnion>, extension: Option<&TUnion>) -> Option<Rc<TUnion>> {
        extension
            .or(if self.resolved { holder } else { None })
            .cloned()
            .map(Rc::new)
            .or_else(|| self.raw_native.clone())
    }

    /// ExpressionResult::atAskPosition: a scope-owned answer becomes eager, while a
    /// position-independent result retains its own raw type. The native holder is independent.
    /// Resolver-extension answers must be recomputed by the caller at the asking position.
    pub fn at_ask_position(
        &self,
        holder: Option<(&TUnion, Option<&TUnion>)>,
        extension: Option<&TUnion>,
        native_extension: Option<&TUnion>,
    ) -> Self {
        let mut result = if self.resolved
            && let Some((full, native)) = holder
        {
            let raw = Rc::new(full.clone());
            let native = native.cloned().map(Rc::new);
            Self {
                raw: Rc::clone(&raw),
                value: raw,
                raw_native: native.clone(),
                native,
                projects_void: self.projects_void,
                resolved: false,
                overridden: false,
                holder: None,
                native_holder: None,
            }
        } else {
            Self {
                value: if self.resolved {
                    project_void(Rc::clone(&self.raw), self.projects_void)
                } else {
                    Rc::clone(&self.raw)
                },
                native: self.raw_native.clone(),
                overridden: false,
                holder: None,
                native_holder: None,
                ..self.clone()
            }
        };
        if let Some(extension) = extension {
            result.value = Rc::new(extension.clone());
            result.overridden = true;
        }
        if let Some(extension) = native_extension {
            result.native = Some(Rc::new(extension.clone()));
        }
        result
    }
}

// PHPStan's !isVoid()->no(), including mixed and template bounds, not just a literal void.
fn may_be_void(ty: &TUnion) -> bool {
    ty.types.iter().any(|atomic| match atomic {
        TAtomic::Void => true,
        TAtomic::Mixed(mixed) => {
            !mixed.effective_subtracted_type().is_some_and(|excluded| definitely_accepts_void(&excluded))
        }
        TAtomic::GenericParameter(parameter) => may_be_void(&parameter.constraint),
        _ => false,
    })
}

// MixedType::isVoid excludes void only when the subtraction definitely contains it.
// A template bounded by mixed can contain void, but is not definitely its supertype.
fn definitely_accepts_void(ty: &TUnion) -> bool {
    ty.types.iter().any(|atomic| match atomic {
        TAtomic::Void => true,
        TAtomic::Mixed(mixed) => !mixed.effective_subtracted_type().is_some_and(|excluded| may_be_void(&excluded)),
        _ => false,
    })
}

// VoidToNullTraverser visits union alternatives, not nested array/callable value types.
fn project_void(raw: Rc<TUnion>, enabled: bool) -> Rc<TUnion> {
    if !enabled || !raw.types.iter().any(TAtomic::is_void) {
        return raw;
    }
    let mut types: Vec<_> = raw
        .types
        .iter()
        .map(|atomic| if matches!(atomic, TAtomic::Void) { TAtomic::Null } else { atomic.clone() })
        .collect();
    types.sort_unstable();
    types.dedup();
    Rc::new(raw.clone_with_types(types))
}

impl AnalysisArtifacts {
    pub(crate) fn begin_expression_result(
        &mut self,
        expression: &Expression<'_>,
        holder: Option<Rc<TUnion>>,
        native_holder: Option<Rc<TUnion>>,
    ) {
        let Expression::Call(call) = expression else { return };
        let projects_void =
            !matches!(call, Call::Function(call) if !matches!(call.function, Expression::Identifier(_)));
        let range = get_expression_range(expression);
        self.remove_expression_type(&range);
        self.own_expression_type_checkpoints.insert(
            &mut self.own_expression_types,
            range,
            OwnExpressionType {
                raw: None,
                raw_native: None,
                projects_void,
                holder,
                native_holder,
                resolved: false,
                overridden: false,
            },
        );
    }

    /// Publish the raw return of normal call resolution, including dynamic return providers.
    /// Source syntax, not InvocationTarget identity, selects projection. Eager hook values use
    /// set_expression_type instead. The pre-evaluation holder wins without skipping evaluation.
    pub(crate) fn set_resolved_expression_type<T: HasSpan>(&mut self, expression: &T, raw: TUnion) {
        let range = get_expression_range(expression);
        let raw = Rc::new(raw);
        let mut own = self.own_expression_types.get(&range).cloned().unwrap_or(OwnExpressionType {
            raw: None,
            raw_native: None,
            projects_void: false,
            holder: None,
            native_holder: None,
            resolved: false,
            overridden: false,
        });
        own.raw = Some(Rc::clone(&raw));
        own.resolved = true;
        own.overridden = false;
        let value = own.holder.clone().unwrap_or_else(|| project_void(raw, own.projects_void));
        self.own_expression_type_checkpoints.insert(&mut self.own_expression_types, range, own);
        self.expression_type_checkpoints.insert(&mut self.expression_types, range, value);
        self.remove_native_expression_type(&range);
    }

    pub(crate) fn set_resolved_native_expression_type<T: HasSpan>(&mut self, expression: &T, raw: Rc<TUnion>) {
        let range = get_expression_range(expression);
        let mut value = Rc::clone(&raw);
        if let Some(mut own) = self.own_expression_types.get(&range).cloned() {
            own.raw_native = Some(raw);
            value = own.native_holder.clone().unwrap_or(value);
            self.own_expression_type_checkpoints.insert(&mut self.own_expression_types, range, own);
        }
        self.set_rc_native_expression_type(expression, value);
    }

    /// The result's own full type, ignoring holders and expression overrides.
    pub fn get_raw_expression_type<T: HasSpan>(&self, expression: &T) -> Option<&TUnion> {
        self.own_expression_types
            .get(&get_expression_range(expression))
            .and_then(|own| own.raw.as_deref())
            .or_else(|| self.get_expression_type(expression))
    }

    /// ExpressionResult::getKeepVoidType. A non-void raw result uses the effective value.
    pub fn get_keep_void_expression_type<T: HasSpan>(&self, expression: &T) -> Option<&TUnion> {
        let raw = self.get_raw_expression_type(expression)?;
        if may_be_void(raw) { Some(raw) } else { self.get_expression_type(expression) }
    }

    pub fn get_expression_result<T: HasSpan>(&self, expression: &T) -> Option<ExpressionResult> {
        let value = Rc::clone(self.get_rc_expression_type(expression)?);
        let own = self.own_expression_types.get(&get_expression_range(expression));
        Some(ExpressionResult {
            raw: own.and_then(|own| own.raw.clone()).unwrap_or_else(|| Rc::clone(&value)),
            value,
            raw_native: self.native_expression_types.as_ref().and_then(|_| match own {
                Some(own) => own.raw_native.clone(),
                None => self.get_rc_native_expression_type(expression).cloned(),
            }),
            native: self.get_rc_native_expression_type(expression).cloned(),
            projects_void: own.is_some_and(|own| own.projects_void),
            resolved: own.is_some_and(|own| own.resolved),
            overridden: own.is_some_and(|own| own.overridden),
            holder: own.and_then(|own| own.holder.clone()),
            native_holder: own.and_then(|own| own.native_holder.clone()),
        })
    }

    /// Restore all result lanes together, without treating a projected value as a new eager raw type.
    pub fn set_expression_result<T: HasSpan>(&mut self, expression: &T, result: ExpressionResult) {
        let range = get_expression_range(expression);
        if self.native_expression_types.is_none() && (result.raw_native.is_some() || result.native.is_some()) {
            let mut native = super::NativeExpressionTypes::default();
            for _ in 0..self.expression_type_checkpoints.depth() {
                native.checkpoints.begin_checkpoint();
            }
            self.native_expression_types = Some(native);
        }
        self.own_expression_type_checkpoints.insert(
            &mut self.own_expression_types,
            range,
            OwnExpressionType {
                raw: Some(result.raw),
                raw_native: result.raw_native,
                projects_void: result.projects_void,
                holder: result.holder,
                native_holder: result.native_holder,
                resolved: result.resolved,
                overridden: result.overridden,
            },
        );
        self.expression_type_checkpoints.insert(&mut self.expression_types, range, result.value);
        self.remove_native_expression_type(&range);
        if let (Some(native), Some(types)) = (result.native, &mut self.native_expression_types) {
            types.checkpoints.insert(&mut types.types, range, native);
        }
    }

    pub(crate) fn transfer_expression_results(&mut self, child: &mut Self) {
        for (range, own) in child.own_expression_types.drain() {
            self.own_expression_type_checkpoints.insert(&mut self.own_expression_types, range, own);
        }
    }
}

#[cfg(test)]
mod tests;
