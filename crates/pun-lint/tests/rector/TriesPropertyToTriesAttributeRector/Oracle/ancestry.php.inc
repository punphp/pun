<?php

use Illuminate\Contracts\Queue\ShouldQueue;

abstract class Job implements ShouldQueue {}

final class A extends Job
{
    public $tries = 3;
}

$b = new class implements ShouldQueue {
    public $tries = 3;
};

final class C
{
    public $tries = 3;
}

#[\Illuminate\Queue\Attributes\Tries(1)]
final class D implements ShouldQueue
{
    public $tries = 3;
}
-----
<?php

use Illuminate\Contracts\Queue\ShouldQueue;

abstract class Job implements ShouldQueue {}

#[\Illuminate\Queue\Attributes\Tries(3)]
final class A extends Job
{
}

$b = new class implements ShouldQueue {
    public $tries = 3;
};

final class C
{
    public $tries = 3;
}

#[\Illuminate\Queue\Attributes\Tries(1)]
final class D implements ShouldQueue
{
    public $tries = 3;
}
