<?php

function coalesceOracle($values, $key, $other, $f)
{
    $r = array_key_exists($key, $values) ? $values[$key] : NULL;
    $r = \array_key_exists($key, $values) ? $values[$key] : \null;
    $r = ARRAY_KEY_EXISTS($key, $values) ? ($values[$key]) : null;
    $r = (array_key_exists(($key), $values)) ? $values[$key] : (null);
    $r = array_key_exists($key, $values) ?: null;
    $r = array_key_exists($key, $values) ? $values : null;
    $r = array_key_exists($key, $other) ? $values[$key] : null;
    $r = array_key_exists($other, $values) ? $values[$key] : null;
    $r = in_array($key, $values) ? $values[$key] : null;
    $r = $f($key, $values) ? $values[$key] : null;
    $r = $values ? $values[$key] : null;
}
-----
<?php

function coalesceOracle($values, $key, $other, $f)
{
    $r = $values[$key] ?? NULL;
    $r = $values[$key] ?? \null;
    $r = $values[$key] ?? null;
    $r = $values[$key] ?? null;
    $r = array_key_exists($key, $values) ?: null;
    $r = array_key_exists($key, $values) ? $values : null;
    $r = array_key_exists($key, $other) ? $values[$key] : null;
    $r = array_key_exists($other, $values) ? $values[$key] : null;
    $r = in_array($key, $values) ? $values[$key] : null;
    $r = $f($key, $values) ? $values[$key] : null;
    $r = $values ? $values[$key] : null;
}
