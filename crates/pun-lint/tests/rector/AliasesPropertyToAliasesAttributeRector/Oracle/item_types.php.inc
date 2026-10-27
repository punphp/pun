<?php

use Illuminate\Console\Command;

class A extends Command
{
    private const NAME = 'a:b';
    protected $aliases = [self::NAME, 'x' . 'y', \PHP_EOL, UNKNOWN_CONSTANT];
}

class B extends Command
{
    protected $aliases = ['a', 1];
}

class C extends Command
{
    protected $aliases = ['a', 1.5];
}

class D extends Command
{
    protected $aliases = ['a', true, null];
}

class E extends Command
{
    protected $aliases = ['a', ['b']];
}

class F extends Command
{
    protected $aliases = ['a', ...['b']];
}

class G extends Command
{
    private const N = 1;
    protected $aliases = [self::N];
}

class H extends Command
{
    protected $aliases = 'a';
}

class I extends Command
{
    public $aliases = ['a'];
}
-----
<?php

use Illuminate\Console\Command;

#[\Illuminate\Console\Attributes\Aliases([self::NAME, 'x' . 'y', \PHP_EOL, UNKNOWN_CONSTANT])]
class A extends Command
{
    private const NAME = 'a:b';
}

class B extends Command
{
    protected $aliases = ['a', 1];
}

class C extends Command
{
    protected $aliases = ['a', 1.5];
}

class D extends Command
{
    protected $aliases = ['a', true, null];
}

class E extends Command
{
    protected $aliases = ['a', ['b']];
}

class F extends Command
{
    protected $aliases = ['a', ...['b']];
}

class G extends Command
{
    private const N = 1;
    protected $aliases = [self::N];
}

class H extends Command
{
    protected $aliases = 'a';
}

class I extends Command
{
    public $aliases = ['a'];
}
