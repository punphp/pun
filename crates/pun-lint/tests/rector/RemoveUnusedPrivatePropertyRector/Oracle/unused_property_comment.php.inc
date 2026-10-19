<?php

final class TrailingComment
{
    private $a;

    public function __construct()
    {
        $this->a = 1;
        echo 1;
        // trailing
    }
}
-----
<?php

final class TrailingComment
{
    public function __construct()
    {
        echo 1;
        // trailing
    }
}
