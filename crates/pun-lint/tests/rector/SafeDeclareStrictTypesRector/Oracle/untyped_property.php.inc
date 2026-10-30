<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public $p;

    public function m(): void
    {
        $this->p = 1;
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public $p;

    public function m(): void
    {
        $this->p = 1;
    }
}
