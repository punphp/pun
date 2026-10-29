<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(int $a): void
    {
    }

    public function n(): void
    {
        $this->m(1);
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public function m(int $a): void
    {
    }

    public function n(): void
    {
        $this->m(1);
    }
}
