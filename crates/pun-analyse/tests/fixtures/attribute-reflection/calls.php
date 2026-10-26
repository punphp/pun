<?php

namespace AttributeReflection;

trait PublicConstructor
{
    public function __construct() {}
}

#[\Attribute]
class PrivateAttribute
{
    use PublicConstructor { __construct as private; }
}

#[PrivateAttribute]
class PrivateUse {}

trait PrivateNested
{
    use PublicConstructor { __construct as private; }
}

#[\Attribute]
class NestedPrivateAttribute
{
    use PrivateNested;
}

#[NestedPrivateAttribute]
class NestedPrivateUse {}

#[\Attribute]
class InheritedPrivateAttribute extends PrivateAttribute {}

#[InheritedPrivateAttribute]
class InheritedPrivateUse {}

trait TypedConstructor
{
    /** @no-named-arguments */
    public function __construct(int $x) {}
}

#[\Attribute]
class TypedAttribute
{
    use TypedConstructor;
}

#[TypedAttribute(x: 'bad')]
class TypedUse {}

#[\Attribute]
class InheritedTypedAttribute extends TypedAttribute {}

#[InheritedTypedAttribute(x: 'bad')]
class InheritedTypedUse {}

trait Initializer
{
    /** @no-named-arguments */
    public function initialize(int $x) {}
}

#[\Attribute]
class AliasedAttribute
{
    use Initializer { initialize as __construct; }
}

#[AliasedAttribute(x: 'bad')]
class AliasedUse {}

#[\Attribute]
class PrivateAliasedAttribute
{
    use Initializer { initialize as private __construct; }
}

#[PrivateAliasedAttribute(x: 'bad')]
class PrivateAliasedUse {}

trait ContextConstructor
{
    /** @param self $value */
    public function __construct($value) {}
}

#[\Attribute]
class ContextAttribute
{
    use ContextConstructor;
}

#[ContextAttribute(1)]
class ContextUse {}

#[\Attribute]
class InheritedContextAttribute extends ContextAttribute {}

#[InheritedContextAttribute(1)]
class InheritedContextUse {}

#[\Attribute]
class TemplateAttribute
{
    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function __construct($value) {}
}

#[TemplateAttribute(1)]
class TemplateUse {}

trait TemplateConstructor
{
    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function __construct($value) {}
}

#[\Attribute]
class TraitTemplateAttribute
{
    use TemplateConstructor { __construct as factory; }
}

#[TraitTemplateAttribute(1)]
class TraitTemplateUse {}

trait TemplateInitializer
{
    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function initialize($value) {}
}

#[\Attribute]
class AliasedTemplateAttribute
{
    use TemplateInitializer { initialize as __construct; }
}

#[AliasedTemplateAttribute(1)]
class AliasedTemplateUse {}

class NativeReturn
{
    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function method($value): int { return 1; }

    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function untyped($value) { return $value; }
}

/**
 * @template T of object
 * @param T $value
 * @return T
 */
function __construct($value) { return $value; }

function controls(TypedAttribute $typed, TraitTemplateAttribute $templated, NativeReturn $native): void
{
    new TypedAttribute(x: 'bad');
    new InheritedTypedAttribute(x: 'bad');
    new AliasedAttribute(x: 'bad');
    new PrivateAttribute();
    $typed->__construct(x: 'bad');
    $templated->factory(1);
    $templated->__construct(1);
    $native->method(1);
    $native->untyped(1);
    __construct(1);
    new ContextAttribute(new InheritedContextAttribute(new \stdClass()));
}

// An explicit native declaration still wins over the implicit magic void, even though PHP rejects that
// constructor declaration. PHPStan checks its calls and still reports the unresolved return template.
#[\Attribute]
class ExplicitReturnAttribute
{
    /**
     * @template T of object
     * @param T $value
     * @return T
     */
    public function __construct($value): object { return $value; }
}

#[ExplicitReturnAttribute(1)]
class ExplicitReturnUse {}

#[\Attribute]
class PlainTraitTemplateAttribute
{
    use TemplateConstructor;
}

#[PlainTraitTemplateAttribute(1)]
class PlainTraitTemplateUse {}

#[\Attribute]
class PublicReadaptedAttribute
{
    use PrivateNested { __construct as public; }
}

#[PublicReadaptedAttribute]
class PublicReadaptedUse {}

/** @no-named-arguments */
trait ClassPolicyConstructor
{
    public function __construct(int $value) {}
}

#[\Attribute]
class ClassPolicyAttribute
{
    use ClassPolicyConstructor;
}

#[ClassPolicyAttribute(value: 1)]
class ClassPolicyUse {}

/** @no-named-arguments */
#[\Attribute]
class ForbiddenClassPolicyAttribute
{
    use ClassPolicyConstructor;
}

#[ForbiddenClassPolicyAttribute(value: 1)]
class ForbiddenClassPolicyUse {}

trait NativeSelfMethod
{
    public function accepts(self $value): void {}
}

class NativeSelfOwner
{
    use NativeSelfMethod { accepts as renamed; }
}

class NativeSelfChild extends NativeSelfOwner {}

function nativeSelfControls(NativeSelfOwner $owner, NativeSelfChild $child): void
{
    $child->accepts($owner);
    $child->renamed($owner);
    $child->renamed(new \stdClass());
}

trait SelectedConstructor
{
    /** @param int $value */
    public function __construct($value) {}
}

trait OtherConstructor
{
    /** @param string $other */
    public function __construct($other, int $extra = 0) {}
}

#[\Attribute]
class SelectedAttribute
{
    use SelectedConstructor, OtherConstructor {
        SelectedConstructor::__construct insteadof OtherConstructor;
        OtherConstructor::__construct as other;
    }
}

#[SelectedAttribute('bad')]
class SelectedUse {}

function selectedControls(SelectedAttribute $selected): void
{
    $selected->other(other: 1);
}

trait AliasedPolicyConstructor
{
    /** @no-named-arguments */
    public function __construct(int $value) {}
}

#[\Attribute]
class AliasedPolicyAttribute
{
    use AliasedPolicyConstructor { __construct as factory; }
}

#[AliasedPolicyAttribute(value: 1)]
class AliasedPolicyUse {}

function aliasedPolicyControls(AliasedPolicyAttribute $attribute): void
{
    $attribute->factory(value: 1);
    $attribute->__construct(value: 1);
}
