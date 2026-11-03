<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Test\Target;

use function array_keys;
use function array_merge;
use function array_unique;
use function array_values;
use function realpath;
use function strcasecmp;

/**
 * @phpstan-type TargetMap array{namespaces: TargetMapPart, traits: TargetMapPart, classes: TargetMapPart, classesThatExtendClass: TargetMapPart, classesThatImplementInterface: TargetMapPart, methods: TargetMapPart, functions: TargetMapPart, files: TargetMapPart, directories: TargetMapPart, directoriesRecursively: TargetMapPart, reverseLookup: ReverseLookup}
 * @phpstan-type TargetMapPart array<non-empty-string, array<non-empty-string, list<positive-int>>>
 * @phpstan-type ReverseLookup array<non-empty-string, non-empty-string>
 *
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Mapper
{
    /**
     * @var TargetMap
     */
    private array $map;

    /**
     * @param TargetMap $map
     */
    public function __construct(array $map)
    {}

    /**
     * @return array<non-empty-string, list<positive-int>>
     */
    public function mapTargets(TargetCollection $targets): array
    {}

    /**
     * @throws InvalidCodeCoverageTargetException
     *
     * @return array<non-empty-string, list<positive-int>>
     */
    public function mapTarget(Target $target): array
    {}

    /**
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @return non-empty-string
     */
    public function lookup(string $file, int $line): string
    {}
}
