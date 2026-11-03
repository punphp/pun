<?php declare(strict_types=1);

namespace PhpParser;

use PhpParser\Node\Name;
use PhpParser\Node\Name\FullyQualified;
use PhpParser\Node\Stmt;

class NameContext {
    /** @var null|Name Current namespace */
    protected ?Name $namespace;

    /** @var Name[][] Map of format [aliasType => [aliasName => originalName]] */
    protected array $aliases = [];

    /** @var Name[][] Same as $aliases but preserving original case */
    protected array $origAliases = [];

    /** @var ErrorHandler Error handler */
    protected ErrorHandler $errorHandler;

    /**
     * Create a name context.
     *
     * @param ErrorHandler $errorHandler Error handling used to report errors
     */
    public function __construct(ErrorHandler $errorHandler) {}

    /**
     * Start a new namespace.
     *
     * This also resets the alias table.
     *
     * @param Name|null $namespace Null is the global namespace
     */
    public function startNamespace(?Name $namespace = null): void {}

    /**
     * Add an alias / import.
     *
     * @param Name $name Original name
     * @param string $aliasName Aliased name
     * @param Stmt\Use_::TYPE_* $type One of Stmt\Use_::TYPE_*
     * @param array<string, mixed> $errorAttrs Attributes to use to report an error
     */
    public function addAlias(Name $name, string $aliasName, int $type, array $errorAttrs = []): void {}

    /**
     * Get current namespace.
     *
     * @return null|Name Namespace (or null if global namespace)
     */
    public function getNamespace(): ?Name {}

    /**
     * Get resolved name.
     *
     * @param Name $name Name to resolve
     * @param Stmt\Use_::TYPE_* $type One of Stmt\Use_::TYPE_{FUNCTION|CONSTANT}
     *
     * @return null|Name Resolved name, or null if static resolution is not possible
     */
    public function getResolvedName(Name $name, int $type): ?Name {}

    /**
     * Get resolved class name.
     *
     * @param Name $name Class ame to resolve
     *
     * @return Name Resolved name
     */
    public function getResolvedClassName(Name $name): Name {}

    /**
     * Get possible ways of writing a fully qualified name (e.g., by making use of aliases).
     *
     * @param string $name Fully-qualified name (without leading namespace separator)
     * @param Stmt\Use_::TYPE_* $type One of Stmt\Use_::TYPE_*
     *
     * @return Name[] Possible representations of the name
     */
    public function getPossibleNames(string $name, int $type): array {}

    /**
     * Get shortest representation of this fully-qualified name.
     *
     * @param string $name Fully-qualified name (without leading namespace separator)
     * @param Stmt\Use_::TYPE_* $type One of Stmt\Use_::TYPE_*
     *
     * @return Name Shortest representation
     */
    public function getShortName(string $name, int $type): Name {}

    private function resolveAlias(Name $name, int $type): ?FullyQualified {}

    private function getNamespaceRelativeName(string $name, string $lcName, int $type): ?Name {}

    private function normalizeConstName(string $name): string {}
}
