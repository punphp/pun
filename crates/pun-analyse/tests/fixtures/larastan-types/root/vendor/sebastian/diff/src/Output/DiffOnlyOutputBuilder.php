<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff\Output;

use function assert;
use function fclose;
use function fopen;
use function fwrite;
use function is_resource;
use function str_ends_with;
use function stream_get_contents;
use function substr;
use SebastianBergmann\Diff\Differ;

/**
 * Builds a diff string representation in a loose unified diff format
 * listing only changes lines. Does not include line numbers.
 */
final class DiffOnlyOutputBuilder implements DiffOutputBuilderInterface
{
    private string $header;

    public function __construct(string $header = "--- Original\n+++ New\n")
    {}

    /**
     * @param list<array{0: mixed, 1: int}> $diff
     */
    public function getDiff(array $diff): string
    {}
}
