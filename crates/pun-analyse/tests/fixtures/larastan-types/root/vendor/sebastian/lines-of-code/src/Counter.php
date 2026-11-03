<?php declare(strict_types=1);
/*
 * This file is part of sebastian/lines-of-code.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\LinesOfCode;

use function assert;
use function file_get_contents;
use function substr_count;
use PhpParser\Error;
use PhpParser\Node;
use PhpParser\NodeTraverser;
use PhpParser\ParserFactory;

final class Counter
{
    /**
     * @throws RuntimeException
     */
    public function countInSourceFile(string $sourceFile): LinesOfCode
    {}

    /**
     * @throws RuntimeException
     */
    public function countInSourceString(string $source): LinesOfCode
    {}

    /**
     * @param positive-int $linesOfCode
     * @param Node[]       $nodes
     *
     * @throws RuntimeException
     */
    public function countInAbstractSyntaxTree(int $linesOfCode, array $nodes): LinesOfCode
    {}
}
