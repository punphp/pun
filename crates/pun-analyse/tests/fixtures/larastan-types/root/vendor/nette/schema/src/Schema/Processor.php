<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema;

use Nette;


/**
 * Schema validator.
 */
final class Processor
{
	/** @var list<\Closure(Context): void> */
	public array $onNewContext = [];
	private Context $context;
	private bool $skipDefaults = false;


	/**
	 * When enabled, missing properties are not filled with their default values.
	 */
	public function skipDefaults(bool $value = true): void
	{}


	/**
	 * Normalizes and validates data. Result is a clean completed data.
	 * @throws ValidationException
	 */
	public function process(Schema $schema, mixed $data): mixed
	{}


	/**
	 * Normalizes and validates and merges multiple data. Result is a clean completed data.
	 * @param  list<mixed>  $dataset
	 * @throws ValidationException
	 */
	public function processMultiple(Schema $schema, array $dataset): mixed
	{}


	/**
	 * Returns all deprecation warnings collected during the last processing run.
	 * @return list<string>
	 */
	public function getWarnings(): array
	{}


	private function throwErrors(Context $context): void
	{}


	private function createContext(): Context
	{}
}
