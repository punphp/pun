<?php

final class A
{
    public function a()
    {
        $f = function () {
            return new STATIC;
        };
        $o = new class {
            public function b()
            {
                return new static();
            }
        };
        return [new static(1), new self(), new A(), static::class];
    }
}

abstract class B
{
    public function a()
    {
        return new static();
    }
}
-----
<?php

final class A
{
    public function a()
    {
        $f = function () {
            return new self;
        };
        $o = new class {
            public function b()
            {
                return new self();
            }
        };
        return [new self(1), new self(), new A(), static::class];
    }
}

abstract class B
{
    public function a()
    {
        return new static();
    }
}
