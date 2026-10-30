<?php

namespace App;

final class MethodExistsVariants
{
    private $fromObject;
    private $fromMixed;
    private $fromString;

    public function a(object $o, $m, string $s)
    {
        if (method_exists($o, 'run')) {
            $this->fromObject = $o;
        }
        if (method_exists($m, 'run')) {
            $this->fromMixed = $m;
        }
        if (method_exists($s, 'run')) {
            $this->fromString = $s;
        }
    }
}
-----
<?php

namespace App;

final class MethodExistsVariants
{
    private ?object $fromObject = null;
    private object|string|null $fromMixed = null;
    private ?string $fromString = null;

    public function a(object $o, $m, string $s)
    {
        if (method_exists($o, 'run')) {
            $this->fromObject = $o;
        }
        if (method_exists($m, 'run')) {
            $this->fromMixed = $m;
        }
        if (method_exists($s, 'run')) {
            $this->fromString = $s;
        }
    }
}
