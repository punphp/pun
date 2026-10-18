<?php

namespace SignatureHooks;

class C {}
final class F {}

new class {
	public int $anonymous {
		set (int|string $value) {
		}
	}
};

class Holder
{
	public F $plain {
		set (C&F $value) {
		}
	}

	public F $get {
		get {
			return new F();
		}
	}

	public function __construct(
		public F $promoted {
			set (Missing $value) {
			}
		},
	) {}
}
