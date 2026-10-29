<?php

namespace Pun\Oracle\DynamicProperties;

class Hooks
{
    public int $hooked {
        get => 1;
    }

    abstract public function m();

    public function set()
    {
        $this->hooked = 2;
        $local[0] = 1;
        $this->closure = function () {
        };
        $this->arrow = fn () => 1;
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

class Hooks
{
    /**
     * @var \Closure(): void
     */
    public $closure;
    /**
     * @var \Closure(): int
     */
    public $arrow;
    public int $hooked {
        get => 1;
    }

    abstract public function m();

    public function set()
    {
        $this->hooked = 2;
        $local[0] = 1;
        $this->closure = function () {
        };
        $this->arrow = fn () => 1;
    }
}
