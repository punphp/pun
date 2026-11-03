<?php declare(strict_types=1);

namespace PhpParser\Node\Scalar;

use PhpParser\Error;
use PhpParser\Node\Scalar;

class Int_ extends Scalar {
    /* For use in "kind" attribute */
    public const KIND_BIN = 2;
    public const KIND_OCT = 8;
    public const KIND_DEC = 10;
    public const KIND_HEX = 16;

    /** @var int Number value */
    public int $value;

    /**
     * Constructs an integer number scalar node.
     *
     * @param int $value Value of the number
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(int $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    /**
     * Constructs an Int node from a string number literal.
     *
     * @param string $str String number literal (decimal, octal, hex or binary)
     * @param array<string, mixed> $attributes Additional attributes
     * @param bool $allowInvalidOctal Whether to allow invalid octal numbers (PHP 5)
     *
     * @return Int_ The constructed LNumber, including kind attribute
     */
    public static function fromString(string $str, array $attributes = [], bool $allowInvalidOctal = false): Int_ {}

    public function getType(): string {}
}

// @deprecated compatibility alias
class_alias(Int_::class, LNumber::class);
