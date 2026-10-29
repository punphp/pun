<?php

namespace Probe;

final class EnumRule
{
    /**
     * @param class-string<\UnitEnum> $type
     * @param list<\UnitEnum> $except
     */
    public function __construct(private string $type, private array $except)
    {
    }

    public function values(): array
    {
        return array_filter($this->type::cases(), fn ($case) => ! in_array($case, $this->except, true));
    }
}
-----
<?php

namespace Probe;

final class EnumRule
{
    /**
     * @param class-string<\UnitEnum> $type
     * @param list<\UnitEnum> $except
     */
    public function __construct(private string $type, private array $except)
    {
    }

    public function values(): array
    {
        return array_filter($this->type::cases(), fn (\UnitEnum $case) => ! in_array($case, $this->except, true));
    }
}
