<?php

use Illuminate\Contracts\Queue\ShouldQueue;

final class A implements ShouldQueue
{
    public $delay = 1 + 2;
}

final class B implements ShouldQueue
{
    public $delay = '10';
}

final class C implements ShouldQueue
{
    const D = 5;
    public $delay = self::D;
}

final class D implements ShouldQueue
{
    public $delay = 1.5;
}

final class E implements ShouldQueue
{
    var $delay = 3;
}

final class F implements ShouldQueue
{
    protected $delay = 3;
}

final class G implements ShouldQueue
{
    public $delay;
}

final class H implements ShouldQueue
{
    public static $delay = 3;
}
-----
<?php

use Illuminate\Contracts\Queue\ShouldQueue;

#[\Illuminate\Queue\Attributes\Delay(1 + 2)]
final class A implements ShouldQueue
{
}

final class B implements ShouldQueue
{
    public $delay = '10';
}

#[\Illuminate\Queue\Attributes\Delay(self::D)]
final class C implements ShouldQueue
{
    const D = 5;
}

final class D implements ShouldQueue
{
    public $delay = 1.5;
}

#[\Illuminate\Queue\Attributes\Delay(3)]
final class E implements ShouldQueue
{
}

final class F implements ShouldQueue
{
    protected $delay = 3;
}

final class G implements ShouldQueue
{
    public $delay;
}

#[\Illuminate\Queue\Attributes\Delay(3)]
final class H implements ShouldQueue
{
}
