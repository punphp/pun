<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, Helpers};
use function count, is_string;


/**
 * @internal
 */
trait Base
{
	private bool $required = false;
	private mixed $default = null;

	/** @var ?\Closure(mixed): mixed */
	private ?\Closure $before = null;

	/** @var list<\Closure(mixed, Context): mixed> */
	private array $transforms = [];
	private ?string $deprecated = null;
	private ?string $description = null;

	/** @var ?\Closure(mixed, mixed): mixed */
	private ?\Closure $mergeWith = null;


	public function default(mixed $value): static
	{}


	public function required(bool $state = true): static
	{}


	/**
	 * Sets a pre-normalization callback applied to the raw input value before any validation.
	 * @param  callable(mixed): mixed  $handler
	 */
	public function before(callable $handler): static
	{}


	/**
	 * Sets a custom strategy combining two layers. Must be a pure combiner; canonicalize layer shape in before() instead.
	 * Either side may be null, a layer can legally be null.
	 * @param  callable(mixed, mixed): mixed  $fn
	 */
	public function mergeWith(callable $fn): static
	{}


	/**
	 * Casts the validated value to a built-in type or instantiates the given class; a null stays null for a class or ?type.
	 */
	public function castTo(string $type): static
	{}


	/**
	 * Adds a post-validation transformation callback. The handler may also report errors via Context.
	 * @param  callable(mixed, Context): mixed  $handler
	 */
	public function transform(callable $handler): static
	{}


	/**
	 * Adds a custom validation assertion; optionally describe it for error messages.
	 * @param  callable(mixed): bool  $handler
	 */
	public function assert(callable $handler, ?string $description = null): static
	{}


	/**
	 * Marks the item as deprecated; emits a warning with the given message when the item is used.
	 */
	public function deprecated(string $message = 'The item %path% is deprecated.'): static
	{}


	/**
	 * Sets a human-readable description of the item; it does not affect validation.
	 */
	public function description(string $description): static
	{}


	/**
	 * The metadata every element reports from describe(); the element adds its own keys.
	 * @return array{required: bool, description: ?string}
	 */
	protected function describeBase(): array
	{}


	public function completeDefault(Context $context): mixed
	{}


	public function doNormalize(mixed $value, Context $context): mixed
	{}


	private function doDeprecation(Context $context): void
	{}


	private function doTransform(mixed $value, Context $context): mixed
	{}


	/** @deprecated use Nette\Schema\Helpers::validateType() */
	private function doValidate(mixed $value, string $expected, Context $context): bool
	{}


	/**
	 * @deprecated use Nette\Schema\Helpers::validateRange()
	 * @param  array{int|float|null, int|float|null}  $range
	 */
	private static function doValidateRange(mixed $value, array $range, Context $context, string $types = ''): bool
	{}


	/** @deprecated use doTransform() */
	private function doFinalize(mixed $value, Context $context): mixed
	{}
}
