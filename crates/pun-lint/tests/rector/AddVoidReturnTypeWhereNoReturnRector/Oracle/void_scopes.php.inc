<?php

trait VoidTrait
{
    public function f()
    {
        echo 1;
    }
}

class VoidProtected
{
    protected function f()
    {
        echo 1;
    }
}

final class VoidProtectedFinal
{
    protected function f()
    {
        echo 1;
    }
}

abstract class VoidAbstract
{
    public function f()
    {
    }
}

class VoidValue
{
    public function f()
    {
        return 1;
    }
}

enum VoidEnum
{
    case A;

    public function f()
    {
        echo 1;
    }
}
-----
<?php

trait VoidTrait
{
    public function f(): void
    {
        echo 1;
    }
}

class VoidProtected
{
    protected function f()
    {
        echo 1;
    }
}

final class VoidProtectedFinal
{
    protected function f(): void
    {
        echo 1;
    }
}

abstract class VoidAbstract
{
    public function f()
    {
    }
}

class VoidValue
{
    public function f()
    {
        return 1;
    }
}

enum VoidEnum
{
    case A;

    public function f(): void
    {
        echo 1;
    }
}
