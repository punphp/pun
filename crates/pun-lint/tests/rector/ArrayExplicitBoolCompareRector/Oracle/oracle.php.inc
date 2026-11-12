<?php

function arrayExplicitBoolCompare(array $items, $mixed, ?array $nullable, array|bool $union, string $s, iterable $it)
{
    if ($items) {
    } elseif (!$items) {
    }
    if ($items):
    elseif (! $items):
    endif;
    $r = $items ? 1 : 2;
    $r = !$items ? 1 : 2;
    $r = $items ?: 2;
    $r = (($items)) ? 1 : 2;
    $r = !($items) ? 1 : 2;
    $r = (!$items) ? 1 : 2;
    $r = !!$items ? 1 : 2;
    if ($mixed) {}
    if ($nullable) {}
    if ($union) {}
    if ($s) {}
    if ($it) {}
    if ($items && $s) {}
    while ($items) { break; }
    $local = [1];
    if ($local) {}
    $made = explode(',', $s);
    if (!$made) {}
    /** @var array $doc */
    $doc = $mixed;
    if ($doc) {}
    if (is_array($mixed)) {
        if ($mixed) {}
    }
    if (  ! /* c */ $items  ) {}
    $f = fn (array $a) => $a ? 1 : 2;
    $g = $items ? $items ? 1 : 2 : 3;
    $h = match (true) { default => $items ? 1 : 2 };
    if ($this ?? $items) {}
}

class ArrayExplicit
{
    private array $prop = [];
    public function run(array ...$rest)
    {
        if ($this->prop) {}
        if ($rest) {}
        if (!$rest) {}
    }
}
-----
<?php

function arrayExplicitBoolCompare(array $items, $mixed, ?array $nullable, array|bool $union, string $s, iterable $it)
{
    if ($items !== []) {
    } elseif ($items === []) {
    }
    if ($items !== []):
    elseif ($items === []):
    endif;
    $r = $items !== [] ? 1 : 2;
    $r = $items === [] ? 1 : 2;
    $r = $items ?: 2;
    $r = (($items !== [])) ? 1 : 2;
    $r = $items === [] ? 1 : 2;
    $r = ($items === []) ? 1 : 2;
    $r = !!$items ? 1 : 2;
    if ($mixed) {}
    if ($nullable) {}
    if ($union) {}
    if ($s) {}
    if ($it) {}
    if ($items && $s) {}
    while ($items) { break; }
    $local = [1];
    if ($local !== []) {}
    $made = explode(',', $s);
    if ($made === []) {}
    /** @var array $doc */
    $doc = $mixed;
    if ($doc) {}
    if (is_array($mixed)) {
        if ($mixed !== []) {}
    }
    if (  $items === []  ) {}
    $f = fn (array $a) => $a !== [] ? 1 : 2;
    $g = $items !== [] ? $items !== [] ? 1 : 2 : 3;
    $h = match (true) { default => $items !== [] ? 1 : 2 };
    if ($this ?? $items) {}
}

class ArrayExplicit
{
    private array $prop = [];
    public function run(array ...$rest)
    {
        if ($this->prop) {}
        if ($rest !== []) {}
        if ($rest === []) {}
    }
}
