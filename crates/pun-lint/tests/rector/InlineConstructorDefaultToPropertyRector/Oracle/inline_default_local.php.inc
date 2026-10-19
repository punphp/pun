<?php

final class InlineLocal
{
    private $a;

    public function __construct()
    {
        $x = 1;
        $this->a = 2;
    }
}
-----
<?php

final class InlineLocal
{
    private $a = 2;

    public function __construct()
    {
        $x = 1;
    }
}
