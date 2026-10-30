<?php
namespace InitializerObjectReaders;
use InitializerObjectIdentity\Box as AliasBox;
use InitializerObjectIdentity\DefaultBox;
use InitializerObjectIdentity\Values;
use const InitializerObjectIdentity\SCANNED_GENERIC;

class Reads {
    public const ALIAS = new AliasBox(1);
    public const COPY = Values::GENERIC;
    public const GLOBAL_COPY = SCANNED_GENERIC;
    public const NESTED = ['outer' => [Values::GENERIC], 'object' => new (AliasBox::class)(1)];
    /** @var AliasBox<string> */ public const DOC = new AliasBox(1);
    final public const FINAL_RAW = new DefaultBox(1);
    final public const object FINAL_TYPED = new AliasBox(1);
    public function inspect(): void {
        \PHPStan\dumpType(static::DOC);
        \PHPStan\dumpType(static::FINAL_RAW);
        \PHPStan\dumpType(static::FINAL_TYPED);
        \PHPStan\dumpType(self::DOC);
    }
}
\PHPStan\dumpType(Reads::ALIAS);
\PHPStan\dumpType(Reads::COPY);
\PHPStan\dumpType(Reads::GLOBAL_COPY);
\PHPStan\dumpType(Reads::NESTED);
\PHPStan\dumpType(Reads::FINAL_RAW);

/** @return Values::GENERIC */
function reflected(): object { return Values::GENERIC; }
\PHPStan\dumpType(reflected());
\PHPStan\dumpType(reflected()->get());

function cached(): void {
    \PHPStan\dumpType(Reads::FINAL_RAW->value);
    \PHPStan\dumpType(Reads::FINAL_RAW->value);
    \PHPStan\dumpType(Reads::FINAL_RAW->get());
    \PHPStan\dumpType(Reads::FINAL_RAW);
    \PHPStan\dumpType(Reads::FINAL_RAW->value);
}

/** @template T of int */
class BoundBox {
    /** @param T $value */ public function __construct(public int $value) {}
    /** @return T */ public function get(): int { return $this->value; }
}
class Bounded { public const VALUE = new BoundBox(1); }
\PHPStan\dumpType(Bounded::VALUE);
\PHPStan\dumpType(Bounded::VALUE->get());
