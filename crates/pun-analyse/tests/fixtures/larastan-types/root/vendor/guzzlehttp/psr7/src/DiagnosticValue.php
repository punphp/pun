<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * Escapes control characters and malformed UTF-8 for use in diagnostics.
 */
final class DiagnosticValue
{
    private function __construct()
    {}

    /**
     * Escapes C0, DEL, and C1 controls as uppercase `\xNN` sequences.
     *
     * ASCII bytes from 0x20 through 0x7E and valid UTF-8 characters outside
     * those control ranges remain unchanged. If the input is malformed UTF-8 or
     * PCRE cannot process it, every byte outside printable ASCII is escaped.
     * Valid C1 characters are rendered as `\xNN` using their Unicode code
     * points. During bytewise fallback, each original byte outside printable
     * ASCII is rendered in the same form. The result is diagnostic text, not a
     * reversible encoding.
     *
     * This does not encode values for HTML, JSON, shells, terminals, URLs, or
     * protocol fields.
     */
    public static function escape(string $value): string
    {}

    private static function escapeBytes(string $value): string
    {}
}
