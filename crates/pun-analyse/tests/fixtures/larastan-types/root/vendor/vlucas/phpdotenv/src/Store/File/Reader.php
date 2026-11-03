<?php

declare(strict_types=1);

namespace Dotenv\Store\File;

use Dotenv\Exception\InvalidEncodingException;
use Dotenv\Util\Str;
use PhpOption\Option;

/**
 * @internal
 */
final class Reader
{
    /**
     * This class is a singleton.
     *
     * @codeCoverageIgnore
     *
     * @return void
     */
    private function __construct()
    {}

    /**
     * Read the file(s), and return their raw content.
     *
     * We provide the file path as the key, and its content as the value. If
     * short circuit mode is enabled, then the returned array with have length
     * at most one. File paths that couldn't be read are omitted entirely.
     *
     * @param string[]    $filePaths
     * @param bool        $shortCircuit
     * @param string|null $fileEncoding
     *
     * @throws \Dotenv\Exception\InvalidEncodingException
     *
     * @return array<string, string>
     */
    public static function read(array $filePaths, bool $shortCircuit = true, ?string $fileEncoding = null)
    {}

    /**
     * Read the given file.
     *
     * @param string      $path
     * @param string|null $encoding
     *
     * @throws \Dotenv\Exception\InvalidEncodingException
     *
     * @return \PhpOption\Option<string>
     */
    private static function readFromFile(string $path, ?string $encoding = null)
    {}
}
