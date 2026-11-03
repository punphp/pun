<?php declare(strict_types=1);

namespace PhpParser\NodeVisitor;

use PhpParser\Comment;
use PhpParser\Node;
use PhpParser\NodeVisitorAbstract;
use PhpParser\Token;

class CommentAnnotatingVisitor extends NodeVisitorAbstract {
    /** @var int Last seen token start position */
    private int $pos = 0;
    /** @var Token[] Token array */
    private array $tokens;
    /** @var list<int> Token positions of comments */
    private array $commentPositions = [];

    /**
     * Create a comment annotation visitor.
     *
     * @param Token[] $tokens Token array
     */
    public function __construct(array $tokens) {}

    public function enterNode(Node $node) {}
}
