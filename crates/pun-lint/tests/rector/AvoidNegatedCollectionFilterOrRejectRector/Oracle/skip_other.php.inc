<?php

namespace App;

use Illuminate\Support\Collection;

$method = 'filter';
(new Collection([0, 1, null, -1]))
    ->$method(fn (?int $number): bool => ! $number);

(new Collection([0, 1, null, -1]))
    ->map(fn (?int $number): bool => ! $number);
