<?php

namespace CoverageFilterVar;

function check(mixed $value, int $unknown, string $text): void
{
	filter_var($value);
	filter_var($value, FILTER_VALIDATE_INT, (FILTER_NULL_ON_FAILURE | FILTER_THROW_ON_FAILURE));
	filter_var($value, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE | 268435456);
	filter_var($value, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE | NOPE_CONSTANT);
	filter_var($value, FILTER_VALIDATE_INT, ['options' => []]);
	filter_var($value, FILTER_VALIDATE_INT, ['options' => [], 'flags' => FILTER_NULL_ON_FAILURE]);
	filter_var($value, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE | -1);
	$null = FILTER_NULL_ON_FAILURE;
	filter_var($value, FILTER_VALIDATE_INT, $null);
	filter_var($value, FILTER_VALIDATE_INT, $unknown);
	filter_var($value, FILTER_VALIDATE_INT, $text);
	filter_var($value, FILTER_VALIDATE_INT, 0x7fffffffffffffff);
	filter_var($value, FILTER_VALIDATE_INT, FILTER_THROW_ON_FAILURE | 134217728);
}

function excludedElse(mixed $value): void
{
	if (true) {
	} else {
		filter_var($value, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE | FILTER_THROW_ON_FAILURE);
		filter_var($value, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE);
	}
}
