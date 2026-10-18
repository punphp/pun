<?php

namespace Keys;

const G1 = 'g';
const G2 = 'g';

class Base
{
	const PA = 'pa';
}

class K extends Base
{
	const A = 'a';
	const B = 'a';
	const C = 1;
	const ARR = [1];
	const T = true;
	const N = null;

	public function f(int $i): void
	{
		$a = [G1 => 1, G2 => 2];
		$b = [
			self::A => 1,
			K::B => 2,
			K::class => 3,
			'Keys\K' => 4,
			self::C => 5,
			'01' => 6,
			'1' => 7,
		];
		$c = array(1, 2, 0 => 3);
		[$x, $y] = [1, 2];
		['a' => $x, 'a' => $y] = ['a' => 1];
		list('b' => $x, 'b' => $y) = ['b' => 1];
		list('c' => [1 => $x, 1 => $y]) = ['c' => [1 => 1]];
		foreach ([] as ['k' => $p, 'k' => $q]) {
		}
		$d = [$i => 1, 5 => 2, 3, 6 => 4];
		$e = [$i => 1, 0 => 2, 3, 1 => 4];
		$f = [...[1], 0 => 2];
		$g = ['a\'b' => 1, "a'b" => 2, -1 => 3, -1.5 => 4];
		$h = [null => 1, '' => 2, false => 3, 0 => 4, 0.5 => 5];
		$j = [('p') => 1, 'p' => 2, \Keys\G1 => 3, G2 => 4, +3 => 5, 3 => 6];
		$k = [~1 => 'a', 'b', -$i => 'c', static::A => 'd', self::{'A'} => 'e', self::ARR => 'f'];
		$l = [parent::PA => 1, Base::PA => 2, self::T => 3, 1 => 4, self::N => 5, '' => 6];
		$m = [PHP_INT_MAX => 1, \PDO::ATTR_ERRMODE => 2];
	}
}
