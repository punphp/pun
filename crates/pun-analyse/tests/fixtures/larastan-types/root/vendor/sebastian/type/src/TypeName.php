<?php declare(strict_types=1);
/*
 * This file is part of sebastian/type.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Type;

use function assert;
use function strrpos;
use function substr;
use ReflectionClass;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final readonly class TypeName
{
    private ?string $namespaceName;

    /**
     * @var non-empty-string
     */
    private string $simpleName;

    /**
     * @var non-empty-string
     */
    private string $qualifiedName;

    /**
     * @param class-string $fullClassName
     */
    public static function fromQualifiedName(string $fullClassName): self
    {}

    /**
     * @param ReflectionClass<*> $type
     */
    public static function fromReflection(ReflectionClass $type): self
    {}

    /**
     * @param non-empty-string $simpleName
     */
    public function __construct(?string $namespaceName, string $simpleName)
    {}

    public function namespaceName(): ?string
    {}

    /**
     * @return non-empty-string
     */
    public function simpleName(): string
    {}

    /**
     * @return non-empty-string
     */
    public function qualifiedName(): string
    {}

    public function isNamespaced(): bool
    {}
}
