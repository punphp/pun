<?php

namespace Pun\Oracle\SafeStrict;

class P
{
    public static function m(int $i): void
    {
    }
}

class C extends P
{
    public function n(): void
    {
        parent::m(1);
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class P
{
    public static function m(int $i): void
    {
    }
}

class C extends P
{
    public function n(): void
    {
        parent::m(1);
    }
}
