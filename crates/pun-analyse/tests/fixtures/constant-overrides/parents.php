<?php
namespace ConstantOverrides;

use DateTimeInterface as Clock;

class ParentConstants
{
    public const int|string NARROW = 1;
    public const int WIDEN = 1;
    public const int MISSING = 1;
    public const UNTYPED = 1;
    public const Clock OBJECT = null;
    /** @var int */
    public const DOC = 1;
    /** @var int */
    public const INHERITED = 1;
    /** @var int */
    public const CLEARED = 1;
    /** @var int */
    public const NAMED = 1;
    /** @var positive-int */
    public const int EFFECTIVE = 1;
    /** @var string */
    public const int BAD_DOC = 1;
    /** @var int */
    public const NO_NATIVE_DOC = 1;
    /** @var array{a: int, b?: string} */
    public const SHAPE = ['a' => 1];
    /** @var int */
    private const PRIVATE_DOC = 1;
    protected const int PROTECTED_NATIVE = 1;
    final public const int FINAL_NATIVE = 1;
}

class Middle extends ParentConstants {}

interface RootContract
{
    /** @var int */
    public const int FROM_INTERFACE = 1;
}
interface ImmediateContract extends RootContract {}

trait ConstantTrait
{
    /** @var positive-int */
    public const int FROM_TRAIT = 1;
}
class TraitParent { use ConstantTrait; }

class Grandparent
{
    /** @var int */
    public const DOC_CHAIN = 1;
}
class ParentWithoutTag extends Grandparent
{
    public const DOC_CHAIN = 2;
}
