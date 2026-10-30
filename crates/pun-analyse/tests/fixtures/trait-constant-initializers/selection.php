<?php
namespace TraitSelection;
trait Inputs { public const VALUE = self::SEED; }
class OrdinaryParent { public const VALUE = 1; }
class OrdinaryChild extends OrdinaryParent { use Inputs; public const SEED = 2; }
class PrivateParent { private const VALUE = 1; }
class PrivateChild extends PrivateParent { use Inputs; public const SEED = 4; }
\PHPStan\dumpType(OrdinaryChild::VALUE);
\PHPStan\dumpType(PrivateChild::VALUE);
trait Shape { public const VALUE = ['z' => true ? 1 : 4, 'a' => 2]; }
class Same { use Shape; public const VALUE = ['z' => true ? 4 : 1, 'a' => 2]; }
class Reordered { use Shape; public const VALUE = ['a' => 2, 'z' => true ? 1 : 4]; }
trait Nested { public const VALUE = array(3 => ['z' => 8, 'a' => 2], 'a' => 4); }
class NestedDifferent { use Nested; public const VALUE = array(3 => ['z' => 8, 'a' => 2], 'a' => 5); }
trait GlobalDependency { public const VALUE = HIDDEN_GLOBAL ?: 4; }
class ShadowedGlobal { use GlobalDependency; public const VALUE = 4; }
const HIDDEN_GLOBAL = null ?? 0;
trait ListValue { public const VALUE = [true ? 1 : 4, 2]; }
class SameList { use ListValue; public const VALUE = [false ? 4 : 1, 2]; }
class DifferentList { use ListValue; public const VALUE = [1, 2]; }
trait Duplicate { public const VALUE = ['a' => 0, 'a' => 4, 'z' => 2]; }
class SameDuplicate { use Duplicate; public const VALUE = ['a' => 4, 'z' => 2]; }
class ArraySource { public const VALUE = ['z' => 1, 'a' => 2]; }
trait Reference { public const VALUE = ArraySource::VALUE; }
class SameReference { use Reference; public const VALUE = ['z' => 1, 'a' => 2]; }
trait Indexed { public const VALUE = [0 => true ? 1 : 4, 1 => 2]; }
class DifferentIndexed { use Indexed; public const VALUE = [0 => 1, 1 => 2]; }
class DifferentScannedShape { use ScannedShape; public const VALUE = ['z' => 1, 'a' => 2]; }
const GLOBAL_SHAPE = ['z' => 1, 'a' => 2];
trait GlobalShape { public const VALUE = GLOBAL_SHAPE; }
class SameGlobalShape { use GlobalShape; public const VALUE = ['z' => 1, 'a' => 2]; }
define('TraitSelection\\DEFINED_SHAPE', ['z' => 1, 'a' => 2]);
trait DefinedShape { public const VALUE = DEFINED_SHAPE; }
class SameDefinedShape { use DefinedShape; public const VALUE = ['z' => 1, 'a' => 2]; }
define('TRAIT_GLOBAL_SHAPE', ['z' => 1, 'a' => 2]);
trait FallbackShape { public const VALUE = TRAIT_GLOBAL_SHAPE; }
class SameFallbackShape { use FallbackShape; public const VALUE = ['z' => 1, 'a' => 2]; }
trait ConditionalShape { public const VALUE = true ? ['a' => 1] : ['a' => 1]; }
class SameConditionalShape { use ConditionalShape; public const VALUE = ['a' => 1]; }
class DifferentConditionalShape { use ConditionalShape; public const VALUE = ['a' => 2]; }
