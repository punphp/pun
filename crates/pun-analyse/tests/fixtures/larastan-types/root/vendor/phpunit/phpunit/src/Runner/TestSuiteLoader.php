<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner;

use function array_slice;
use function basename;
use function count;
use function get_declared_classes;
use function realpath;
use function str_ends_with;
use function strpos;
use function strtolower;
use function substr;
use PHPUnit\Framework\TestCase;
use ReflectionClass;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestSuiteLoader
{
    /**
     * @var ?non-negative-int
     */
    private static ?int $numberOfDeclaredClasses = null;

    /**
     * @var array<non-empty-string, list<class-string>>
     */
    private static array $fileToClassesMap = [];

    /**
     * @throws Exception
     *
     * @return ReflectionClass<TestCase>
     */
    public function load(string $suiteClassFile): ReflectionClass
    {}

    private function classNameFromFileName(string $suiteClassFile): string
    {}

    /**
     * @return array<class-string>
     */
    private function loadSuiteClassFile(string $suiteClassFile): array
    {}

    /**
     * @param array<class-string> $classes
     */
    private static function mapClassesToTheFilesThatDeclareThem(array $classes): void
    {}
}
