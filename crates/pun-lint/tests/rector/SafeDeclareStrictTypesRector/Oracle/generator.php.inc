<?php

namespace Pun\Oracle\SafeStrict;

function f(): iterable
{
    yield 1;
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

function f(): iterable
{
    yield 1;
}
