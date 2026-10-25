<?php declare(strict_types=1);

namespace AllowedConstantBoundaries;

json_encode([], JSON_BIGINT_AS_STRING);
json_encode([], JSON_INVALID_UTF8_IGNORE | JSON_INVALID_UTF8_SUBSTITUTE);
array_unique([], SORT_REGULAR | SORT_NUMERIC);
json_encode([], JSON_HEX_AMP);
json_encode([], 2);
