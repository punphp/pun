<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use InvalidArgumentException;
use Psr\Http\Message\UploadedFileInterface;

/**
 * @internal
 *
 * @phpstan-type UploadedFileTree array<array-key, UploadedFileInterface|array>
 */
final class UploadedFileNormalizer
{
    private function __construct()
    {}

    /**
     * Return an UploadedFile instance array.
     *
     * @param array $files An array which respect $_FILES structure
     *
     * @return UploadedFileTree
     *
     * @throws InvalidArgumentException for unrecognized values
     */
    public static function normalize(array $files): array
    {}

    /**
     * Create and return an UploadedFile instance from a $_FILES specification.
     *
     * If the specification represents an array of values, this method will
     * delegate to normalizeNestedFileSpec() and return that return value.
     *
     * @param array $value $_FILES struct
     *
     * @return UploadedFileInterface|UploadedFileTree
     */
    private static function createUploadedFileFromSpec(array $value)
    {}

    private static function assertFileSpec(array $value): void
    {}

    /**
     * Normalize an array of file specifications.
     *
     * Loops through all nested files and returns a normalized array of
     * UploadedFileInterface instances.
     *
     * @return UploadedFileTree
     */
    private static function normalizeNestedFileSpec(array $files = []): array
    {}

    private static function assertNestedFileSpec(array $files): void
    {}
}
