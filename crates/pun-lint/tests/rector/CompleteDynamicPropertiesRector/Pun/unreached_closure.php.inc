<?php

namespace Pun\Oracle\DynamicProperties;

class UnreachedClosure
{
    public function set()
    {
        return;
        $this->closure = function () {
        };
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

class UnreachedClosure
{
    public $closure;
    public function set()
    {
        return;
        $this->closure = function () {
        };
    }
}
