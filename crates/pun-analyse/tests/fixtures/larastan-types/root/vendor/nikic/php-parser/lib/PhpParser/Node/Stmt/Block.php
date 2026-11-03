<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node\Stmt;

class Block extends Stmt {
    /** @var Stmt[] Statements */
    public array $stmts;

    /**
     * A block of statements.
     *
     * @param Stmt[] $stmts Statements
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(array $stmts, array $attributes = []) {}

    public function getType(): string {}

    public function getSubNodeNames(): array {}
}
