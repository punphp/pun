<?php declare(strict_types=1);

namespace PhpParser\Builder;

use PhpParser\Builder;
use PhpParser\BuilderHelpers;
use PhpParser\Modifiers;
use PhpParser\Node;
use PhpParser\Node\Stmt;

class TraitUseAdaptation implements Builder {
    private const TYPE_UNDEFINED  = 0;
    private const TYPE_ALIAS      = 1;
    private const TYPE_PRECEDENCE = 2;

    protected int $type;
    protected ?Node\Name $trait;
    protected Node\Identifier $method;
    protected ?int $modifier = null;
    protected ?Node\Identifier $alias = null;
    /** @var Node\Name[] */
    protected array $insteadof = [];

    /**
     * Creates a trait use adaptation builder.
     *
     * @param Node\Name|string|null $trait Name of adapted trait
     * @param Node\Identifier|string $method Name of adapted method
     */
    public function __construct($trait, $method) {}

    /**
     * Sets alias of method.
     *
     * @param Node\Identifier|string $alias Alias for adapted method
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function as($alias) {}

    /**
     * Sets adapted method public.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePublic() {}

    /**
     * Sets adapted method protected.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeProtected() {}

    /**
     * Sets adapted method private.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makePrivate() {}

    /**
     * Adds overwritten traits.
     *
     * @param Node\Name|string ...$traits Traits for overwrite
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function insteadof(...$traits) {}

    protected function setModifier(int $modifier): void {}

    /**
     * Returns the built node.
     *
     * @return Node The built node
     */
    public function getNode(): Node {}
}
