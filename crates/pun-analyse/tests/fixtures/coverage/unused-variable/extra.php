<?php declare(strict_types = 1);

namespace UnusedVariableExtra;

/** @phpstan-impure */
function cond(): bool
{
	return (bool) rand(0, 1);
}

function sink(mixed $v): void
{
}

class WithDestructor
{
	public function __destruct()
	{
		echo 'gone';
	}
}

/** @throws \DomainException */
function mayThrow(): void
{
}

function kinds(): void
{
	$a = 1;
	$a++;
	$b = 1;
	++$b;
	$c = 1;
	$c--;
	$d = 1;
	--$d;
	foreach ([1, 2] as $k => $v) {
	}
	foreach ([1, 2] as $w) {
	}
	try {
		mayThrow();
	} catch (\DomainException $e) {
	}
	$_ignored = 1;
	[$x, $y] = [1, 2];
	sink($x);
	sink(++$b);
}

function consumedIncrement(): int
{
	$i = 0;
	return ++$i;
}

function redundantKeys(): array
{
	$a = [true => 1, false => 2];
	$a[true] = 1;
	return $a;
}

function redundantEmptyKey(): array
{
	$a = ['' => 2, '1' => 3];
	$a[''] = 2;
	return $a;
}

function redundantNested(): array
{
	$a = [[1, [2]]];
	$a[0][1][0] = 2;
	$a[0][1] = [2];
	return $a;
}

/** @param 1 $p */
function redundantPhpDocOnly(int $p): int
{
	$p = 1;
	return $p;
}

function redundantAfterBranches(): int
{
	if (cond()) {
		$x = 1;
	} else {
		$x = 1;
	}
	$x = 1;
	return $x;
}

function redundantUnused(): void
{
	$x = 1;
	$x = 1;
}

function redundantInClosure(): \Closure
{
	return function (): int {
		$x = 'a';
		$x = 'a';
		return $x;
	};
}

class Holder
{
	public function method(): int
	{
		$x = 2;
		$x = 2;
		return $x;
	}
}

function nestedReferenceArray(): array
{
	$x = 1;
	$a = [[&$x]];
	$a[0] = 1;
	return $a;
}

function unsetObject(): void
{
	$o = new WithDestructor();
	unset($o);
}

function unsetScalar(): void
{
	$s = 1;
	unset($s);
}

function nullsafeProperty(?\stdClass $o): void
{
	$p = 'a';
	$o?->{$p = 'b'};
	echo $p;
}

function nullsafeChainedMethod(?\SplStack $s): void
{
	$m = 'count';
	$s?->{$m = 'isEmpty'}();
	echo $m;
}

function freeBlock(): void
{
	$a = 1;
	{
		$a = 2;
	}
	echo $a;
}

function nestedBlockInIf(): void
{
	$a = 1;
	if (cond()) {
		{
			$a = 2;
		}
	}
	echo $a;
}

function whileBody(): void
{
	$a = 1;
	while (cond()) {
		$a = 2;
	}
	echo $a;
}

function doWhileBody(): void
{
	$a = 1;
	do {
		$a = 2;
	} while (cond());
	echo $a;
}

function throwableAfterCaught(): void
{
	$name = 'n';
	try {
		mayThrow();
	} catch (\DomainException $e) {
		echo $e->getMessage();
	} catch (\Throwable $e) {
		echo $name;
		throw $e;
	}
}

function throwableAfterCaughtThrow(): void
{
	$name = 'n';
	try {
		throw new \DomainException();
	} catch (\DomainException $e) {
		echo $e->getMessage();
	} catch (\Throwable $e) {
		echo $name;
	}
}

function undefinedOffsetWrite(): void
{
	$a['x']['y'] = 1;
}

function onlyAForeach(): void
{
	foreach ([1] as $v) {
	}
}
