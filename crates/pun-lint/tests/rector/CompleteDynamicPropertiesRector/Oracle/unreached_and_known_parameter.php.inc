<?php

namespace Pun\Oracle\DynamicProperties;

class Known
{
}

class Unreached
{
    public function set(Known $known)
    {
        $this->known = $known;
        return;
        $this->value = 1;
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

class Known
{
}

class Unreached
{
    /**
     * @var \Pun\Oracle\DynamicProperties\Known
     */
    public $known;
    /**
     * @var int
     */
    public $value;
    public function set(Known $known)
    {
        $this->known = $known;
        return;
        $this->value = 1;
    }
}
