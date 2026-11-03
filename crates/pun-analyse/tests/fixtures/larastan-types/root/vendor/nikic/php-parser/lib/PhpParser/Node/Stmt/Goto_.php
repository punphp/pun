<?php declare(strict_types=1);

namespace PhpParser\Node\Stmt;

use PhpParser\Node\Identifier;
use PhpParser\Node\Stmt;

class Goto_ extends Stmt {
    /** @var Identifier Name of label to jump to */
    public Identifier $name;

    /**
     * Constructs a goto node.
     *
     * @param string|Identifier $name Name of label to jump to
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct($name, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    public function getType(): string {}
}
