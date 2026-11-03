<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node\Stmt;

class InlineHTML extends Stmt {
    /** @var string String */
    public string $value;

    /**
     * Constructs an inline HTML node.
     *
     * @param string $value String
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(string $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
