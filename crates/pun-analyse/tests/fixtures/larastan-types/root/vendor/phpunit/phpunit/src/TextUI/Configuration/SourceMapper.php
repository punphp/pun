<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use const DIRECTORY_SEPARATOR;
use function array_values;
use function file_get_contents;
use function file_put_contents;
use function is_array;
use function is_string;
use function preg_match;
use function realpath;
use function serialize;
use function str_replace;
use function unserialize;
use SebastianBergmann\FileIterator\Facade as FileIteratorFacade;
use SplObjectStorage;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class SourceMapper
{
    /**
     * @var ?SplObjectStorage<Source, array<non-empty-string, true>>
     */
    private static ?SplObjectStorage $files = null;

    public static function saveTo(string $path, Source $source): bool
    {}

    /**
     * @codeCoverageIgnore
     */
    public static function loadFrom(string $path, Source $source): void
    {}

    /**
     * @return array<non-empty-string, true>
     */
    public function map(Source $source): array
    {}

    /**
     * @return array<non-empty-string, true>
     */
    public function mapForCodeCoverage(Source $source): array
    {}

    private function isInHiddenDirectory(string $path, false|string $basePath): bool
    {}

    /**
     * @return list<array{path: non-empty-string, prefixes: list<non-empty-string>, suffixes: list<non-empty-string>}>
     */
    private function aggregateDirectories(FilterDirectoryCollection $directories): array
    {}
}
