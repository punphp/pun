<?php declare(strict_types=1);

namespace PhpParser;

/**
 * A PHP token. On PHP 8.0 this extends from PhpToken.
 */
class Token extends Internal\TokenPolyfill {
    /** Get (exclusive) zero-based end position of the token. */
    public function getEndPos(): int {}

    /** Get 1-based end line number of the token. */
    public function getEndLine(): int {}
}
