<?php
namespace HolderEdges;
/** @template T = string */
class GenericObject {}
enum Token { case First; case Second; }
final class A {
    public const V = 1;
    public const OBJECT = new GenericObject;
    public const SHAPE = ['value' => new GenericObject, 'token' => Token::First];
}
final class B { public const V = 4; public const OBJECT = new GenericObject; }
class EmptyHolder {}
class Unknown { public const V = UNDEFINED_GLOBAL; }
class Values {
    public const NESTED_OBJECT = (true ? A::class : B::class)::OBJECT;
    public const NESTED_SHAPE = (A::class)::SHAPE;
    public const FIRST_CASE = (true ? Token::class : EmptyHolder::class)::First;
    public const MISSING_FALLBACK = (true ? new A : EmptyHolder::class)::V;
    public const LITERAL_CLASS = ('HolderEdges\A')::class;
    public const LEADING_SLASH = (true ? '\HolderEdges\A' : '\HolderEdges\B')::V;
    public const MIXED_ARRAY = [1, 'value' => (true ? A::class : B::class)::V];
    public const LEGACY = array((true ? A::class : B::class)::V, 'next' => 7);
    public const RICH_OPERATOR = ((true ? A::class : B::class)::V) + 3;
    public const FLOATS = true ? 1.5 : 3.5;
    public const BOOLEANS = true ? true : false;
}
\PHPStan\dumpType(Values::NESTED_OBJECT);
\PHPStan\dumpType(Values::NESTED_SHAPE);
\PHPStan\dumpType(Values::FIRST_CASE);
\PHPStan\dumpType(Values::MISSING_FALLBACK);
\PHPStan\dumpType(Values::LITERAL_CLASS);
\PHPStan\dumpType(Values::LEADING_SLASH);
\PHPStan\dumpType(Values::MIXED_ARRAY);
\PHPStan\dumpType(Values::LEGACY);
\PHPStan\dumpType(Values::RICH_OPERATOR);
\PHPStan\dumpType(Values::FLOATS);
\PHPStan\dumpType(Values::BOOLEANS);
\PHPStan\dumpType(Unknown::V);
trait UnionSource { public const V = (true ? A::class : B::class)::V; }
class EqualReversed { use UnionSource; public const V = false ? 4 : 1; }
class DifferentUnion { use UnionSource; public const V = true ? 1 : 7; }
class LongerUnion { use UnionSource; public const V = true ? 1 : (false ? 4 : 7); }
class ScalarOrder {
    public const BOOL_FIRST = true ? true : new GenericObject;
    public const BOOL_LAST = true ? new GenericObject : false;
}
\PHPStan\dumpType(ScalarOrder::BOOL_FIRST);
\PHPStan\dumpType(ScalarOrder::BOOL_LAST);
