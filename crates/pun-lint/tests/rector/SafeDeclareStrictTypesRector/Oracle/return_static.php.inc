<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): static
    {
        return $this;
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(): static
    {
        return $this;
    }
}
