<?php
namespace TraitInitializers;
trait AttributeFlags {
    public const FLAGS = self::ZERO ?: parent::FLAGS;
}
class ClassTarget { public const FLAGS = \Attribute::TARGET_CLASS; }
class MethodTarget { public const FLAGS = \Attribute::TARGET_METHOD; }
#[\Attribute(self::FLAGS)]
class GoodAttribute extends ClassTarget { use AttributeFlags; public const ZERO = 0; }
#[\Attribute(self::FLAGS)]
class WrongTargetAttribute extends MethodTarget { use AttributeFlags; public const ZERO = 0; }
#[GoodAttribute]
class GoodUse {}
#[WrongTargetAttribute]
class WrongUse {}
#[\Attribute(self::FLAGS)]
class ScannedAttribute extends ClassTarget { use ScannedInputs; public const ZERO = 0; }
#[ScannedAttribute]
class ScannedUse {}
#[\Attribute(\Attribute::TARGET_CLASS_CONSTANT)]
class Text { public function __construct(string $value) {} }
trait AnnotatedConstant {
    #[Text(self::SEED)]
    public const MARKED = 1;
}
class TextConsumer { use AnnotatedConstant; public const SEED = 'okay'; }
class NumberConsumer { use AnnotatedConstant; public const SEED = 1; }
class InheritedTextConsumer extends TextConsumer { public const SEED = 2; }
class ReusedTextConsumer extends TextConsumer { use AnnotatedConstant; public const SEED = 3; }
#[\Attribute(\Attribute::TARGET_CLASS)]
class TraitText { public function __construct(string $value) {} }
#[TraitText(self::SEED)]
trait AnnotatedTrait { public const SEED = 'trait'; }
class AnnotationNotInherited { use AnnotatedTrait; public const SEED = 4; }
