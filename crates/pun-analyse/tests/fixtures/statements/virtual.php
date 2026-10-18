<?php

namespace Virtual;

function rsort(&$array): void
{
}

class Sorts
{
	public function m(): void
	{
		sort($this['a']);
		\array_push($this['b'], 1);
		array_push($this['c']);
		array_pop($this);
		array_walk($this['d'], function (&$v) {
		});
		array_walk($this['e'], function ($v) {
		});
		array_walk($this['f'], fn (&$v) => 1);
		array_walk($this['g']);
		usort(callback: fn ($a, $b) => 0, array: $this['h']);
		ksort($this['i'], flags: 1);
		rsort($this['j']);
		strlen($this['k']);
	}
}
