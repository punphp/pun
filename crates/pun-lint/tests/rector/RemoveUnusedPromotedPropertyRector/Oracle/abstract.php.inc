<?php

namespace App;

abstract class AbstractConstructor
{
    abstract public function __construct(private $value);
}
-----
<?php

namespace App;

abstract class AbstractConstructor
{
    abstract public function __construct();
}
