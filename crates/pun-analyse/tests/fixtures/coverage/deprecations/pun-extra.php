<?php

namespace PunDeprecations;

const PLAIN = 1;

/**
 * @deprecated Use NEW_ONE.
 */
const OLD_ONE = 1;

/** @deprecated %s is old */
const FORMATTED = 2;

/** @deprecated */
define('PunDeprecations\DEFINED', 3);

/**
 * @deprecated first line
 *   second line
 *
 * after a blank line
 * @param int $x
 */
function multi(int $x): void
{
}

/**
 * @deprecated
 * @not-deprecated
 */
function both(): void
{
}

interface Contract
{
	/** @deprecated Use other(). */
	public function fromInterface(): void;

	/** @deprecated Interface constant. */
	public const FROM_INTERFACE = 1;
}

trait AbstractTrait
{
	/** @deprecated From the trait. */
	abstract public function fromTrait(): void;
}

class Base
{
	/** @deprecated */
	public $old;

	/** @deprecated Parent constant. */
	public const INHERITED = 1;

	/** @deprecated */
	public function inherited(): void
	{
	}

	/** @deprecated */
	public static $staticOld;

	/** @deprecated */
	public function __toString(): string
	{
		return '';
	}
}

class Child extends Base implements Contract
{
	use AbstractTrait;

	public $old;

	/** @not-deprecated */
	public function inherited(): void
	{
	}

	public function fromInterface(): void
	{
	}

	public function fromTrait(): void
	{
	}

	public function __toString(): string
	{
		return '';
	}
}

/**
 * @deprecated The whole class.
 */
class Gone
{
	public $prop;

	public static $static;

	public const C = 1;

	public static function make(): void
	{
	}

	public function run(): void
	{
		multi(1);
	}
}

/** @deprecated */
interface GoneInterface
{
}

/** @deprecated */
trait GoneTrait
{
	public function inTrait(): void
	{
		multi(1);
	}
}

class UsesGoneTrait
{
	use GoneTrait;
}

/** @deprecated */
enum GoneEnum
{
	case A;
}

/** @deprecated */
#[\Attribute]
class GoneAttribute
{
}

/** @deprecated */
class GoneException extends \Exception
{
}

/** @deprecated */
class DeprecatedChild extends Gone implements GoneInterface
{
	use GoneTrait;
}

function uses(Child $child, Gone $gone, Base $base): Gone
{
	echo PLAIN, OLD_ONE, FORMATTED, DEFINED, \E_STRICT, \FILTER_SANITIZE_STRING, E_STRICT;
	multi(1);
	both();
	$child->inherited();
	$child->fromInterface();
	$child->fromTrait();
	echo $child->old;
	echo Child::INHERITED;
	echo Child::FROM_INTERFACE;
	echo (string) $child;
	echo (string) $base;
	echo $gone->prop;
	echo Gone::$static;
	echo Gone::C;
	Gone::make();
	$gone->run();
	echo Child::$staticOld;
	echo $base::$staticOld;
	new Gone();
	if ($gone instanceof Gone) {
	}
	try {
	} catch (GoneException $e) {
	}
	$f = #[GoneAttribute] function (GoneEnum $e): GoneInterface {
	};
	ini_set('assert.active', '1');
	ini_get('session.sid_length');
	ini_get('display_errors');
	return $gone;
}

class TypedProperties
{
	public Gone $gone;
}
