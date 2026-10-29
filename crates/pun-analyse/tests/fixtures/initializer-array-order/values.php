<?php
namespace InitializerArrayOrder;

use function PHPStan\dumpType;

const GLOBAL_EXPRESSION = true ? ['z' => 1, 'a' => 2] : ['m' => 3, 'b' => 4];

class Values {
    const EXPRESSION = true ? ['z' => 1, 'a' => 2] : ['m' => 3, 'b' => 4];
    const FALSE_CONDITION = false ? ['z' => 1, 'a' => 2] : ['m' => 3, 'b' => 4];
    const SAME_KEYS = true ? ['z' => 1, 'a' => 2] : ['a' => 20, 'z' => 10];
    const SAME_VALUES = true ? ['z' => 1, 'a' => 2] : ['a' => 2, 'z' => 1];
    const SUBSET = true ? ['z' => 1, 'a' => 2] : ['a' => 2];
    const EMPTY_THEN = true ? [] : ['z' => 1, 'a' => 2];
    const SHORT = ['z' => 1, 'a' => 2] ?: ['m' => 3, 'b' => 4];
    const UNION_EXPRESSION = ['z' => 1, 'a' => 2] + [];
    const UNION_RIGHT = [] + ['z' => 1, 'a' => 2];
    const UNION_BOTH = ['z' => 1, 'a' => 2] + ['m' => 3, 'b' => 4];
    const UNION_DUPLICATE = ['z' => ['left' => 1], 'a' => 2] + ['m' => 3, 'z' => ['right' => 9], 'b' => 4];
    const BASE = ['z' => ['y' => 1, 'b' => 2], 'a' => 3];
    const UNION_ALIAS = self::BASE + ['m' => ['z' => 4, 'a' => 5]];
    const UNION_OPTIONAL = self::SUBSET + ['q' => 5];
    const UNION_NUMERIC = [2 => 1, 0 => 2] + [];
    const NESTED = ['z' => (true ? ['y' => 1, 'b' => 2] : ['x' => 3, 'a' => 4]), 'a' => ['y' => 5, 'b' => 6] + []];
    const NESTED_NUMBER = ['z' => 1 + 2, 'a' => 2] + [];
    const UNKNOWN = strlen('x');
    const UNKNOWN_CONDITIONAL = true ? strlen('x') : ['z' => 1, 'a' => 2];
    const UNKNOWN_VALUE = ['z' => strlen('x'), 'a' => 2] + [];
    const NUMBER = 1 + 2;
    const CONDITIONAL_COPY = self::EXPRESSION;
    const UNION_COPY = self::UNION_DUPLICATE;
    const GLOBAL_COPY = GLOBAL_EXPRESSION;
    const CHOICE = true ? 'a' : 'b';
    const FINITE_KEY = [self::CHOICE => 1, 'z' => 2];
}

dumpType(Values::EXPRESSION, Values::FALSE_CONDITION);
dumpType(Values::SAME_KEYS, Values::SAME_VALUES, Values::SUBSET);
dumpType(Values::EMPTY_THEN, Values::SHORT);
dumpType(Values::UNION_EXPRESSION, Values::UNION_RIGHT, Values::UNION_BOTH);
dumpType(Values::UNION_DUPLICATE, Values::UNION_ALIAS);
dumpType(Values::UNION_OPTIONAL, Values::UNION_NUMERIC);
dumpType(Values::NESTED, Values::NESTED_NUMBER);
dumpType(Values::UNKNOWN, Values::UNKNOWN_CONDITIONAL, Values::UNKNOWN_VALUE, Values::NUMBER);
dumpType(Values::CONDITIONAL_COPY, Values::UNION_COPY, Values::GLOBAL_COPY);
dumpType(Values::FINITE_KEY);
