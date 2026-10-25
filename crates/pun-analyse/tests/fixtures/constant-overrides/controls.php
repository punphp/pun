<?php
namespace ConstantOverrides;

class ChildConstants extends Middle
{
    public const int NARROW = 1;
    public const int|string WIDEN = 1;
    public const MISSING = 1;
    public const int UNTYPED = 1;
    public const \DateTimeImmutable OBJECT = null;
    /** @var string */
    public const DOC = 'bad';
    public const INHERITED = 'the inherited PHPDoc still wins';
    /** No @var: cancels inherited type. */
    public const CLEARED = 'no PHPDoc type';
    /** @var string $NAMED */
    public const NAMED = 'named tags are not constant types';
    /** @var int */
    public const int EFFECTIVE = 1;
    /** @var positive-int */
    public const int BAD_DOC = 1;
    public const NO_NATIVE_DOC = 'the inherited PHPDoc wins again';
    /** @var array{a: int|string, b?: string} */
    public const SHAPE = ['a' => 1];
    /** @var string */
    public const PRIVATE_DOC = 'not an override';
    private const int|string PROTECTED_NATIVE = 1;
    protected const FINAL_NATIVE = 1;
}

class InterfaceChild implements ImmediateContract
{
    /** @var string */
    public const string FROM_INTERFACE = 'bad';
}

class TraitChild extends TraitParent
{
    /** @var int */
    public const int|string FROM_TRAIT = 1;
}

class ChainChild extends ParentWithoutTag
{
    /** @var string */
    public const DOC_CHAIN = 'bad';
}

class UntypedParent { public const VALUE = 1; }
class DocAdded extends UntypedParent
{
    /** @var string */
    public const VALUE = 'no prototype PHPDoc';
}

class SelfParent { public const self INSTANCE = null; }
class SelfChild extends SelfParent { public const self INSTANCE = null; }
class SelfWiden extends SelfChild { public const parent INSTANCE = null; }

interface FirstContract { public const int ORDER = 1; }
interface SecondContract { public const string ORDER = 's'; }
class InterfacePriority extends UntypedParent implements FirstContract, SecondContract
{
    public const int ORDER = 1;
}

class MultiParent
{
    /** @var int */
    public const int A = 1, B = 2;
}
class MultiChild extends MultiParent
{
    /** @var string */
    public const string A = 'a', B = 'b';
}

class NullableParent { public const ?int VALUE = null; }
class NullableChild extends NullableParent { public const int VALUE = 1; }
class NullableWiden extends NullableChild { public const ?int VALUE = null; }

trait OverrideTrait
{
    /** @var string */
    public const string DOC_CHAIN = 'bad';
}
class TraitOverride extends Grandparent { use OverrideTrait; }

class OwnTraitOverride extends Grandparent
{
    use OverrideTrait;
    /** @var int */
    public const DOC_CHAIN = 1;
}

interface DifferentContract
{
    /** @var string */
    public const string DOC_CHAIN = 's';
}
class TraitInterfaceOverride extends Grandparent implements DifferentContract { use OverrideTrait; }
class PrivateParent { private const int DOC_CHAIN = 1; }
class PrivateTraitOverride extends PrivateParent implements DifferentContract { use OverrideTrait; }

class GrandchildAfterTrait extends TraitOverride
{
    /** @var int */
    public const DOC_CHAIN = 1;
}
