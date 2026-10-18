<?php
final class C {
    final public const A = 1;
    public final const B = 2;
    final const C = 3, D = 4;
    #[Attr]
    final protected const E = 5;
    const F = 6;
}
class D {
    final public const A = 1;
}
final class E {
    public function f() {
        return new class {
            final public const A = 1;
        };
    }
}
-----
<?php
final class C {
    public const A = 1;
    public const B = 2;
    const C = 3, D = 4;
    #[Attr]
    protected const E = 5;
    const F = 6;
}
class D {
    final public const A = 1;
}
final class E {
    public function f() {
        return new class {
            final public const A = 1;
        };
    }
}
