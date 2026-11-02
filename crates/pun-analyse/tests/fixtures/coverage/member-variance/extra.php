<?php

namespace MemberVarianceExtra;

/** @template T */
interface Inv {}

/** @template-covariant T */
interface Out {}

/** @template-contravariant T */
interface In {}

/**
 * @template-covariant X
 * @template-contravariant Y
 */
class Props
{
	/** @var X */
	public $plain;
	/** @var X */
	private $private;
	/** @var X */
	public readonly mixed $readonly;
	/** @var X */
	public private(set) mixed $privateSet;
	/** @var X */
	protected private(set) mixed $protectedPrivateSet;
	/** @var X */
	public mixed $getOnly { get => $this->plain; }
	/** @var X */
	public mixed $withSet { get => $this->plain; set => $value; }
	/** @var X */
	public static $static;
	/** @var Y */
	public $contra;
	/** @var Y */
	public readonly mixed $contraReadonly;
	/** @var In<X> */
	public readonly mixed $nestedIn;
	/** @var Out<Y> */
	public readonly mixed $nestedOut;
	/** @var Inv<X> */
	public readonly mixed $nestedInv;
	/** @var array<X> */
	public readonly array $arrayX;
	/** @var callable(X): Y */
	public readonly mixed $callable;

	/**
	 * @param X $a
	 * @param X $b
	 * @param Y $c
	 */
	public function __construct(
		public $a,
		public readonly mixed $b,
		/** @readonly */
		public $c,
		/** @var X */
		protected $d = null,
	) {
	}
}

/**
 * @template-covariant X
 * @readonly
 */
class DocReadonly
{
	/** @var X */
	public $x;
}

/** @template-covariant X */
readonly class NativeReadonly
{
	/** @param X $x */
	public function __construct(public mixed $x) {}
}

/** @template-covariant X */
class Methods
{
	/** @param X $x */
	public function param($x): void {}
	/** @param X $x */
	private function privateParam($x): void {}
	/** @param X $x */
	public static function staticParam($x): void {}
	/** @return In<X> */
	public function returnIn() {}
	/** @param Out<X> $x */
	public function paramOut($x): void {}
	/** @param In<X> $x */
	public function paramIn($x): void {}
	/** @param X $x */
	final public function finalParam($x): void {}
	/**
	 * @template Z of X
	 * @param Z $z
	 */
	public function bound($z): void {}
	/** @param-out X $x */
	public function out(&$x): void {}
}

/**
 * @template-covariant T
 * @param T $t
 * @return T
 */
function functionCovariant($t) { return $t; }

/**
 * @template-contravariant T
 * @param T $t
 * @return T
 */
function functionContravariant($t) { return $t; }

/** @template-covariant X */
class ConstructorReadonly
{
	/**
	 * @readonly
	 * @param X $x
	 */
	public function __construct(public $x) {}
}

/** @template-covariant X */
class ConstructorPhpstanReadonly
{
	/**
	 * @phpstan-readonly
	 * @param X $x
	 */
	public function __construct(public $x, /** @phpstan-readonly */ public $y = null) {}
}

/**
 * @template-covariant X
 * @phpstan-immutable
 */
class Immutable
{
	/** @var X */
	public $x;
}

/** @template-covariant X */
class InheritsConstructorDoc extends ConstructorReadonly
{
	/** @param X $x */
	public function __construct(public $x, public $z = null) {}
}

/** @template-covariant X */
trait ReadonlyConstructorTrait
{
	/**
	 * @psalm-readonly
	 * @param X $x
	 */
	public function __construct(public $x) {}
}

/** @template-covariant X */
class UsesReadonlyConstructor
{
	/** @use ReadonlyConstructorTrait<X> */
	use ReadonlyConstructorTrait;
}
