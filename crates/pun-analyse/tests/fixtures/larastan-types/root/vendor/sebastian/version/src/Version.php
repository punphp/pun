<?php declare(strict_types=1);
/*
 * This file is part of sebastian/version.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann;

use const DIRECTORY_SEPARATOR;
use function array_slice;
use function assert;
use function end;
use function explode;
use function fclose;
use function implode;
use function is_dir;
use function is_resource;
use function proc_close;
use function proc_open;
use function stream_get_contents;
use function substr_count;
use function trim;

final readonly class Version
{
    /**
     * @var non-empty-string
     */
    private string $version;

    /**
     * @param non-empty-string $release
     * @param non-empty-string $path
     */
    public function __construct(string $release, string $path)
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}

    /**
     * @param non-empty-string $release
     * @param non-empty-string $path
     *
     * @return non-empty-string
     */
    private function generate(string $release, string $path): string
    {}

    /**
     * @param non-empty-string $path
     * @param non-empty-string $release
     *
     * @return false|non-empty-string
     */
    private function getGitInformation(string $path, string $release): false|string
    {}

    /**
     * @param non-empty-string $path
     * @param list<string>     $arguments
     *
     * @return false|non-empty-string
     */
    private function describe(string $path, array $arguments): false|string
    {}
}
