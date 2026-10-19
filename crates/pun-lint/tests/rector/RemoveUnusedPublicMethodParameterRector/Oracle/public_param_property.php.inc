<?php

final class A
{
    public $property;

    public function run($unused)
    {
        echo 1;
    }
}
-----
<?php

final class A
{
    public $property;

    public function run()
    {
        echo 1;
    }
}
