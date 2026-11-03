<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node\Identifier;
use PhpParser\Node\Stmt;

class Label extends Stmt {
    /** @var Identifier Name */
    public Identifier $name;

    /**
     * Constructs a label node.
     *
     * @param string|Identifier $name Name
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
