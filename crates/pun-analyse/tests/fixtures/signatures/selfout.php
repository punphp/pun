<?php

namespace SignatureSelfOut;

use SignatureSelfOut\Sub\Imported;
use SignatureSelfOut\Sub\Missing as Alias;

trait SomeTrait {}

/** @template T */
class Foo
{
	/** @phpstan-self-out Foo<int>|Nonexistent */
	public function generic(): void {}

	/** @phpstan-self-out ?SomeTrait */
	public function nullable(): void {}

	/** @phpstan-self-out Imported&Alias */
	public function imported(): void {}

	/** @phpstan-self-out \Exception|\SignatureSelfOut\Gone|array<int, Gone2> */
	public function qualified(): void {}

	/**
	 * @psalm-self-out Psalm
	 * @phpstan-this-out Stan
	 */
	public function priority(): void {}

	/** @phpstan-self-out self|static|T|int */
	public function keywords(): void {}

	/**
	 * @template U
	 * @param U $u
	 * @phpstan-self-out U
	 */
	public function template($u): void {}

	/** @phpstan-self-out array{a: InShape}|object{b: InObject}|(Foo is Foo ? InIf : InElse) */
	public function shapes(): void {}

	/** @phpstan-self-out ($p is int ? CondIf : CondElse) */
	public function conditional(int $p): void {}

	/** @phpstan-self-out $this|1|Gone3 */
	public function constants(): void {}

	/** @phpstan-self-out callable(Shapeless): Returned */
	public function callable(): void {}
}

/** @phpstan-self-out Nonexistent */
function notAMethod(): void {}
