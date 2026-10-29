<?php declare(strict_types=1);

namespace BuiltinPurityOverride;

class ImpureDate extends \DateTime
{
    /** @impure */
    public function format(string $format): string { return parent::format($format); }
}

class InheritedDate extends \DateTime {}

class ImpureInheritedDate extends InheritedDate
{
    /** @impure */
    public function format(string $format): string { return parent::format($format); }
}

class ImpureImmutableDate extends \DateTimeImmutable
{
    /** @phpstan-impure */
    public function format(string $format): string { return parent::format($format); }
}

class PureDate extends \DateTime
{
    /** @pure */
    public function format(string $format): string { return parent::format($format); }
}

class UnspecifiedDate extends \DateTime
{
    public function format(string $format): string { return parent::format($format); }
}

class ImpureCount implements \Countable
{
    /** @impure */
    public function count(): int { return 1; }
}

class ImpureMutation extends \DateTime
{
    /** @impure */
    public function modify(string $modifier): \DateTime { return parent::modify($modifier); }
}

class SignatureGates
{
    /** @pure */
    public function __construct() {}
    /** @pure */
    public static function create(): int { return 1; }
    /** @pure */
    private function hidden(): int { return 1; }
}

class ImpureGates extends SignatureGates
{
    /** @impure */
    public function __construct() {}
    /** @impure */
    public static function create(): int { return 1; }
    /** @impure */
    private function hidden(): int { return 1; }
}
