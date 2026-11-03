<?php declare(strict_types = 1);

namespace PHPStan\PhpDocParser\Ast\Type;

use PHPStan\PhpDocParser\Ast\NodeAttributes;
use function implode;
use function sprintf;

class GenericTypeNode implements TypeNode
{

	public const VARIANCE_INVARIANT = 'invariant';
	public const VARIANCE_COVARIANT = 'covariant';
	public const VARIANCE_CONTRAVARIANT = 'contravariant';
	public const VARIANCE_BIVARIANT = 'bivariant';

	use NodeAttributes;

	/** @var IdentifierTypeNode|NameNode */
	public TypeNode $type;

	/** @var TypeNode[] */
	public array $genericTypes;

	/** @var (self::VARIANCE_*)[] */
	public array $variances;

	/**
	 * @param IdentifierTypeNode|NameNode $type
	 * @param TypeNode[] $genericTypes
	 * @param (self::VARIANCE_*)[] $variances
	 */
	public function __construct(TypeNode $type, array $genericTypes, array $variances = [])
	{}

	public function __toString(): string
	{}

	/**
	 * @param array<string, mixed> $properties
	 */
	public static function __set_state(array $properties): self
	{}

}
