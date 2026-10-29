<?php

namespace Pun\Oracle\SafeStrict;

abstract class A
{
    abstract public function m(): int;
}

interface I
{
    public function m(): int;
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

abstract class A
{
    abstract public function m(): int;
}

interface I
{
    public function m(): int;
}
