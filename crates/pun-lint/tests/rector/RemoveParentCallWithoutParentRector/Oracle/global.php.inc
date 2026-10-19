<?php

class Base
{
    public function run()
    {
    }
}

class Child extends Base
{
    private $items = [];

    public function run()
    {
        parent::run();
        parent::stop();
    }
}
-----
<?php

class Base
{
    public function run()
    {
    }
}

class Child extends Base
{
    private $items = [];

    public function run()
    {
        parent::run();
    }
}
