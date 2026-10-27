<?php

use Illuminate\Contracts\Queue\ShouldQueue;

final class A implements ShouldQueue
{
    public $deleteWhenMissingModels = TRUE;
}

final class B implements ShouldQueue
{
    public $deleteWhenMissingModels = !false;
}

final class C implements ShouldQueue
{
    public $deleteWhenMissingModels = 1;
}

final class D implements ShouldQueue
{
    public $deleteWhenMissingModels = \true;
}
-----
<?php

use Illuminate\Contracts\Queue\ShouldQueue;

#[\Illuminate\Queue\Attributes\DeleteWhenMissingModels]
final class A implements ShouldQueue
{
}

#[\Illuminate\Queue\Attributes\DeleteWhenMissingModels]
final class B implements ShouldQueue
{
}

final class C implements ShouldQueue
{
    public $deleteWhenMissingModels = 1;
}

#[\Illuminate\Queue\Attributes\DeleteWhenMissingModels]
final class D implements ShouldQueue
{
}
