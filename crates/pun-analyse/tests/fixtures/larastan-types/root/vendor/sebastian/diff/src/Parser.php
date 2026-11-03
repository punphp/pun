<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff;

use const PREG_UNMATCHED_AS_NULL;
use function array_pop;
use function count;
use function max;
use function preg_match;
use function preg_split;

/**
 * Unified diff parser.
 */
final class Parser
{
    private const string LINE_BREAK       = '(\r\n|\r|\n)';
    private const string FROM_FILE_HEADER = '#^---\h+"?(?P<file>[^\\v\\t"]+)#';
    private const string TO_FILE_HEADER   = '#^\\+\\+\\+\\h+"?(?P<file>[^\\v\\t"]+)#';
    private const string METADATA_HEADER  = '/^(?:diff --git |index [\da-f.]+|(?:---|\+\+\+) [ab]\/)/';
    private const string CHUNK_HEADER     = '/^@@\s+-(?P<start>\d+)(?:,\s*(?P<startrange>\d+))?\s+\+(?P<end>\d+)(?:,\s*(?P<endrange>\d+))?\s+@@/';
    private const string CHUNK_LINE       = '/^(?P<type>[+ -])?(?P<line>.*)/';

    /**
     * @return list<Diff>
     */
    public function parse(string $string): array
    {}

    /**
     * @param string[] $lines
     */
    private function parseFileDiff(Diff $diff, array $lines): void
    {}
}
