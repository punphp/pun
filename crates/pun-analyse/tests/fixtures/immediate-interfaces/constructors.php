<?php
namespace ImmediateConstructors;

interface ZConstructor { public function __construct(string $z); }
interface AConstructor { public function __construct(string $a); }
class ParentClass implements ZConstructor, AConstructor {
    public function __construct(string $middle) {}
}
class Repeated extends ParentClass implements ZConstructor, AConstructor {
    public function __construct(int $new) {}
}
class Inherited extends ParentClass {
    public function __construct(int $new) {}
}
class ReverseParent implements AConstructor, ZConstructor {
    public function __construct(string $middle) {}
}
class ReverseInherited extends ReverseParent {
    public function __construct(int $new) {}
}
interface NarrowConstructor extends ZConstructor { public function __construct(string $narrow); }
class NarrowParent implements NarrowConstructor {
    public function __construct(string $middle) {}
}
class NarrowRepeated extends NarrowParent implements ZConstructor, NarrowConstructor {
    public function __construct(int $new) {}
}
class ConcreteParent { public function __construct(string $old) {} }
class FreeConstructor extends ConcreteParent { public function __construct(int $new) {} }
abstract class AbstractParent { abstract public function __construct(string $old); }
class AbstractChild extends AbstractParent { public function __construct(int $new) {} }

interface ZBranch extends ZConstructor {}
interface ABranch extends ZConstructor {}
interface Diamond extends ZBranch, ABranch {}
class DiamondParent implements Diamond { public function __construct(string $middle) {} }
class DiamondChild extends DiamondParent { public function __construct(int $new) {} }
