<?php

namespace ReadonlyFlow;

function stop(): never
{
	throw new \Exception();
}

class Parentish
{
	public function inherited(): void
	{
	}
}

final class Conditions extends Parentish
{
	private readonly int $a;
	private readonly int $b;
	private readonly int $c;
	private readonly int $d;
	private readonly int $e;
	private readonly int $f;
	private readonly int $g;
	private readonly int $h;
	private readonly int $i;
	private readonly int $j;
	private readonly int $k;
	private readonly int $l;
	private readonly int $m;
	private readonly array $n;
	private readonly array $o;
	private readonly int $p;

	public function __construct(int $x, array $list)
	{
		if ($x === 1) {
			if (throw new \Exception()) {
				$this->a = 1;
			}
			return;
		}
		if ($x === 2):
			$this->a = 2;
		elseif ($x === 3):
			$this->a = 3;
		else:
			$this->a = 4;
		endif;
		if ($x === 5) {
			$this->b = 1;
		} elseif (stop()) {
			$this->b = 2;
		}
		switch ($x):
			case 1:
				$this->c = 1;
				break;
			default:
				$this->c = 2;
		endswitch;
		while ($x > 10):
			echo $this->d;
		endwhile;
		for ($y = 0; $y < 1; $y++):
			echo $this->e;
		endfor;
		foreach ($list as $item):
			echo $this->f;
		endforeach;
		do {
			$this->g = 1;
		} while ($x > 100);
		echo $this->h ?? 1;
		$this->i = $x > 3 ? 1 : 2;
		echo ($this->j);
		get_object_vars($this);
		$this->n[$x] = 1;
		$this->o[] = 2;
		[$first] = $list;
		$list[$x] = 3;
		$other = new Parentish();
		$other::class;
		$this->inherited();
		$this->k = 1;
		$this->l = 1;
		$this->m = 1;
		$this->p = 1;
	}
}

final class Exits
{
	private readonly int $a;

	public function __construct(int $x)
	{
		if ($x === 1) {
			switch (stop()) {
			}
		}
		if ($x === 2) {
			match (stop()) {
				default => 1,
			};
		}
		if ($x === 3) {
			while (stop()) {
			}
		}
		if ($x === 4) {
			for ($i = stop(); ;) {
			}
		}
		if ($x === 5) {
			foreach (stop() as $v) {
			}
		}
		if ($x === 6) {
			do {
				stop();
			} while (true);
		}
		if ($x === 7) {
			return stop();
		}
		if ($x === 8) {
			stop() && $x;
		}
		if ($x === 9) {
			stop() ? 1 : 2;
		}
		if ($x === 10) {
			$this->a = stop();
		}
		if ($x === 11) {
			$y = [];
			$y[stop()] = 1;
		}
		if ($x === 12) {
			[$z] = stop();
		}
		if ($x === 13) {
			$w = [];
			$w[stop()] = 1;
		}
		$this->a = 1;
	}
}

final class Dims
{
	private readonly array $a;

	public function __construct(int $x)
	{
		if ($x === 1) {
			$this->a[stop()] = 1;
		}
		if ($x === 2) {
			$this->a[] = stop();
		}
		$this->a = [];
	}
}

final class Hints
{
	/** @readonly */
	public int $withDefault = 1;

	public readonly (\Countable&\ArrayAccess)|null $dnf;

	public function __construct()
	{
		$this->dnf[1] = 1;
		$this->dnf = null;
	}
}

final class UpperConstructor
{
	private readonly int $a;
	private readonly int $b;

	public function __CONSTRUCT()
	{
		$this->a = 1;
		echo $this->a;
		$this->init();
	}

	private function init(): void
	{
		$this->b = 1;
	}
}

trait ReadsEarly
{
	public function readEarly(): void
	{
		echo $this->early;
	}
}

final class UsesReadsEarly
{
	use ReadsEarly;

	private readonly int $early;

	public function __construct()
	{
		$this->readEarly();
		$this->early = 1;
	}
}

trait DeclaresClasses
{
	public function make(): object
	{
		return new class {
			private readonly int $x;
		};
	}
}

final class UsesDeclaresClasses
{
	use DeclaresClasses;
}
