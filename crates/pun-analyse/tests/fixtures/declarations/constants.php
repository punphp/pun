<?php

namespace Constants;

interface I0
{
	const A = 1;
	final const B = 2;
}

interface I1 extends I0
{
}

class Base implements I1
{
	const A = 1;
	private const P = 1;
	protected const Q = 1;
	final public const F = 1;
	/** @final */
	const D = 1;
}

class Child extends Base
{
	const A = 2;
	const B = 3;
	public const P = 2;
	private const Q = 2;
	protected const F = 2, D = 2;
}

class Direct implements I1
{
	const B = 4;
}

trait TBase
{
	public const X = 1;
	protected const Y = 'a';
	public const ?int N = 1;
	public const Z = 1;
}

trait TOuter
{
	use TBase;
}

class Parent2
{
	use TOuter;
}

class Child2 extends Parent2
{
	public const X = 2;
	private const Y = 'a';
	public const int N = 1;
	public const Z = 1;
}

class UsesOuter
{
	use TOuter;

	const X = 2;
	final public const Z = 1;
	const ?int N = 1;
}

class UsesBase
{
	use TBase;

	const X = 'x';
	const Y = 'a';
}

function () {
	return new class {
		const ANON = 1;
	};
};
