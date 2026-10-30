<?php

class NativeMixedParent
{
    public mixed $inherited;

    public function inheritedReturn(): mixed { return 1; }
}

final class NativeMixedProperties extends NativeMixedParent
{
    public mixed $prop;
    public $untyped;
    /** @var mixed */
    public mixed $documentedMixed;
    /** @var int */
    public mixed $documentedInt;
    /** @var mixed */
    public $documentedOnly;

    public function declaredReturn(): mixed { return 1; }
    public function untypedReturn() { return 1; }
    /** @return mixed */
    public function documentedReturn() { return 1; }

    public function properties(mixed $explicit, $implicit, NativeMixedProperties $object): void
    {
        $this->prop; // mixed: direct-property
        ($this)->prop; // mixed: parenthesized-receiver
        $this->inherited; // mixed: inherited-property
        $this->documentedMixed; // mixed: documented-mixed-declaration
        $this->documentedInt; // mixed: documented-integer-declaration
        $this->documentedOnly; // mixed: documented-without-declaration
        $this->untyped; // mixed: untyped-property
        $this->prop['key']; // mixed: property-offset
        $this->prop['key']['nested']; // mixed: nested-property-offset
        $object->prop; // mixed: ordinary-receiver
        $object->prop['key']; // mixed: ordinary-property-offset
        $receiver = $this;
        $receiver->prop; // mixed: receiver-copy
        $newObject = new NativeMixedProperties();
        $newObject->prop; // mixed: constructed-receiver
        $copy = $this->prop;
        $copy; // mixed: property-copy
        $copy['key']; // mixed: copied-offset
        $alias =& $this->prop;
        $alias; // mixed: property-alias
        $alias['key']; // mixed: aliased-offset
        $newAlias =& $newObject->prop;
        $newAlias; // mixed: ordinary-property-alias
        $explicit; // mixed: explicit-parameter
        $implicit; // mixed: implicit-parameter
        $explicit['key']; // mixed: explicit-offset
        $implicit['key']; // mixed: implicit-offset
        $explicitAlias =& $explicit;
        $explicitAlias; // mixed: explicit-parameter-alias
        $this->declaredReturn(); // mixed: declared-method-return
        $this->inheritedReturn(); // mixed: inherited-method-return
        $this->untypedReturn(); // mixed: untyped-method-return
        $this->documentedReturn(); // mixed: documented-method-return
        $methodCopy = $this->declaredReturn();
        $methodCopy; // mixed: method-return-copy
    }

    public function captures(mixed $explicit, $implicit): void
    {
        $copy = $this->prop;
        $direct = fn () => $this->prop;
        $offset = fn () => $this->prop['key'];
        $copied = fn () => $copy;
        $copiedOffset = fn () => $copy['key'];
        $byValue = function () use ($copy) {
            $copy; // mixed: closure-value-capture
            $arrow = fn () => $copy;
        };
        $copy; // mixed: after-value-capture
        $byReference = function () use (&$copy) {
            $copy; // mixed: closure-reference-capture
            $arrow = fn () => $copy;
        };
        $copy; // mixed: after-reference-capture
        $this->prop; // mixed: original-property-after-reference-capture
        $methodCopy = $this->declaredReturn();
        $method = fn () => $methodCopy;
        $parameter = fn () => $explicit;
        $untyped = fn () => $implicit;
        $value = function () use ($explicit, $implicit) {
            $explicit; // mixed: explicit-value-capture
            $implicit; // mixed: implicit-value-capture
        };
        $reference = function () use (&$implicit) {
            $implicit; // mixed: implicit-reference-capture
        };
    }

    public function unknownProperty(mixed $receiver): void
    {
        $receiver->unknown; // mixed: unknown-property
    }

    public function unknownMethod(mixed $receiver): void
    {
        $receiver->unknown(); // mixed: unknown-method
    }
}
