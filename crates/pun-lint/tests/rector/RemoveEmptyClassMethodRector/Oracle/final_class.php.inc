<?php

namespace Pun\Oracle\EmptyMethod;

final class Final_
{
    protected function protectedOne()
    {
    }

    public function __clone()
    {
    }

    public function __invoke()
    {
    }

    public function kept()
    {
        // a comment is a statement
    }
}

class Open
{
    private function __clone()
    {
    }

    public function __destruct()
    {
    }

    private function unused()
    {
    }
}
-----
<?php

namespace Pun\Oracle\EmptyMethod;

final class Final_
{
    public function __invoke()
    {
    }

    public function kept()
    {
        // a comment is a statement
    }
}

class Open
{
    private function __clone()
    {
    }
}
