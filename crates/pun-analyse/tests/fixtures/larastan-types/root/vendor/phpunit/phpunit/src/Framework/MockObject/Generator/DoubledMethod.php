<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject\Generator;

use function array_key_exists;
use function assert;
use function count;
use function explode;
use function implode;
use function is_object;
use function is_string;
use function preg_match;
use function preg_replace;
use function strlen;
use function strpos;
use function strtolower;
use function substr;
use function substr_count;
use function trim;
use function var_export;
use ReflectionMethod;
use ReflectionParameter;
use SebastianBergmann\Type\ReflectionMapper;
use SebastianBergmann\Type\Type;
use SebastianBergmann\Type\UnknownType;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DoubledMethod
{
    use TemplateLoader;

    /**
     * @var class-string
     */
    private readonly string $className;

    /**
     * @var non-empty-string
     */
    private readonly string $methodName;
    private readonly string $modifier;
    private readonly string $argumentsForDeclaration;
    private readonly string $argumentsForCall;
    private readonly Type $returnType;
    private readonly string $reference;
    private readonly bool $static;
    private readonly ?string $deprecation;

    /**
     * @var array<int, mixed>
     */
    private readonly array $defaultParameterValues;

    /**
     * @var non-negative-int
     */
    private readonly int $numberOfParameters;

    /**
     * @throws ReflectionException
     * @throws RuntimeException
     */
    public static function fromReflection(ReflectionMethod $method): self
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function fromName(string $className, string $methodName): self
    {}

    /**
     * @param class-string      $className
     * @param non-empty-string  $methodName
     * @param array<int, mixed> $defaultParameterValues
     * @param non-negative-int  $numberOfParameters
     */
    private function __construct(string $className, string $methodName, string $modifier, string $argumentsForDeclaration, string $argumentsForCall, array $defaultParameterValues, int $numberOfParameters, Type $returnType, string $reference, bool $static, ?string $deprecation)
    {}

    /**
     * @return non-empty-string
     */
    public function methodName(): string
    {}

    /**
     * @throws RuntimeException
     */
    public function generateCode(): string
    {}

    public function returnType(): Type
    {}

    /**
     * @return array<int, mixed>
     */
    public function defaultParameterValues(): array
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfParameters(): int
    {}

    /**
     * @see https://wiki.php.net/rfc/deprecate-return-value-from-construct
     */
    private function mustNotReturnValue(): bool
    {}

    /**
     * Returns the parameters of a function or method.
     *
     * @throws RuntimeException
     */
    private static function methodParametersForDeclaration(ReflectionMethod $method): string
    {}

    /**
     * Returns the parameters of a function or method.
     *
     * @throws ReflectionException
     */
    private static function methodParametersForCall(ReflectionMethod $method): string
    {}

    /**
     * @throws ReflectionException
     */
    private static function exportDefaultValue(ReflectionParameter $parameter): string
    {}

    /**
     * @return array<int, mixed>
     */
    private static function methodParametersDefaultValues(ReflectionMethod $method): array
    {}
}
