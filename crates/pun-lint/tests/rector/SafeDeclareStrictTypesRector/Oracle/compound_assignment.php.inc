<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public int $i = 0;

    public function m(): void
    {
        $this->i .= "a";
        $this->i ??= "b";
    }
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public int $i = 0;

    public function m(): void
    {
        $this->i .= "a";
        $this->i ??= "b";
    }
}
