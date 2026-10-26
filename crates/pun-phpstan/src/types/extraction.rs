//! Type-owned getConstantStrings/getConstantArrays, including UnionType's domain-sensitive benevolent policy.

use super::*;

impl Type {
    /// `getConstantStrings()`: template subclasses inherit extraction from their concrete parent.
    pub fn constant_strings(&self) -> Vec<String> {
        match self.unreasoned() {
            Type::ConstString(s) => vec![s.clone()],
            Type::Template(template) => {
                let mut bound = &template.bound;
                while let Type::Template(template) = bound {
                    bound = &template.bound;
                    // TemplateTypeFactory selects TemplateStringType before TemplateConstantStringType
                    // when the bound is itself a string template. Its inherited extraction is empty.
                    if matches!(bound, Type::ConstString(_)) {
                        return vec![];
                    }
                }
                bound.constant_strings()
            }
            Type::Union(types) | Type::Benevolent(types) => {
                self.pick_constants(types, Type::constant_strings, Type::is_string)
            }
            Type::Intersection(types) => types.iter().flat_map(Type::constant_strings).collect(),
            _ => vec![],
        }
    }

    /// `getConstantArrays()`. A callable-array intersection owns exactly offsets 0 and 1, even when its
    /// array member is generic; each value comes from that offset, not the union of all array items.
    pub fn constant_arrays(&self) -> Vec<ConstArray> {
        match self.unreasoned() {
            _ if self.is_callable_array() => {
                let zero = self.callable_array_offset(0);
                let one = self.callable_array_offset(1);
                vec![ConstArray {
                    keys: vec![Type::ConstInt(0), Type::ConstInt(1)],
                    values: vec![
                        intersect(vec![
                            zero,
                            Type::Union(vec![Type::ClassString(None), Type::ObjectWithoutClass]),
                        ]),
                        intersect(vec![
                            one,
                            Type::Intersection(vec![
                                Type::String,
                                Type::Accessory(Accessory::NonFalsyString),
                            ]),
                        ]),
                    ],
                    next_auto_indexes: vec![2],
                    optional: vec![],
                    is_list: Tri::Yes,
                    unsealed: None,
                }]
            }
            Type::ConstArray(a) => vec![(**a).clone()],
            Type::Template(template) => template.bound.constant_arrays(),
            Type::Union(types) | Type::Benevolent(types) => {
                self.pick_constants(types, Type::constant_arrays, Type::is_array)
            }
            Type::Intersection(types) => types.iter().flat_map(Type::constant_arrays).collect(),
            _ => vec![],
        }
    }

    /// UnionType/BenevolentUnionType::pickFromTypes(). Only a benevolent union can skip empty results
    /// from members outside the requested domain; a broad string/array still cancels its extraction.
    fn pick_constants<T>(
        &self,
        types: &[Type],
        get: fn(&Type) -> Vec<T>,
        domain: fn(&Type) -> Tri,
    ) -> Vec<T> {
        let mut values = Vec::new();
        for ty in types {
            let inner = get(ty);
            if inner.is_empty()
                && (!matches!(self.unreasoned(), Type::Benevolent(_)) || domain(ty).yes())
            {
                return vec![];
            }
            values.extend(inner);
        }
        values
    }

    /// The constant integer getOffsetValueType() queries needed by callable-array synthesis. Callable
    /// and array accessories contribute mixed unless they own this exact offset's value.
    fn callable_array_offset(&self, index: i64) -> Type {
        let key = Type::ConstInt(index);
        match self.unreasoned() {
            Type::Array(k, v) => {
                if is_super_type_of(k, &key).no() {
                    Type::Error
                } else {
                    (**v).clone()
                }
            }
            Type::ConstArray(a) => {
                if let Some(i) = a.keys.iter().position(|k| *k == key) {
                    return a.values[i].clone();
                }
                match &a.unsealed {
                    Some((k, v)) if a.is_unsealed().yes() && !is_super_type_of(k, &key).no() => {
                        v.clone()
                    }
                    _ => Type::Error,
                }
            }
            Type::Template(t) => t.bound.callable_array_offset(index),
            Type::Accessory(Accessory::HasOffsetValue(k, v)) if **k == key => (**v).clone(),
            Type::Intersection(types) => intersect(
                types
                    .iter()
                    .map(|t| t.callable_array_offset(index))
                    .collect(),
            ),
            Type::Union(types) | Type::Benevolent(types) => {
                let values: Vec<_> = types
                    .iter()
                    .map(|t| t.callable_array_offset(index))
                    .filter(|t| *t != Type::Error)
                    .collect();
                if values.is_empty() {
                    Type::Error
                } else {
                    union(values)
                }
            }
            _ => mixed(),
        }
    }
}
