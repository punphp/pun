<?php

namespace TemplateParams;

// Reduced from Illuminate's base and Eloquent collections: inherited @param tags still reference a template
// redeclared by the child, including PHPDoc variadics on a non-variadic native parameter.
abstract class BaseCollection
{
    /**
     * @template TPadValue
     * @param int $size
     * @param TPadValue $value
     * @return list<TPadValue>
     */
    abstract public function pad($size, $value);

    /**
     * @template TZipValue
     * @param iterable<array-key, TZipValue> ...$items
     * @return list<TZipValue>
     */
    abstract public function zip($items);

    /**
     * @template TRenamed
     * @param TRenamed $original
     * @return TRenamed
     */
    abstract public function renamed($original);

    /**
     * @template TOverride
     * @param TOverride $value
     * @return TOverride
     */
    abstract public function overridden($value);

    /**
     * @template TOriginal
     * @param TOriginal $value
     * @return TOriginal
     */
    abstract public function differentTemplate($value);

    /**
     * @template TConditional
     * @return ($flag is TConditional ? TConditional : int)
     */
    abstract public function conditional($flag);

    /**
     * @template TNegated
     * @return ($flag is not TNegated ? TNegated : int)
     */
    abstract public function negated($flag);
}

abstract class ModelCollection extends BaseCollection
{
    /**
     * {@inheritDoc}
     * @template TPadValue
     * @return list<TPadValue>
     */
    abstract public function pad($size, $value);

    /**
     * {@inheritDoc}
     * @template TZipValue
     * @return list<TZipValue>
     */
    abstract public function zip($items);

    /** @template TRenamed */
    abstract public function renamed($renamed);

    /**
     * @template TOverride
     * @param int $renamed
     */
    abstract public function overridden($renamed);

    /** @template TFresh */
    abstract public function differentTemplate($value);

    /** @template TConditional */
    abstract public function conditional($renamedFlag);

    /** @template TNegated */
    abstract public function negated($renamedFlag);
}

interface Source
{
    /**
     * @template TSource
     * @param TSource $source
     */
    public function from($source);
}

abstract class Implementation implements Source
{
    /** @template TSource */
    abstract public function from($renamed);
}

class NamedType {}

abstract class ConcreteTypeParent
{
    /** @param NamedType $value */
    abstract public function named($value);
}

abstract class ConcreteTypeChild extends ConcreteTypeParent
{
    // A class with the same short name as the child's template is not a TemplateType.
    /** @template NamedType */
    abstract public function named($value);
}

abstract class LocalTypes
{
    /**
     * @template TUnused
     * @return TUnused
     */
    abstract public function unused($value);

    /**
     * @template TOutput
     * @param-out TOutput $value
     */
    abstract public function output(&$value);

    /**
     * @template TBound
     * @template TInput of TBound
     * @param TInput $value
     */
    abstract public function bounds($value);

    /**
     * @template TDefault
     * @template TInput = TDefault
     * @param TInput $value
     */
    abstract public function defaults($value);

    /**
     * @template TConflict of object
     * @param TConflict $value
     */
    abstract public function incompatible(string $value);

    /**
     * @template TNative of object
     * @param TNative $value
     */
    abstract public function compatible(object $value);

    /**
     * @template TParamConditional
     * @return ($flag is TParamConditional ? TParamConditional : int)
     */
    abstract public function conditional($flag);

    /**
     * @template TTypeConditional
     * @return (TTypeConditional is object ? TTypeConditional : int)
     */
    abstract public function typeConditional($flag);
}

/** @template TClass */
abstract class ClassTemplate
{
    /**
     * @template TClass
     * @param TClass $value
     */
    abstract public function shadowed($value);
}

abstract class ScopedParent
{
    /**
     * @template TSeen
     * @param TSeen $value
     */
    abstract public function scoped($value);
}

abstract class ScopedChild extends ScopedParent
{
    // Inherited TSeen keeps the parent's bound; it must not be rebound to the child's TNotSeen.
    /**
     * @template TNotSeen
     * @template TSeen of TNotSeen
     */
    abstract public function scoped($value);
}

abstract class DefaultParent
{
    /**
     * @template TDefault
     * @template TInput = TDefault
     * @param TInput $value
     */
    abstract public function defaults($value);
}

abstract class DefaultChild extends DefaultParent
{
    /**
     * @template TDefault
     * @template TInput
     */
    abstract public function defaults($value);
}

abstract class NativeReturnChild extends BaseCollection
{
    // The inherited conditional does not win over an incompatible native return type.
    /** @template TConditional */
    abstract public function conditional($flag): int;
}

abstract class LocalReturnChild extends BaseCollection
{
    /**
     * @template TConditional
     * @return int
     */
    abstract public function conditional($flag);
}

abstract class NativeParent
{
    /**
     * @template TNative of object
     * @param TNative $value
     */
    abstract public function native(string $value);
}

abstract class NativeChild extends NativeParent
{
    // An inherited @param that does not narrow the native type is not the effective parameter type.
    /** @template TNative of object */
    abstract public function native(string $value);
}

abstract class PartialCollection extends BaseCollection
{
    /**
     * @template TPadValue
     * @param int $size
     */
    abstract public function pad($size, $value);
}

abstract class TypeConditionalParent
{
    /**
     * @template TTypeConditional
     * @return (TTypeConditional is object ? TTypeConditional : int)
     */
    abstract public function conditional($flag);
}

abstract class TypeConditionalChild extends TypeConditionalParent
{
    /** @template TTypeConditional */
    abstract public function conditional($flag);
}

/** @template TFunctionUnused */
function standalone(): void {}
