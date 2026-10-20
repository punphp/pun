<?php

namespace CoverageHooks;

class Foo
{
	public int $a {
		get => $this->a;
		set {
			if (rand(0, 1)) {
				$this->a = 1;
			} elseif (rand(0, 1)) {
				$this->a = 2;
			} else {
				$this->a = 3;
			}
		}
	}

	public int $b {
		get => $this->b;
		set {
			if (rand(0, 1)):
				$this->b = 1;
			elseif (rand(0, 1)):
				return;
			else:
				$this->b = 3;
			endif;
		}
	}

	public int $c {
		get => $this->c;
		set {
			while (rand(0, 1)) {
				$this->c = 1;
			}
		}
	}

	public int $d {
		get => $this->d;
		set {
			do {
				$this->d = 1;
			} while (rand(0, 1));
		}
	}

	public int $e {
		get => $this->e;
		set {
			foreach (range(0, $value) as $x):
				$this->e = $x;
			endforeach;
			$this->e = $value;
			for ($i = 0; $i < rand(0, 9); $i++):
			endfor;
			while (rand(0, 1)):
			endwhile;
			foreach (range(0, $value) as $y) {
			}
			for ($i = 0; $i < rand(0, 9); $i++) {
			}
		}
	}

	public int $f {
		get => $this->f;
		set {
			if ($this->f = $value) {
			}
		}
	}

	public int $g {
		get => $this->g;
		set {
			return $this->g = $value;
		}
	}

	public int $h {
		get => $this->h;
		set {
			{
				exit(1);
			}
		}
	}

	public int $i {
		get {
			$this->i = 1;
			return 1;
		}
		set {
			echo $value;
			$this->i = $value;
		}
	}

	public int $j {
		get {
			$this->j += 1;
			return 1;
		}
		set => $value;
	}

	public int $k {
		get => 1;
		set {
			try {
				$this->k = $value;
			} catch (\Throwable $e) {
			}
		}
	}

	public int $l {
		get {
			$fn = fn () => $this->l;
			return 1;
		}
		set => $value;
	}

	public int $m {
		set {
			if (rand(0, 1)) {
				return;
			}
			die;
		}
	}

	public int $o {
		get => $this->o;
		set {
			return;
			$this->o = $value;
		}
	}

	public int $p {
		get => $this->p;
		set {
			throw new \Exception();
		}
	}

	public int $n {
		get => 1;
		set => $value;
		get => 2;
	}

	public function __construct(
		public int $promoted {
			get => 1;
			set => $value;
		},
	) {
	}
}

trait HookTrait
{
	public int $t {
		get => 1;
		set => $value;
	}
}

class UsesHookTrait
{
	use HookTrait;
}

abstract class AbstractHooks
{
	abstract public int $v {
		get;
	}

}
