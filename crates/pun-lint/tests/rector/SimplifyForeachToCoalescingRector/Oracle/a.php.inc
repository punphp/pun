<?php

function a(array $map, $k)
{
    $x = 1;

    // lead
    foreach ($map as $key => $value) {
        if ($key === $k) {
            return $value;
        }
    }

    // tail
    return null;
}

function b(array $map, $k)
{
    $r = 1;

    // lead
    foreach ($map as $key => $value) {
        if ($k === $key) {
            $r = $value;
        }
    }
    return $r;
}
-----
<?php

function a(array $map, $k)
{
    $x = 1;
    return $map[$k] ?? null;
}

function b(array $map, $k)
{
    $r = 1;

    $r = $map[$k] ?? $r;
    return $r;
}
