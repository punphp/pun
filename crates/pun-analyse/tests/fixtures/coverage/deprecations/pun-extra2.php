<?php

namespace PunDeprecations2;

/** @deprecated Use the new attribute. */
#[\Attribute]
class OldAttribute
{
}

#[OldAttribute]
class Attributed
{
}

class MyParameter extends \ReflectionParameter
{
	public function isArray(): bool
	{
		return false;
	}
}

/** @deprecated */
class GoneParameter extends \ReflectionParameter
{
}

enum Status
{
	/** @deprecated Use New. */
	case Old;
	case New;
}

class ConstBase
{
	/** @deprecated */
	public const X = 1;

	/** @deprecated */
	private const P = 1;
}

class ConstChild extends ConstBase
{
	public const X = 2;

	public const P = 2;
}

class Stringy
{
	/** @deprecated Cast something else. */
	public function __toString(): string
	{
		return '';
	}
}

/** @deprecated */
class Gone
{
	public static $s;

	public static function m(): void
	{
	}
}

/** @deprecated */
trait GoneTrait
{
}

function uses(
	\ReflectionParameter $p,
	MyParameter $mine,
	GoneParameter $gone,
	Stringy $stringy,
	callable $f,
	string $option,
): void {
	$p->isArray();
	$mine->isArray();
	$gone->isArray();
	echo Status::Old;
	echo ConstChild::X;
	echo ConstChild::P;
	echo (string) $stringy;
	echo Gone::$missing;
	Gone::missing();
	$f('assert.active');
	ini_get();
	ini_get($option);
	echo utf8_encode('x');
	new class {
		use GoneTrait;
	};
}
