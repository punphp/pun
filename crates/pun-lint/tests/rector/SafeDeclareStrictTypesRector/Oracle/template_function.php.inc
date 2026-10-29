<?php

namespace Pun\Oracle\SafeStrict;

/**
 * @template T
 * @param T $x
 * @return T
 */
function id($x)
{
    return $x;
}

id(1);
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

/**
 * @template T
 * @param T $x
 * @return T
 */
function id($x)
{
    return $x;
}

id(1);
