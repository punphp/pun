<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Calls
{
    public function run((Left&Right)|null $either, Left&Right $both)
    {
        $this->either($either);
        $this->both($both);
    }

    private function either($value)
    {
    }

    private function both($value)
    {
    }
}
-----
<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Calls
{
    public function run((Left&Right)|null $either, Left&Right $both)
    {
        $this->either($either);
        $this->both($both);
    }

    private function either($value)
    {
    }

    private function both(\Probe\Left&\Probe\Right $value)
    {
    }
}
