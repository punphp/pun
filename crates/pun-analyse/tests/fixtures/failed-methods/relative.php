<?php
namespace RawInitializerRelativeProbe;
class Base {
    public function value(): string { return 'base'; }
}
class Child extends Base {
    public const SELF_VALUE = new self();
    public const PARENT_VALUE = new parent();
    public const STATIC_VALUE = new static();
}
\PHPStan\dumpType(Child::SELF_VALUE);
\PHPStan\dumpType(Child::SELF_VALUE->value());
\PHPStan\dumpType(Child::PARENT_VALUE->value());
\PHPStan\dumpType(Child::STATIC_VALUE->value());
final class Named {
    public const VALUE = new Base();
    public const RELATIVE = new self();
    public function value(): int { return 31; }
}
\PHPStan\dumpType(Named::VALUE->value());
\PHPStan\dumpType(Named::RELATIVE->value());
