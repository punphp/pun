<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\NodeAbstract;

/**
 * Represents a non-namespaced name. Namespaced names are represented using Name nodes.
 */
class Identifier extends NodeAbstract {
    /**
     * @psalm-var non-empty-string
     * @var string Identifier as string
     */
    public string $name;

    /** @var array<string, bool> */
    private static array $specialClassNames = [
        'self'   => true,
        'parent' => true,
        'static' => true,
    ];

    /**
     * Constructs an identifier node.
     *
     * @param string $name Identifier as string
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(string $name, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    /**
     * Get identifier as string.
     *
     * @psalm-return non-empty-string
     * @return string Identifier as string.
     */
    public function toString(): string {}

    /**
     * Get lowercased identifier as string.
     *
     * @psalm-return non-empty-string&lowercase-string
     * @return string Lowercased identifier as string
     */
    public function toLowerString(): string {}

    /**
     * Checks whether the identifier is a special class name (self, parent or static).
     *
     * @return bool Whether identifier is a special class name
     */
    public function isSpecialClassName(): bool {}

    /**
     * Get identifier as string.
     *
     * @psalm-return non-empty-string
     * @return string Identifier as string
     */
    public function __toString(): string {}

    public function getType(): string {}
}
