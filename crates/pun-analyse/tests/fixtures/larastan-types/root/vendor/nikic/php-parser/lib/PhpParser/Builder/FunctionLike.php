<?php declare(strict_types=1);

namespace PhpParser\Builder;

use PhpParser\BuilderHelpers;
use PhpParser\Node;

abstract class FunctionLike extends Declaration {
    protected bool $returnByRef = false;
    /** @var Node\Param[] */
    protected array $params = [];

    /** @var Node\Identifier|Node\Name|Node\ComplexType|null */
    protected ?Node $returnType = null;

    /**
     * Make the function return by reference.
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function makeReturnByRef() {}

    /**
     * Adds a parameter.
     *
     * @param Node\Param|Param $param The parameter to add
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function addParam($param) {}

    /**
     * Adds multiple parameters.
     *
     * @param (Node\Param|Param)[] $params The parameters to add
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function addParams(array $params) {}

    /**
     * Sets the return type for PHP 7.
     *
     * @param string|Node\Name|Node\Identifier|Node\ComplexType $type
     *
     * @return $this The builder instance (for fluid interface)
     */
    public function setReturnType($type) {}
}
