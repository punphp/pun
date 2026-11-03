<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\TestDox;

use const PHP_EOL;
use function array_key_exists;
use function array_keys;
use function array_map;
use function array_pop;
use function array_splice;
use function array_values;
use function assert;
use function class_exists;
use function explode;
use function gettype;
use function implode;
use function is_bool;
use function is_float;
use function is_int;
use function is_object;
use function is_scalar;
use function is_string;
use function method_exists;
use function preg_quote;
use function preg_replace;
use function preg_replace_callback_array;
use function rtrim;
use function sprintf;
use function str_contains;
use function str_ends_with;
use function str_replace;
use function str_starts_with;
use function strlen;
use function strtolower;
use function substr;
use function trim;
use function ucfirst;
use BackedEnum;
use PHPUnit\Event\Code\TestMethodBuilder;
use PHPUnit\Event\Emitter;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Metadata\TestDox;
use PHPUnit\Metadata\TestDoxFormatter;
use PHPUnit\Util\Color;
use PHPUnit\Util\Exporter;
use PHPUnit\Util\Filter;
use PHPUnit\Util\Sanitizer;
use ReflectionMethod;
use Stringable;
use Throwable;
use UnitEnum;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class NamePrettifier
{
    /**
     * @var array<string, int>
     */
    private array $strings = [];

    /**
     * @var array<non-empty-string, string>
     */
    private array $prettifiedTestCases = [];

    /**
     * @var array<non-empty-string, true>
     */
    private array $erroredFormatters = [];
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param class-string $className
     */
    public function prettifyTestClassName(string $className): string
    {}

    // NOTE: this method is on a hot path and very performance sensitive. change with care.
    public function prettifyTestMethodName(string $name): string
    {}

    public function prettifyTestCase(TestCase $test, bool $colorize): string
    {}

    public function prettifyDataSet(TestCase $test, bool $colorize): string
    {}

    private function prettifyRepetition(TestCase $test, bool $colorize): string
    {}

    /**
     * @return array<non-empty-string, string>
     */
    private function mapTestMethodParameterNamesToProvidedDataValues(TestCase $test, bool $colorize): array
    {}

    private function objectToString(object $value): string
    {}

    /**
     * @return array{0: string, 1: bool}
     */
    private function processTestDox(TestCase $test, TestDox $testDox, bool $colorize): array
    {}

    /**
     * @return array{0: string, 1: bool}
     */
    private function processTestDoxFormatter(TestCase $test, TestDoxFormatter $formatter): array
    {}

    /**
     * @param array{string, string} $matches
     */
    private static function separateUppercaseLetter(array $matches): string
    {}

    /**
     * @param array{string, string} $matches
     */
    private static function separateNumber(array $matches): string
    {}
}
