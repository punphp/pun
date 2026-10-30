<?php

namespace QualityPropertyContracts;

final class PartialDefaults
{
    private $boolean = true;
    private $float = 1.5;
    private $string = 'a' . 'b';
    private $nullable = null;

    public function __construct(bool $condition, ?int $nullable)
    {
        if ($condition) {
            $this->boolean = true;
            $this->float = 1.5;
            $this->string = 'ab';
            $this->nullable = $nullable;
        }
    }
}
-----
<?php

namespace QualityPropertyContracts;

final class PartialDefaults
{
    private bool $boolean = true;
    private float $float = 1.5;
    private string $string = 'a' . 'b';
    private $nullable = null;

    public function __construct(bool $condition, ?int $nullable)
    {
        if ($condition) {
            $this->boolean = true;
            $this->float = 1.5;
            $this->string = 'ab';
            $this->nullable = $nullable;
        }
    }
}
