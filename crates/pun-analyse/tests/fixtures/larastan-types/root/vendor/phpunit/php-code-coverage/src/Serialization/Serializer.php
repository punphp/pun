<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Serialization;

use const PHP_EOL;
use function preg_replace_callback;
use function serialize;
use DateTimeImmutable;
use SebastianBergmann\CodeCoverage\CodeCoverage;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Util\PathReducer;
use SebastianBergmann\CodeCoverage\Util\Phar;
use SebastianBergmann\CodeCoverage\Version;
use SebastianBergmann\Environment\Runtime;
use SebastianBergmann\GitState\Builder as GitStateBuilder;

/**
 * @phpstan-type SerializedCoverage array{
 *     buildInformation: array{
 *         timestamp: string,
 *         runtime: array{
 *             name: string,
 *             version: string,
 *             vendorUrl: string,
 *         },
 *         phpCodeCoverage: array{
 *             version: string,
 *             serializationFormat: int,
 *             driverInformation: array{
 *                 name: non-empty-string,
 *                 version: non-empty-string,
 *             },
 *         },
 *         git?: array{
 *             originUrl: string,
 *             branch: string,
 *             commit: string,
 *             isClean: bool,
 *             status: string,
 *         },
 *     },
 *     basePath: string,
 *     codeCoverage: ProcessedCodeCoverageData,
 *     testResults: array<non-empty-string, array{size: string, status: string, time: float}>,
 * }
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Serializer
{
    public const int SERIALIZATION_FORMAT = 4;

    /**
     * @param non-empty-string $target
     *
     * @throws PharPrefixCouldNotBeStrippedException
     */
    public function serialize(string $target, CodeCoverage $codeCoverage, bool $includeGitInformation = false): void
    {}

    /**
     * @throws PharPrefixCouldNotBeStrippedException
     *
     * @return non-empty-string
     */
    private function stripPharPrefix(string $serialized): string
    {}
}
