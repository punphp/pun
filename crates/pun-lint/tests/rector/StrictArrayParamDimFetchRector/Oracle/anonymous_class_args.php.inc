<?php

function wrap($row)
{
    return new class($row['name']) {
        public function __construct(public $name)
        {
        }
    };
}

function plain($row)
{
    return new class {
    };
}
-----
<?php

function wrap(array $row)
{
    return new class($row['name']) {
        public function __construct(public $name)
        {
        }
    };
}

function plain($row)
{
    return new class {
    };
}
