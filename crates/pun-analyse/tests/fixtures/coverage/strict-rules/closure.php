<?php

namespace StrictClosure;

class Foo
{
	public function doFoo(): void
	{
		$self = $this;
		$other = new \stdClass();
		$fn = function () use ($self, $other) {
			return [$self, $other];
		};
		$static = static function () use ($self) {
			return $self;
		};
		$byRef = function () use (&$self) {
			return $self;
		};
		$bound = \Closure::bind(function () use ($self) {
			return $self;
		}, null, Foo::class);
		$self = 1;
		$after = function () use ($self) {
			return $self;
		};
		$nested = function () {
			$that = $this;
			return function () use ($that) {
				return $that;
			};
		};
	}

	public function doNewStatic(): void
	{
		$st = new static();
		$fn = function () use ($st) {
			return $st;
		};
		$this->doFoo();
		$self = $this;
		if (rand(0, 1)) {
			$self = null;
		}
		$maybe = function () use ($self) {
			return $self;
		};
	}

	public static function doStatic(): void
	{
		$x = new self();
		$fn = function () use ($x) {
			return $x;
		};
	}
}

function outside(): void
{
	$fn = function () use ($undefined) {
		return $undefined;
	};
}
