<?php

namespace Probe;

final class Calls
{
    public function run(?int $maybe, int $count)
    {
        $this->maybe($maybe);
        $this->count($count);
    }

    private function maybe($value)
    {
    }

    private function count($value)
    {
    }
}
-----
<?php

namespace Probe;

final class Calls
{
    public function run(?int $maybe, int $count)
    {
        $this->maybe($maybe);
        $this->count($count);
    }

    private function maybe($value)
    {
    }

    private function count(int $value)
    {
    }
}
