<?php

namespace Pun\Pun\ArrayKeyExistsNull;

function f($k, $m, ?string $s, $arr)
{
    array_key_exists($k ? $s : $m . 'a', $arr);
    array_key_exists($k = $m, $arr);
}
-----
<?php

namespace Pun\Pun\ArrayKeyExistsNull;

function f($k, $m, ?string $s, $arr)
{
    array_key_exists((string) ($k ? $s : $m . 'a'), $arr);
    array_key_exists((string) ($k = $m), $arr);
}
