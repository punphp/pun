<?php
namespace Other {
use Some\Thing as Alias;
class PBase
{
    /**
     * Doc of construct.
     */
    protected function __construct(Dep $dep, Alias $a, array $opts = ['a' => "x\n"], int &$ref = 0x1F, ?self $s = null, private readonly ?int $pp = null)
    {
    }
}
class GP
{
    public function __construct(int $a, $b = [1, 'x' => -2.50, null, PHP_EOL, \PHP_INT_MAX, Dep::class, 1_000, 017, 0b101, 1e3, 'it\'s', "tab\t$"])
    {
    }
}
class Mid extends GP {}
class Attributed
{
    public function __construct(
        // the name
        #[\SensitiveParameter] string $name,
        /* opts */ array $opts = array(1, 2),
    ) {
    }
}
class WithApp
{
    public function __construct(private \Illuminate\Foundation\Application $application) {}
}
}
namespace N {
use Illuminate\Support\Facades\App;
class A extends \Other\PBase
{
    public function go() { return App::get('y'); }
}
class B extends \Other\Mid
{
    public function go() { return App::get('y'); }
}
class C extends \Other\Attributed
{
    public function go() { return App::get('y'); }
}
class D extends \Other\WithApp
{
    public function go() { return App::get('y'); }
}
}
-----
<?php
namespace Other {
use Some\Thing as Alias;
class PBase
{
    /**
     * Doc of construct.
     */
    protected function __construct(Dep $dep, Alias $a, array $opts = ['a' => "x\n"], int &$ref = 0x1F, ?self $s = null, private readonly ?int $pp = null)
    {
    }
}
class GP
{
    public function __construct(int $a, $b = [1, 'x' => -2.50, null, PHP_EOL, \PHP_INT_MAX, Dep::class, 1_000, 017, 0b101, 1e3, 'it\'s', "tab\t$"])
    {
    }
}
class Mid extends GP {}
class Attributed
{
    public function __construct(
        // the name
        #[\SensitiveParameter] string $name,
        /* opts */ array $opts = array(1, 2),
    ) {
    }
}
class WithApp
{
    public function __construct(private \Illuminate\Foundation\Application $application) {}
}
}
namespace N {
use Illuminate\Support\Facades\App;
class A extends \Other\PBase
{
    /**
     * Doc of construct.
     */
    protected function __construct(\Other\Dep $dep, \Some\Thing $a, array $opts = ['a' => "x\n"], int &$ref = 0x1f, ?self $s = null, private readonly ?int $pp = null, private \Illuminate\Foundation\Application $application)
    {
        parent::__construct($dep, $a, $opts, $ref, $s, $pp);
    }
    public function go() { return $this->application->get('y'); }
}
class B extends \Other\Mid
{
    public function __construct(int $a, $b = [1, 'x' => -2.5, null, PHP_EOL, \PHP_INT_MAX, \Other\Dep::class, 1000, 017, 0b101, 1000.0, 'it\'s', "tab\t\$"], private \Illuminate\Foundation\Application $application)
    {
        parent::__construct($a, $b);
    }
    public function go() { return $this->application->get('y'); }
}
class C extends \Other\Attributed
{
    public function __construct(
        // the name
        #[\SensitiveParameter]
        string $name,
        /* opts */
        array $opts = array(1, 2),
        private \Illuminate\Foundation\Application $application
    )
    {
        parent::__construct($name, $opts);
    }
    public function go() { return $this->application->get('y'); }
}
class D extends \Other\WithApp
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
}
}
