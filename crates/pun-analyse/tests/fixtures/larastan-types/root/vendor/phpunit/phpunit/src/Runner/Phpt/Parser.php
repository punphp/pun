<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Phpt;

use const DIRECTORY_SEPARATOR;
use function assert;
use function count;
use function dirname;
use function explode;
use function file;
use function file_get_contents;
use function implode;
use function in_array;
use function is_array;
use function is_file;
use function is_readable;
use function is_string;
use function preg_match;
use function realpath;
use function rtrim;
use function sprintf;
use function str_contains;
use function trim;
use PHPUnit\Runner\Exception;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @see https://qa.php.net/phpt_details.php
 */
final readonly class Parser
{
    private const array SUPPORTED_SECTIONS = [
        'TEST',
        'FILE',
        'FILEEOF',
        'FILE_EXTERNAL',
        'EXPECT',
        'EXPECT_EXTERNAL',
        'EXPECTF',
        'EXPECTF_EXTERNAL',
        'EXPECTREGEX',
        'EXPECTREGEX_EXTERNAL',
        'INI',
        'ENV',
        'SKIPIF',
        'XFAIL',
        'CLEAN',
        'STDIN',
        'ARGS',
    ];
    private const array UNSUPPORTED_SECTIONS = [
        'CGI',
        'COOKIE',
        'DEFLATE_POST',
        'EXPECTHEADERS',
        'EXTENSIONS',
        'GET',
        'GZIP_POST',
        'HEADERS',
        'PHPDBG',
        'POST',
        'POST_RAW',
        'PUT',
        'REDIRECTTEST',
        'REQUEST',
    ];
    private const array IGNORED_SECTIONS = [
        'CAPTURE_STDIO',
        'CONFLICTS',
        'CREDITS',
        'DESCRIPTION',
        'FLAKY',
        'WHITESPACE_SENSITIVE',
        'XLEAK',
    ];
    private const array FILE_SECTIONS = [
        'FILE',
        'FILEEOF',
        'FILE_EXTERNAL',
    ];
    private const array EXPECTATION_SECTIONS = [
        'EXPECT',
        'EXPECT_EXTERNAL',
        'EXPECTF',
        'EXPECTF_EXTERNAL',
        'EXPECTREGEX',
        'EXPECTREGEX_EXTERNAL',
    ];

    /**
     * @param non-empty-string $phptFile
     *
     * @throws Exception
     *
     * @return array<non-empty-string, string>
     */
    public function parse(string $phptFile): array
    {}

    /**
     * @return array<non-empty-string, string>
     */
    public function parseEnvSection(string $content): array
    {}

    /**
     * @param array<string>|string                          $content
     * @param array<non-empty-string, array<string>|string> $ini
     *
     * @return array<non-empty-string, array<string>|string>
     */
    public function parseIniSection(array|string $content, array $ini = []): array
    {}

    /**
     * @param non-empty-string                $phptFile
     * @param array<non-empty-string, string> $sections
     *
     * @throws Exception
     */
    private function parseExternal(string $phptFile, array &$sections): void
    {}

    /**
     * @param array<non-empty-string, string>  $sections
     * @param non-empty-list<non-empty-string> $sectionFamily
     *
     * @throws ConflictingPhptSectionsException
     * @throws InvalidPhptFileException
     *
     * @return non-empty-string
     */
    private function ensureExactlyOneSectionOf(array $sections, array $sectionFamily): string
    {}

    /**
     * @param array<non-empty-string, string> $sections
     * @param non-empty-string                $codeSection
     *
     * @throws InvalidPhptFileException
     */
    private function ensureCodeIsNotEmpty(array $sections, string $codeSection): void
    {}
}
