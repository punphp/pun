<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema\Elements;

use Nette;
use Nette\Schema\{Context, DynamicParameter, Helpers, Kind, MergeMode, Message, Schema, TypeExpression};
use Nette\Utils\Validators;
use function array_key_exists, is_array, is_int, is_string;
use const INF;


class Type implements Schema
{
	use Base;

	private ?Schema $itemsValue = null;
	private ?Schema $itemsKey = null;

	/** @var array{int|float|null, int|float|null} */
	private array $range = [null, null];
	private ?string $pattern = null;
	private bool $merge = true;
	private ?MergeMode $mergeMode = null;


	public function __construct(
		private string $type,
	) {}


	/**
	 * Allows the value to be null in addition to the declared type.
	 */
	public function nullable(): static
	{}


	/**
	 * Controls whether the default value is merged with the input array (enabled by default).
	 */
	public function mergeDefaults(bool $state = true): static
	{}


	/** @internal */
	protected function mergeMode(MergeMode $mode): static
	{}


	/**
	 * Allows the value to be a DynamicParameter, which is recorded for deferred validation.
	 */
	public function dynamic(): static
	{}


	public function min(int|float|null $min): static
	{}


	public function max(int|float|null $max): static
	{}


	/**
	 * @internal  use arrayOf() or listOf()
	 */
	public function items(string|Schema $valueType = 'mixed', string|Schema|null $keyType = null): static
	{}


	/**
	 * Sets a regex pattern the string value must match entirely (anchored to start and end).
	 */
	public function pattern(?string $pattern): static
	{}


	/**
	 * A union of kinds ('int|string') becomes anyOf() in the next major and takes no options of its own;
	 * they belong to the variants: anyOf(Expect::int()->min(1), Expect::string()->min(1)).
	 */
	private function unionDeprecated(string $option): void
	{}


	/********************* inspection ****************d*g**/


	/**
	 * Reports what the type accepts as a plain array (see TypeExpression::parse()); min(), max(), pattern()
	 * and items() narrow every variant they apply to, child schemas are reported as they are.
	 * @return array<string, mixed>
	 * @internal
	 */
	public function describe(): array
	{}


	/**
	 * The parsed expression; 'kind', 'nullable', 'dynamic' and the keys of the kind.
	 * @return array<string, mixed>
	 */
	protected function getParsed(): array
	{}


	/**
	 * Puts min(), max(), pattern() and items() into every variant of the parsed expression that has the key.
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private function narrow(array $item): array
	{}


	/********************* processing ****************d*g**/


	public function normalize(mixed $value, Context $context): mixed
	{}


	/**
	 * @param  array<mixed>  $value
	 * @return array<mixed>
	 * @internal
	 */
	protected function normalizeItems(array $value, Context $context): array
	{}


	public function merge(mixed $value, mixed $base): mixed
	{}


	/**
	 * Without a mode, an array without described items merges deeply.
	 * @internal
	 */
	protected function mergeValues(mixed $value, mixed $base): mixed
	{}


	public function complete(mixed $value, Context $context): mixed
	{}


	/**
	 * @param  array<mixed>  $value
	 * @internal
	 */
	protected function validateItems(array &$value, Context $context): void
	{}
}
