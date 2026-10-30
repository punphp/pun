<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): ?self
    {
        return null;
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): ?self
    {
        return null;
    }
}
