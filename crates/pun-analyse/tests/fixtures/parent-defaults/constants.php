<?php
namespace ParentGenericDefaults;

class AncestorConstants
{
    public const VALUE = 41;
    public const INHERITED = 47;
}

class ParentConstants extends AncestorConstants
{
    public const VALUE = 42;
}

class OtherConstants
{
    public const VALUE = 49;
}

/** @template T of int = parent::VALUE */
class ParentDefault extends ParentConstants
{
    public const VALUE = 43;
    /** @var T */
    public $value;
}

/** @template T of int = PaReNt::VALUE */
class MixedCaseParentDefault extends ParentConstants
{
    public const VALUE = 44;
    /** @var T */
    public $value;
}

/** @template T of int = parent::INHERITED */
class InheritedConstantDefault extends ParentConstants
{
    public const INHERITED = 48;
    /** @var T */
    public $value;
}

/** @template T of int = parent::VALUE */
class EmptyConstructorDefault extends ParentConstants
{
    public const VALUE = 45;
    /** @var T */
    public $value;
    public function __construct() {}
}

/**
 * @template Z of int = parent::VALUE
 * @template A = array{value: Z}
 */
class DependentParentDefault extends ParentConstants
{
    public const VALUE = 46;
    /** @var A */
    public $nested;
}

/** @template T of string = parent::class */
class ParentClassDefault extends ParentConstants
{
    /** @var T */
    public $value;
}

/** @template T of string = pArEnT::cLaSs */
class MixedCaseParentClassDefault extends ParentConstants
{
    /** @var T */
    public $value;
}

/** @template T of int = self::VALUE */
class SelfDefault extends ParentConstants
{
    public const VALUE = 50;
    /** @var T */
    public $value;
}

/** @template T of string = self::class */
class SelfClassDefault extends ParentConstants
{
    /** @var T */
    public $value;
}

/** @template T of int = OtherConstants::VALUE */
class QualifiedDefault extends ParentConstants
{
    public const VALUE = 51;
    /** @var T */
    public $value;
}

/** @template T */
class ConstructorParent extends ParentConstants
{
    public const VALUE = 52;
    /** @param T $value */
    public function __construct(public mixed $value) {}
}

/**
 * @template T of int = parent::VALUE
 * @extends ConstructorParent<string>
 */
class InheritedConstructorDefault extends ConstructorParent
{
    public const VALUE = 53;
    /** @var T */
    public $id;
}

/** @param 42 $value */
function needs42(int $value): void {}
/** @param 43 $value */
function needs43(int $value): void {}
/** @param 47 $value */
function needs47(int $value): void {}
/** @param 48 $value */
function needs48(int $value): void {}

function defaults(string $text): void
{
    $parent = new ParentDefault();
    \PHPStan\dumpType($parent);
    \PHPStan\dumpType($parent->value);
    needs42($parent->value);
    needs43($parent->value);
    $mixedCase = new MixedCaseParentDefault();
    \PHPStan\dumpType($mixedCase);
    \PHPStan\dumpType($mixedCase->value);
    $inherited = new InheritedConstantDefault();
    \PHPStan\dumpType($inherited);
    \PHPStan\dumpType($inherited->value);
    needs47($inherited->value);
    needs48($inherited->value);
    $empty = new EmptyConstructorDefault();
    \PHPStan\dumpType($empty);
    \PHPStan\dumpType($empty->value);
    $dependent = new DependentParentDefault();
    \PHPStan\dumpType($dependent);
    \PHPStan\dumpType($dependent->nested);
    $className = new ParentClassDefault();
    \PHPStan\dumpType($className);
    \PHPStan\dumpType($className->value);
    $mixedClassName = new MixedCaseParentClassDefault();
    \PHPStan\dumpType($mixedClassName);
    \PHPStan\dumpType($mixedClassName->value);
    $self = new SelfDefault();
    \PHPStan\dumpType($self);
    \PHPStan\dumpType($self->value);
    $selfClass = new SelfClassDefault();
    \PHPStan\dumpType($selfClass);
    \PHPStan\dumpType($selfClass->value);
    $qualified = new QualifiedDefault();
    \PHPStan\dumpType($qualified);
    \PHPStan\dumpType($qualified->value);
    $constructor = new InheritedConstructorDefault($text);
    \PHPStan\dumpType($constructor);
    \PHPStan\dumpType($constructor->id);
}

class DifferentCaller extends OtherConstants
{
    public function fromOtherScope(): void
    {
        $parent = new ParentDefault();
        \PHPStan\dumpType($parent);
        \PHPStan\dumpType($parent->value);
    }
}
