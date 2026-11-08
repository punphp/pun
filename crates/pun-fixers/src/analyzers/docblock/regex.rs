// Generated from PhpCsFixer\DocBlock\TypeExpression constants (v3.95.17) via reflection; do not hand-edit.
// Crate-local macros (`#[macro_use] mod regex` in `docblock/mod.rs`), so `concat!` can splice them.

/// `TypeExpression::REGEX_IDENTIFIER`.
macro_rules! __regex_identifier {
    () => {
        r"(?:(?!(?<!\*)\d)[^\x00-\x2f\x3a-\x40\x5b-\x5e\x60\x7b-\x7f]++)"
    };
}

/// `TypeExpression::REGEX_TYPE` (private upstream).
macro_rules! __regex_type { () => { r#"(?<type>(?x) # single type
            (?:co(?:ntra)?variant\h+)?
            (?<nullable>\??\h*)
            (?:
                (?<array_shape>
                    (?<array_shape_name>(?i)(?:array|list|object)(?-i))
                    (?<array_shape_start>\h*\{[\h\v*]*)
                    (?<array_shape_inners>
                        (?<array_shape_inner>
                            (?<array_shape_inner_key>(?:(?&constant)|(?&identifier)|(?&name))\h*\??\h*:\h*|)
                            (?<array_shape_inner_value>(?&types_inner))
                        )
                        (?:
                            \h*,[\h\v*]*
                            (?&array_shape_inner)
                        )*+
                        (?:\h*,|(?!(?&array_shape_unsealed_variadic)))
                    |)
                    (?<array_shape_unsealed> # unsealed array shape, e.g. `...`. `...<string>`
                        (?<array_shape_unsealed_variadic>\h*\.\.\.)
                        (?<array_shape_unsealed_type>
                            (?<array_shape_unsealed_type_start>\h*<\h*)
                            (?<array_shape_unsealed_type_a>(?&types_inner))
                            (?:
                                (?<array_shape_unsealed_type_comma>\h*,\h*)
                                (?<array_shape_unsealed_type_b>(?&array_shape_unsealed_type_a))
                            |)
                            \h*>
                        |)
                    |)
                    [\h\v*]*\}
                )
                |
                (?<callable> # callable syntax, e.g. `callable(string, int...): bool`, `\Closure<T>(T, int): T`
                    (?<callable_name>(?&name))
                    (?<callable_template>
                        (?<callable_template_start>\h*<\h*)
                        (?<callable_template_inners>
                            (?<callable_template_inner>
                                (?<callable_template_inner_name>
                                    (?&identifier)
                                )
                                (?<callable_template_inner_b> # template bound
                                    \h+(?i)(?<callable_template_inner_b_kw>of|as)(?-i)\h+
                                    (?<callable_template_inner_b_types>(?&types_inner))
                                |)
                                (?<callable_template_inner_d> # template default
                                    \h*=\h*
                                    (?<callable_template_inner_d_types>(?&types_inner))
                                |)
                            )
                            (?:
                                \h*,\h*
                                (?&callable_template_inner)
                            )*+
                        )
                        \h*>
                        (?=\h*\()
                    |)
                    (?<callable_start>\h*\(\h*)
                    (?<callable_arguments>
                        (?<callable_argument>
                            (?<callable_argument_type>(?&types_inner))
                            (?<callable_argument_is_reference>\h*&|)
                            (?<callable_argument_is_variadic>\h*\.\.\.|)
                            (?<callable_argument_name>\h*\$(?&identifier)|)
                            (?<callable_argument_is_optional>\h*=|)
                        )
                        (?:
                            \h*,\h*
                            (?&callable_argument)
                        )*+
                        (?:\h*,)?
                    |)
                    \h*\)
                    (?:
                        \h*\:\h*
                        (?<callable_return>(?&type))
                    )?
                )
                |
                (?<generic> # generic syntax, e.g.: `array<int, \Foo\Bar>`
                    (?<generic_name>(?&name))
                    (?<generic_start>\h*<[\h\v*]*)
                    (?<generic_types>
                        (?&types_inner)
                        (?:
                            \h*,[\h\v*]*
                            (?&types_inner)
                        )*+
                        (?:\h*,)?
                    )
                    [\h\v*]*>
                )
                |
                (?<class_constant> # class constants with optional wildcard, e.g.: `Foo::*`, `Foo::CONST_A`, `FOO::CONST_*`
                    (?<class_constant_name>(?&name))
                    ::\*?(?:(?&identifier)\*?)*
                )
                |
                (?<constant> # single constant value (case insensitive), e.g.: 1, -1.8E+6, `'a'`
                    (?i)
                    # all sorts of numbers: with or without sign, supports literal separator and several numeric systems,
                    # e.g.: 1, +1.1, 1., .1, -1, 123E+8, 123_456_789, 0x7Fb4, 0b0110, 0o777
                    [+-]?(?:
                        (?:0b[01]++(?:_[01]++)*+)
                        | (?:0o[0-7]++(?:_[0-7]++)*+)
                        | (?:0x[\da-f]++(?:_[\da-f]++)*+)
                        | (?:(?<constant_digits>\d++(?:_\d++)*+)|(?=\.\d))
                          (?:\.(?&constant_digits)|(?<=\d)\.)?+
                          (?:e[+-]?(?&constant_digits))?+
                    )
                    | '(?:[^'\\]|\\.)*+'
                    | "(?:[^"\\]|\\.)*+"
                    (?-i)
                )
                |
                (?<this> # self reference, e.g.: $this, $self, @static
                    (?i)
                    [@$](?:this | self | static)
                    (?-i)
                )
                |
                (?<name> # full name, e.g.: `int`, `\DateTime`, `\Foo\Bar`, `positive-int`
                    \\?+
                    (?<identifier>(?:(?!(?<!\*)\d)[^\x00-\x2f\x3a-\x40\x5b-\x5e\x60\x7b-\x7f]++))
                    (?:[\\\-](?&identifier))*+
                )
                |
                (?<parenthesized> # parenthesized type, e.g.: `(int)`, `(int|\stdClass)`
                    (?<parenthesized_start>
                        \(\h*
                    )
                    (?:
                        (?<parenthesized_types>
                            (?&types_inner)
                        )
                        |
                        (?<conditional> # conditional type, e.g.: `$foo is \Throwable ? false : $foo`
                            (?<conditional_cond_left>
                                (?:\$(?&identifier))
                                |
                                (?<conditional_cond_left_types>(?&types_inner))
                            )
                            (?<conditional_cond_middle>
                                \h+(?i)is(?:\h+not)?(?-i)\h+
                            )
                            (?<conditional_cond_right_types>(?&types_inner))
                            (?<conditional_true_start>\h*\?\h*)
                            (?<conditional_true_types>(?&types_inner))
                            (?<conditional_false_start>\h*:\h*)
                            (?<conditional_false_types>(?&types_inner))
                        )
                    )
                    \h*\)
                )
            )
            (?<array> # array, e.g.: `string[]`, `array<int, string>[][]`
                (\h*\[\h*\])*
            )
            (?:(?=1)0
                (?<types_inner>(?>
                    (?&type)
                    (?:
                        \h*[|&]\h*
                        (?&type)
                    )*+
                ))
            |)
        )"# } }

/// `TypeExpression::REGEX_TYPES`.
macro_rules! __regex_types {
    () => {
        concat!(
            "(?<types>(?x) # one or several types separated by `|` or `&`\n",
            __regex_type!(),
            r#"
        (?:
            \h*(?<glue>[|&])\h*
            (?&type)
        )*+
    )"#
        )
    };
}
