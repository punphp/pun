<?php

use Illuminate\Http\Resources\Json\JsonResource;

class A extends JsonResource
{
    protected $collects = self::class;
}

class B extends JsonResource
{
    protected $collects = A::NAME;
}

class C extends JsonResource
{
    protected $collects = <<<'X'
    App\Resource
    X;
}

class D extends JsonResource
{
    protected $collects = 1;
}

class E extends JsonResource
{
    public $collects = 'A';
}
-----
<?php

use Illuminate\Http\Resources\Json\JsonResource;

#[\Illuminate\Http\Resources\Attributes\Collects(self::class)]
class A extends JsonResource
{
}

#[\Illuminate\Http\Resources\Attributes\Collects(A::NAME)]
class B extends JsonResource
{
}

#[\Illuminate\Http\Resources\Attributes\Collects(<<<'X'
    App\Resource
    X)]
class C extends JsonResource
{
}

class D extends JsonResource
{
    protected $collects = 1;
}

class E extends JsonResource
{
    public $collects = 'A';
}
