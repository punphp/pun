<?php
namespace TraitInitializerConflicts;
trait UnionValue { public const VALUE = true ? 1 : 4; }
class EqualUnion { use UnionValue; public const VALUE = false ? 4 : 1; }
class DifferentUnion { use UnionValue; public const VALUE = true ? 1 : 8; }
class ScalarSubset { use UnionValue; public const VALUE = 1; }
trait SelfValue { public const VALUE = true ? self::SEED : parent::SEED; }
class ParentValue { public const SEED = 4; }
class ContextEqual extends ParentValue { use SelfValue; public const SEED = 1; public const VALUE = true ? 4 : 1; }
class ContextDifferent extends ParentValue { use SelfValue; public const SEED = 2; public const VALUE = true ? 1 : 4; }
trait ArrayValue { public const VALUE = ['flag' => true ? 1 : 4]; }
class ArrayEqual { use ArrayValue; public const VALUE = ['flag' => false ? 4 : 1]; }
class ArrayDifferent { use ArrayValue; public const VALUE = ['flag' => 1]; }
trait DeclaredValue { /** @var int */ public const VALUE = true ? 1 : 4; }
class DeclaredSame { use DeclaredValue; /** @var positive-int */ public const VALUE = true ? 4 : 1; }
class DeclaredDifferent { use DeclaredValue; /** @var int */ public const VALUE = 8; }
trait NativeValue { public const int VALUE = true ? 1 : 4; }
class NativeDifferent { use NativeValue; public const int VALUE = 1; }
