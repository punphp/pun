<?php declare(strict_types = 1);

namespace PHPStan\PhpDocParser\Ast\Type;

use PHPStan\PhpDocParser\Ast\NodeAttributes;

class IdentifierTypeNode implements TypeNode
{

	use NodeAttributes;

	public string $name;

	public function __construct(string $name)
	{}

	public function __toString(): string
	{}

	/**
	 * @param array<string, mixed> $properties
	 */
	public static function __set_state(array $properties): self
	{}

}
