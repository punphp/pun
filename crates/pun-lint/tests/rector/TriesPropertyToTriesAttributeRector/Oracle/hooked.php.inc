<?php

use Illuminate\Contracts\Queue\ShouldQueue;

final class A implements ShouldQueue
{
    public int $tries = 3 {
        get => $this->tries;
    }
}

final class B implements ShouldQueue
{
    public int $tries {
        get => 3;
    }
}
-----
<?php

use Illuminate\Contracts\Queue\ShouldQueue;

#[\Illuminate\Queue\Attributes\Tries(3)]
final class A implements ShouldQueue
{
}

final class B implements ShouldQueue
{
    public int $tries {
        get => 3;
    }
}
