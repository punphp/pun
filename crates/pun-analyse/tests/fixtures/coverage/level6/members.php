<?php

namespace CoverageMembers;

/**
 * @method array magic(array $a)
 */
class MagicParent
{
}

class MagicChild extends MagicParent
{
    public function magic($a)
    {
        return [];
    }
}

class Templated
{
    /**
     * @template T of array
     * @param T $value
     * @return T
     */
    public function same($value)
    {
        return $value;
    }

    /**
     * @template T
     * @param T|null $value
     */
    public function maybe($value = null): void
    {
    }
}

class PropParent
{
    /** @var array<int, int> */
    public $typed = [];

    /** @var array */
    public $loose = [];

    /** @var array<int, int>|null */
    public static ?array $nativeTyped = [];

    /** @var array<int, int> */
    private $secret = [];
}

interface PropInterface
{
}

class PropChild extends PropParent implements PropInterface
{
    public $typed = [];

    public $loose = [];

    public static ?array $nativeTyped = [];

    public $secret = [];

    public $fresh;
}

class ExceptionChild extends \Exception
{
    protected $message = '';
}

class ConstParent
{
    /** @var array<int, int> */
    const TYPED = [];

    /** @var array */
    const LOOSE = [];

    /** @var array<int, int> */
    private const SECRET = [];
}

interface ConstInterface
{
    /** @var array */
    const FROM_INTERFACE = [];
}

class ConstChild extends ConstParent implements ConstInterface
{
    const TYPED = [];

    const LOOSE = [];

    const SECRET = [];

    const FROM_INTERFACE = [];

    /**
     * @var array
     * @var array<int, int>
     */
    const TWICE = [];
}

/**
 * @template T
 */
class GenericBase
{
}

/**
 * @template U
 * @extends GenericBase<U>
 */
class GenericMiddle extends GenericBase
{
}

/**
 * @extends GenericMiddle<array>
 */
class GenericLeaf extends GenericMiddle
{
}

class LeafWithout extends GenericMiddle
{
}

class OnUnknown extends UnknownParent
{
    public function f(array $a): void
    {
    }
}
