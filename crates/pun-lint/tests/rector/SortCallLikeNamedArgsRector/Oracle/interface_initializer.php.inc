<?php

interface ParentArgs
{
    public function __construct($first, $second);
}

interface ChildArgs extends ParentArgs
{
    public function run($value = new self(second: 2, first: 1));
}
-----
<?php

interface ParentArgs
{
    public function __construct($first, $second);
}

interface ChildArgs extends ParentArgs
{
    public function run($value = new self(first: 1, second: 2));
}
