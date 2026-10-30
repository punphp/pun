<?php

namespace Oracle\PropertyExists3;

class Box
{
    public $item;
}

trait ChecksBox
{
    public function check(Box $box)
    {
        return isset($box->item) && isset($this->own);
    }
}

class UsesTrait
{
    use ChecksBox;

    public $own;

    public function run(Box $box)
    {
        $f = static fn () => isset($box->item);

        return $f;
    }
}
-----
<?php

namespace Oracle\PropertyExists3;

class Box
{
    public $item;
}

trait ChecksBox
{
    public function check(Box $box)
    {
        return $box->item !== null && isset($this->own);
    }
}

class UsesTrait
{
    use ChecksBox;

    public $own;

    public function run(Box $box)
    {
        $f = static fn () => $box->item !== null;

        return $f;
    }
}
