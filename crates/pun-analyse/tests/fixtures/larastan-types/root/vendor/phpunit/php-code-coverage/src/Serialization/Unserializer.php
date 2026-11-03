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
use function fclose;
use function fgets;
use function fopen;
use function is_array;
use function is_bool;
use function is_int;
use function is_string;
use function preg_match;
use function restore_error_handler;
use function set_error_handler;
use function trim;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;

/**
 * @phpstan-import-type SerializedCoverage from Serializer
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Unserializer
{
    /**
     * @param non-empty-string $path
     *
     * @throws FileCouldNotBeReadException
     * @throws InvalidCoverageDataException
     * @throws VersionMismatchException
     *
     * @return SerializedCoverage
     */
    public function unserialize(string $path): array
    {}

    /**
     * @phpstan-assert SerializedCoverage $data
     *
     * @throws InvalidCoverageDataException
     */
    private function validate(mixed $data): void
    {}
}
