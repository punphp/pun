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

use function array_merge;
use function array_unique;
use function assert;
use function count;
use PhpParser\Comment;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\NodeVisitorAbstract;

final class LineCountingVisitor extends NodeVisitorAbstract
{
    /**
     * @var positive-int
     */
    private readonly int $linesOfCode;

    /**
     * @var Comment[]
     */
    private array $comments = [];

    /**
     * @var int[]
     */
    private array $linesWithStatements = [];

    /**
     * @param positive-int $linesOfCode
     */
    public function __construct(int $linesOfCode)
    {}

    public function enterNode(Node $node): null
    {}

    public function result(): LinesOfCode
    {}

    /**
     * @return Comment[]
     */
    private function comments(): array
    {}
}
