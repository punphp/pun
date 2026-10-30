<?php

namespace App;

use Illuminate\Support\Traits\Tappable;

abstract class A
{
    use Tappable;

    public function a()
    {
        return tap(new static, function ($i) {
        });
    }

    public static function b()
    {
        return tap(new static(), function ($i) {
        });
    }

    public function c()
    {
        return tap(new self, function ($i) {
        });
    }

    public function d()
    {
        return tap(static::make(), function ($i) {
        });
    }

    /** @return static */
    public static function make()
    {
        return new static;
    }
}

final class F
{
    public function a()
    {
        return tap(new static, function ($i) {
        });
    }
}
-----
<?php

namespace App;

use Illuminate\Support\Traits\Tappable;

abstract class A
{
    use Tappable;

    public function a()
    {
        return tap(new static, function ($i) {
        });
    }

    public static function b()
    {
        return tap(new static(), function ($i) {
        });
    }

    public function c()
    {
        return tap(new self, function (\App\A $i) {
        });
    }

    public function d()
    {
        return tap(static::make(), function (self $i) {
        });
    }

    /** @return static */
    public static function make()
    {
        return new static;
    }
}

final class F
{
    public function a()
    {
        return tap(new static, function ($i) {
        });
    }
}
