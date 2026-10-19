<?php

final class InlineTwice
{
    private $a;

    public function __construct()
    {
        $this->a = 1;
        $this->a = 2;
    }
}
-----
<?php

final class InlineTwice
{
    private $a = 2;

    public function __construct()
    {
    }
}
