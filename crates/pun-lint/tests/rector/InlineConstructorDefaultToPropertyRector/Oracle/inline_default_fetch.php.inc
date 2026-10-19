<?php

final class InlineFetch
{
    private $a;

    private $b;

    public function __construct($o, $x)
    {
        $o->a->b = 1;
        $this->{$x} = 1;
        $this->a = 2;
    }
}
-----
<?php

final class InlineFetch
{
    private $a = 2;

    private $b;

    public function __construct($o, $x)
    {
        $o->a->b = 1;
        $this->{$x} = 1;
    }
}
