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

use function array_key_exists;
use function array_merge;
use function assert;
use DateTimeImmutable;
use SebastianBergmann\CodeCoverage\Version;

/**
 * @phpstan-import-type SerializedCoverage from Serializer
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Merger
{
    /**
     * @param iterable<non-empty-string> $paths
     *
     * @throws DriverMismatchException
     * @throws EmptyPathListException
     * @throws GitInformationMismatchException
     * @throws MixedGitInformationException
     * @throws RuntimeMismatchException
     *
     * @return SerializedCoverage
     */
    public function merge(iterable $paths, bool $requireMatchingGitInformation = true, bool $requireMatchingPhpVersion = true, bool $requireMatchingCodeCoverageDriver = true): array
    {}

    /**
     * @param SerializedCoverage $reference
     * @param SerializedCoverage $current
     *
     * @throws RuntimeMismatchException
     */
    private function assertPhpVersionMatches(array $reference, array $current): void
    {}

    /**
     * @param SerializedCoverage $reference
     * @param SerializedCoverage $current
     *
     * @throws DriverMismatchException
     */
    private function assertDriverMatches(array $reference, array $current): void
    {}

    /**
     * @param SerializedCoverage $reference
     * @param SerializedCoverage $current
     *
     * @throws GitInformationMismatchException
     * @throws MixedGitInformationException
     */
    private function assertGitInformationMatches(array $reference, array $current): void
    {}
}
