<?php

namespace OverridingGaps;

interface IA {}
interface IB {}
interface Unrelated {}

class TypedParent
{
	public function n(): never { throw new \Exception(); }
	public function cb(callable $c): void {}
	public function cl(\Closure $c): void {}
	public function st(): static { return $this; }
	public function inter(IA&IB $x): void {}
	public function nul(): null { return null; }
	public function fls(): false { return false; }
	public function obj(object $o): void {}
	public function it(iterable $i): void {}
}

class TypedChild extends TypedParent implements Unrelated
{
	public function n(): never { throw new \Exception(); }
	public function cb(callable $c): void {}
	public function cl(\Closure $c): void {}
	public function st(): static { return $this; }
	public function inter(IA&IB $x): void {}
	public function nul(): null { return null; }
	public function fls(): false { return false; }
	public function obj(object $o): void {}
	public function it(iterable $i): void {}
}

class PrivateGrand
{
	private function h(): void {}
}

class PublicParent extends PrivateGrand
{
	public function h(): void {}
}

class PublicChild extends PublicParent
{
	public function h(): void {}
}

class FinalByDoc
{
	/** @final */
	public function f(): void {}
}

class OverridesFinalByDoc extends FinalByDoc
{
	public function f(): void {}
}

/** @phpstan-consistent-constructor */
class ConsistentWithout
{
}

class ConsistentChild extends ConsistentWithout
{
	public function __construct(int $x) {}
}

class Deep0 extends Deep1 {}
class Deep1 extends Deep2 {}
class Deep2 extends Deep3 {}
class Deep3 extends Deep4 {}
class Deep4 extends Deep5 {}
class Deep5 extends Deep6 {}
class Deep6 extends Deep7 {}
class Deep7 extends Deep8 {}
class Deep8 extends Deep9 {}
class Deep9 extends Deep10 {}
class Deep10 extends Deep11 {}
class Deep11 extends Deep12 {}
class Deep12 extends Deep13 {}
class Deep13 extends Deep14 {}
class Deep14 extends Deep15 {}
class Deep15 extends Deep16 {}
class Deep16 extends Deep17 {}
class Deep17 extends Deep18 {}
class Deep18 extends Deep19 {}
class Deep19 extends Deep20 {}
class Deep20 extends Deep21 {}
class Deep21 extends Deep22 {}
class Deep22 extends Deep23 {}
class Deep23 extends Deep24 {}
class Deep24 extends Deep25 {}
class Deep25 extends Deep26 {}
class Deep26 extends Deep27 {}
class Deep27 extends Deep28 {}
class Deep28 extends Deep29 {}
class Deep29 extends Deep30 {}
class Deep30 extends Deep31 {}
class Deep31 extends Deep32 {}
class Deep32 extends Deep33 {}
class Deep33 extends Deep34 {}
class Deep34 extends Deep35 {}
class Deep35 extends Deep36 {}
class Deep36 extends Deep37 {}
class Deep37 extends Deep38 {}
class Deep38 extends Deep39 {}
class Deep39 extends Deep40 {}
class Deep40 extends Deep41 {}
class Deep41 extends Deep42 {}
class Deep42 extends Deep43 {}
class Deep43 extends Deep44 {}
class Deep44 extends Deep45 {}
class Deep45 extends Deep46 {}
class Deep46 extends Deep47 {}
class Deep47 extends Deep48 {}
class Deep48 extends Deep49 {}
class Deep49 extends Deep50 {}
class Deep50 extends Deep51 {}
class Deep51 extends Deep52 {}
class Deep52 extends Deep53 {}
class Deep53 extends Deep54 {}
class Deep54 extends Deep55 {}
class Deep55 extends Deep56 {}
class Deep56 extends Deep57 {}
class Deep57 extends Deep58 {}
class Deep58 extends Deep59 {}
class Deep59 extends Deep60 {}
class Deep60 extends Deep61 {}
class Deep61 extends Deep62 {}
class Deep62 extends Deep63 {}
class Deep63 extends Deep64 {}
class Deep64 extends Deep65 {}
class Deep65 extends Deep66 {}
class Deep66 extends Deep67 {}
class Deep67 extends Deep68 {}
class Deep68 extends Deep69 {}
class Deep69 extends Deep70 {}
class Deep70 {}

class DeepChild extends Deep0
{
	public function __construct(int $x) {}
}

trait MakesStatic
{
	public function make(): static { return $this; }
}

class UsesMakesStatic
{
	use MakesStatic;
}

class OverridesMakesStatic extends UsesMakesStatic
{
	public function make(): static { return $this; }
}

class MessageParent extends \ErrorException
{
}

class MessageChild extends MessageParent
{
	protected $message = 'x';
}

class TraitPropertyParent
{
	public int $p = 1;
}

trait DeclaresP
{
	public int $p = 2;
}

class TraitPropertyChild extends TraitPropertyParent
{
	use DeclaresP;
}

interface HookedInterface
{
	public string $name { set; }
}

function makes(): object
{
	return new static();
}

trait CtorTrait
{
	public function __construct()
	{
	}
}

class CtorParent
{
	use CtorTrait;
}

class CtorChild extends CtorParent
{
	public function __construct()
	{
	}
}

class FinalCtorParent
{
	/** @final */
	public function __construct()
	{
	}
}

class FinalCtorChild extends FinalCtorParent
{
	public function __construct()
	{
	}
}

class Grand
{
}

class ParentTyped extends Grand
{
	public function p(): parent
	{
		return new Grand();
	}
}

class ChildTyped extends ParentTyped
{
	public function p(): parent
	{
		return new ParentTyped();
	}
}

class WithUnknownProperty extends \ErrorException
{
	public $foo;
}

class NativeOverload extends \PDO
{
	public function query(): int
	{
		return 1;
	}
}
