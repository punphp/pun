<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function short_ternaries(?string $s, ?int $i, int|string|null $u, $arr)
{
    array_key_exists($s ?: null, $arr);
    array_key_exists($s ?: $u, $arr);
    array_key_exists($i ?: $s, $arr);
}
-----
<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

function short_ternaries(?string $s, ?int $i, int|string|null $u, $arr)
{
    array_key_exists($s ?: '', $arr);
    array_key_exists($s ?: (string) $u, $arr);
    array_key_exists($i ?: (string) $s, $arr);
}
