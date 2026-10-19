<?php

namespace Rwo;

/**
 * @property-write int $w
 * @property-read int $r
 */
#[\AllowDynamicProperties]
class Foo
{
	public int $virt { set { } }
	public int $both { get => 1; set { } }
	public static int $s = 1;
	private int $priv { set { } }

	public function doFoo(): void
	{
		echo $this->w;
		$this->w = 1;
		$this->w .= 1;
		$this->w ??= 1;
		$this->w[] = 1;
		$this->w['a'] = 1;
		[$this->w] = [1];
		$x = &$this->w;
		echo $this?->w;
		echo $this->virt;
		$this->virt = 1;
		$this->virt++;
		echo $this->both;
		echo self::$s;
		echo $this->priv;
		echo isset($this->w);
		unset($this->w);
		foreach ([1] as $this->w) {}
		$f = fn () => $this->w;
		$this->w->foo = 1;
		echo $this->w->foo;
	}
}

class Bar extends Foo
{
	public function doBar(Foo $f): void
	{
		echo $this->w;
		echo $this->virt;
		echo $f->w;
	}
}

function (Foo $foo) {
	echo $foo->w;
};
