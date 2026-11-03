<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Stream that when read returns bytes for a streaming multipart or
 * multipart/form-data stream.
 */
final class MultipartStream implements StreamInterface
{
    use StreamDecoratorTrait;
    use NonSerializableStreamTrait;

    private string $boundary;

    private StreamInterface $stream;

    private const BOUNDARY_CHARS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'()+_,-./:=? ";

    /**
     * @param array       $elements Array of associative arrays, each containing a
     *                              required "name" key mapping to the form field,
     *                              name, a required "contents" key mapping to any
     *                              non-array value accepted by Utils::streamFor()
     *                              (non-string scalar field values are cast to
     *                              string), or an array for nested expansion.
     *                              Optional keys include "headers" (associative
     *                              array of custom headers) and "filename" (string
     *                              to send as the filename in the part).
     *                              When "contents" is an array, it is recursively
     *                              expanded into multiple fields using bracket notation
     *                              (e.g., name[0][key]). Empty arrays produce no fields.
     *                              The "filename" and "headers" options cannot be used
     *                              with array contents.
     * @param string|null $boundary You can optionally provide a specific boundary
     *
     * @throws \InvalidArgumentException
     */
    public function __construct(array $elements = [], ?string $boundary = null)
    {}

    public function getBoundary(): string
    {}

    public function isWritable(): bool
    {}

    /**
     * Get the headers needed before transferring the content of a POST file
     *
     * @param array<array-key, string> $headers
     */
    private function getHeaders(array $headers): string
    {}

    /**
     * Create the aggregate stream that will be used to upload the POST data
     */
    protected function createStream(array $elements = []): StreamInterface
    {}

    private function addElement(AppendStream $stream, array $element): void
    {}

    /**
     * Recursively expand array contents into multiple form fields.
     *
     * @param array<array-key, mixed> $contents
     */
    private function addNestedElements(AppendStream $stream, array $contents, string $root): void
    {}

    /**
     * @param array<array-key, mixed> $headers
     *
     * @return array{0: StreamInterface, 1: array<array-key, string>}
     */
    private function createElement(string $name, StreamInterface $stream, ?string $filename, array $headers): array
    {}

    /**
     * @param array<array-key, string> $headers
     */
    private static function getHeader(array $headers, string $key): ?string
    {}

    private static function validateBoundary(string $boundary): void
    {}

    /**
     * @param array<array-key, mixed> $headers
     *
     * @return array<array-key, string>
     */
    private static function normalizePartHeaders(array $headers): array
    {}

    private static function validatePartHeaderName(string $name): void
    {}

    private static function validatePartHeaderValue(string $name, string $value): void
    {}

    private static function escapeContentDispositionParameter(string $value): string
    {}
}
