<?php

namespace Pun\Oracle\SafeStrict;

enum E
{
    case A;
}

function g(E $e): void
{
}

g(E::A);
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

enum E
{
    case A;
}

function g(E $e): void
{
}

g(E::A);
