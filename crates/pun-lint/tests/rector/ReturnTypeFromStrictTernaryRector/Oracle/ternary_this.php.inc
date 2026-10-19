<?php

final class TernaryThis
{
    public function f($x)
    {
        return $x ? $this : $this;
    }

    public function g($x)
    {
        return $x ? new class {} : new class {};
    }
}
-----
<?php

final class TernaryThis
{
    public function f($x): self
    {
        return $x ? $this : $this;
    }

    public function g($x)
    {
        return $x ? new class {} : new class {};
    }
}
