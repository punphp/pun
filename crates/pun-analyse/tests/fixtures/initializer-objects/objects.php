<?php
namespace InitializerObjectIdentity;
class Plain { public function __construct(public int $value) {} }
class Other { public function __construct(public int $value) {} }
/** @template T */
class Box {
    /** @param T $value */ public function __construct(public mixed $value) {}
    /** @return T */ public function get(): mixed { return $this->value; }
}
/** @template T = string */
class DefaultBox {
    /** @param T $value */ public function __construct(public mixed $value) {}
    /** @return T */ public function get(): mixed { return $this->value; }
}
\PHPStan\dumpType(GLOBAL_PLAIN);
\PHPStan\dumpType(GLOBAL_GENERIC);
\PHPStan\dumpType(GLOBAL_DEFAULT);
const GLOBAL_PLAIN = new Plain(1);
const GLOBAL_GENERIC = new Box(1);
const GLOBAL_DEFAULT = new DefaultBox(1);
\PHPStan\dumpType(GLOBAL_PLAIN);
\PHPStan\dumpType(GLOBAL_GENERIC);
\PHPStan\dumpType(GLOBAL_DEFAULT);
class Values {
    public const PLAIN = new Plain(1);
    public const GENERIC = new Box(1);
    public const DEFAULT = new DefaultBox(1);
    public const ARRAY_VALUE = ['box' => new Box(1)];
    public const UNION_VALUE = false ? new Box(1) : new Other(1);
    public const DYNAMIC = new (Plain::class)(1);
    /** @var Box<string> */ public const DOC = new Box(1);
    public const object NATIVE = new Box(1);
}
\PHPStan\dumpType(Values::PLAIN);
\PHPStan\dumpType(Values::GENERIC);
\PHPStan\dumpType(Values::DEFAULT);
\PHPStan\dumpType(Values::ARRAY_VALUE);
\PHPStan\dumpType(Values::UNION_VALUE);
\PHPStan\dumpType(Values::DYNAMIC);
\PHPStan\dumpType(Values::DOC);
\PHPStan\dumpType(Values::NATIVE);
\PHPStan\dumpType(Values::GENERIC->get());
\PHPStan\dumpType(Values::DEFAULT->get());
function local(): void {
    \PHPStan\dumpType(new Box(1));
    \PHPStan\dumpType(new DefaultBox(1));
}
trait GenericInput { public const VALUE = new Box(1); }
class EqualGeneric { use GenericInput; public const VALUE = new Box(2); }
class WrongGeneric { use GenericInput; public const VALUE = new Other(1); }
trait ShapeInput { public const VALUE = ['box' => new Box(1)]; }
class WrongShape { use ShapeInput; public const VALUE = ['box' => new Other(1)]; }
class ParentPlain extends Plain {
    public const SELF_VALUE = new self(1);
    public const PARENT_VALUE = new parent(1);
    public const STATIC_VALUE = new static(1);
}
\PHPStan\dumpType(ParentPlain::SELF_VALUE);
\PHPStan\dumpType(ParentPlain::PARENT_VALUE);
\PHPStan\dumpType(ParentPlain::STATIC_VALUE);
\PHPStan\dumpType(SCANNED_GENERIC);
\PHPStan\dumpType(Scanned::GENERIC);
