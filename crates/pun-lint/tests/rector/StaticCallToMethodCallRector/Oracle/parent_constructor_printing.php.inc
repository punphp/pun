<?php
namespace Other {
use Some\Thing as Alias;
#[\Attribute]
class Attr
{
    public function __construct(...$a)
    {
    }
}
class Printed
{
    const X = 1;
    public function __construct(
        int|string $u,
        Alias&\Countable $i,
        (Alias&\Countable)|null $dnf,
        #[Attr(1, name: Alias::class)] $attr,
        #[Attr(...[1])] $spread,
        $plus = +1,
        $neg = -(-1),
        $not = ~1,
        $const = self::X,
        $new = new Alias(1, n: 2),
        $newSpread = new Alias(...[1]),
        $arr = [...[1, 2]],
    ) {
    }
}
}
namespace N {
use Illuminate\Support\Facades\App;
class A extends \Other\Printed
{
    public function go() { return App::get(1); }
}
}
-----
<?php
namespace Other {
use Some\Thing as Alias;
#[\Attribute]
class Attr
{
    public function __construct(...$a)
    {
    }
}
class Printed
{
    const X = 1;
    public function __construct(
        int|string $u,
        Alias&\Countable $i,
        (Alias&\Countable)|null $dnf,
        #[Attr(1, name: Alias::class)] $attr,
        #[Attr(...[1])] $spread,
        $plus = +1,
        $neg = -(-1),
        $not = ~1,
        $const = self::X,
        $new = new Alias(1, n: 2),
        $newSpread = new Alias(...[1]),
        $arr = [...[1, 2]],
    ) {
    }
}
}
namespace N {
use Illuminate\Support\Facades\App;
class A extends \Other\Printed
{
    public function __construct(
        int|string $u,
        \Some\Thing&\Countable $i,
        (\Some\Thing&\Countable)|null $dnf,
        #[\Other\Attr(1, name: \Some\Thing::class)]
        $attr,
        #[\Other\Attr(...[1])]
        $spread,
        $plus = +1,
        $neg = -(-1),
        $not = ~1,
        $const = self::X,
        $new = new \Some\Thing(1, n: 2),
        $newSpread = new \Some\Thing(...[1]),
        $arr = [...[1, 2]],
        private \Illuminate\Foundation\Application $application
    )
    {
        parent::__construct($u, $i, $dnf, $attr, $spread, $plus, $neg, $not, $const, $new, $newSpread, $arr);
    }
    public function go() { return $this->application->get(1); }
}
}
