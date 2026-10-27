<?php declare(strict_types = 1);

namespace ArrayFilterStrict;

/** @var list<int> $list */
$list = [1, 2, 3];

/** @var array<string, int> $array */
$array = ["a" => 1, "b" => 2, "c" => 3];

array_filter([1, 2, 3], function (int $value): bool {
	return $value > 1;
});

array_filter([1, 2, 3]);

array_filter([1, 2, 3], function (int $value): bool {
	return $value > 1;
}, ARRAY_FILTER_USE_KEY);

array_filter([1, 2, 3], function (int $value): int {
	return $value;
});

array_filter($list);
array_filter($array);

array_filter($array, null);

array_filter($list, 'intval');

/** @var bool $bool */
$bool = doFoo();
array_filter($list, foo() ? null : function (int $value): bool {
	return $value > 1;
});
-----
15: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
25: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
26: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
28: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
