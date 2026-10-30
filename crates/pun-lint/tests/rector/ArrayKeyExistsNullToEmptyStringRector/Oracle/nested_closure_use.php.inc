<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function nested_closure_use($k, $arr)
{
    return function () use ($k, $arr) {
        return function () use ($k, $arr) {
            if ($k !== null) {
                return array_key_exists($k, $arr);
            }
        };
    };
}
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function nested_closure_use($k, $arr)
{
    return function () use ($k, $arr) {
        return function () use ($k, $arr) {
            if ($k !== null) {
                return array_key_exists((string) $k, $arr);
            }
        };
    };
}
