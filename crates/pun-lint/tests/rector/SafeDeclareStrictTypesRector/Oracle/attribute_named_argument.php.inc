<?php

namespace Pun\Oracle\SafeStrict;

#[\Attribute]
final class Typed
{
    public function __construct(public int $value, public string $name = '')
    {
    }
}

#[Typed(1, name: 'a')]
class WithAttribute
{
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

#[\Attribute]
final class Typed
{
    public function __construct(public int $value, public string $name = '')
    {
    }
}

#[Typed(1, name: 'a')]
class WithAttribute
{
}
