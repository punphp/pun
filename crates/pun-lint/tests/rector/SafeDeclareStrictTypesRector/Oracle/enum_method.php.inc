<?php

namespace Pun\Oracle\SafeStrict;

enum E
{
    case A;

    public function m(int $i): void
    {
    }
}

E::A->m(1);
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

enum E
{
    case A;

    public function m(int $i): void
    {
    }
}

E::A->m(1);
