<?php

namespace InvalidUnsetExtra;

class Open
{
	public int $public = 1;
	protected int $protected = 1;
	private int $private = 1;
	final public int $final = 1;
	public readonly int $readonly;
	/** @readonly */
	public int $docReadonly = 1;
	public static int $static = 1;
	public $untyped;
	public int $hooked { get => 1; }
	public array $list = [];

	public function __construct()
	{
		$this->readonly = 1;
	}

	public function run(self $other, ?self $maybe, string $name): void
	{
		unset($this->public);
		unset($this->protected);
		unset($this->private);
		unset($this->final);
		unset($this->readonly);
		unset($this->docReadonly);
		unset($this->hooked);
		unset($this->untyped);
		unset($other->public, $other->private);
		unset($maybe->public);
		unset($this->$name);
		unset($this->missing);
		unset(self::$static);
		unset($this->list[0]);
		unset($this->list['a']['b']);
	}
}

final class Closed
{
	public int $public = 1;

	public function run(): void
	{
		unset($this->public);
	}
}

/** @readonly */
class DocReadonlyClass
{
	public int $a = 1;

	public function run(): void
	{
		unset($this->a);
	}
}

readonly class ReadonlyClass
{
	public int $a;

	public function __construct()
	{
		$this->a = 1;
	}

	public function run(): void
	{
		unset($this->a);
	}
}

interface HasHook
{
	public int $hook { get; }
}

function variables(bool $b, int $i, string $s, array $a, \ArrayAccess $access, mixed $m): void
{
	unset($undefined);
	if ($b) {
		$maybe = 1;
	}
	unset($maybe);
	$defined = 1;
	unset($defined);
	unset($defined);
	unset($i[0]);
	unset($s[0]);
	unset($a['x']);
	$shape = ['a' => 1];
	unset($shape['b']);
	unset($shape['a']['b']);
	unset($shape['b']['c']);
	$nested = ['a' => ['b' => 1]];
	unset($nested['a']['c']);
	unset($nested['x']['c']);
	unset($access['x']);
	unset($m['x']);
	$list = [1, 2];
	unset($list[5]);
	unset($list[1]);
	unset($$s);
	unset($GLOBALS['x']);
	$null = null;
	unset($null['x']);
	unset($undefinedArray['x']);
}

function hooks(HasHook $h): void
{
	unset($h->hook);
}
