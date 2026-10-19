<?php

class GlobalParent
{
    protected function run()
    {
    }
}

class GlobalChild extends GlobalParent
{
    private $property;

    private function run()
    {
    }
}
-----
<?php

class GlobalParent
{
    protected function run()
    {
    }
}

class GlobalChild extends GlobalParent
{
    private $property;

    protected function run()
    {
    }
}
