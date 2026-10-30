<?php

namespace Pun\Oracle\SafeStrict;

function f(\DateTime $d): string
{
    return $d->format("Y");
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(\DateTime $d): string
{
    return $d->format("Y");
}
