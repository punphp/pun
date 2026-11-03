<?php declare(strict_types=1);

namespace PhpParser\Node\Scalar;

use PhpParser\Node\Scalar;

class Float_ extends Scalar {
    /** @var float Number value */
    public float $value;

    /**
     * Constructs a float number scalar node.
     *
     * @param float $value Value of the number
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(float $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    /**
     * @param mixed[] $attributes
     */
    public static function fromString(string $str, array $attributes = []): Float_ {}

    /**
     * @internal
     *
     * Parses a DNUMBER token like PHP would.
     *
     * @param string $str A string number
     *
     * @return float The parsed number
     */
    public static function parse(string $str): float {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(Float_::class, DNumber::class);
