<?php

use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    protected $visible = ['x' . 'y', \PHP_EOL, <<<'X'
    a
    X];
}

class B extends Model
{
    protected $visible = ['a', UNKNOWN_CONSTANT];
}

class C extends Model
{
    protected $visible = ['a', 1];
}

class D extends Model
{
    protected $visible = [];
}

class E extends Model
{
    protected $visible = 'a';
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\Visible(['x' . 'y', \PHP_EOL, <<<'X'
    a
    X])]
class A extends Model
{
}

class B extends Model
{
    protected $visible = ['a', UNKNOWN_CONSTANT];
}

class C extends Model
{
    protected $visible = ['a', 1];
}

class D extends Model
{
    protected $visible = [];
}

class E extends Model
{
    protected $visible = 'a';
}
