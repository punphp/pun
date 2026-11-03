<?php

namespace Illuminate\Validation\Rules;

use InvalidArgumentException;
use Stringable;

class IpAddress implements Stringable
{
    /**
     * @var 4|6|null
     */
    protected ?int $version = null;
    private array $allowedVersions = [4, 6];

    /**
     * Set the IP version.
     *
     * @template TVersion of int
     *
     * @param  TVersion  $version
     *
     * @phpstan-return  ($version is 4|6 ? static : never)
     *
     * @throws InvalidArgumentException
     */
    public function version(int $version): static
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return 'ip'|'ipv4'|'ipv6'
     */
    public function __toString(): string
    {}
}
