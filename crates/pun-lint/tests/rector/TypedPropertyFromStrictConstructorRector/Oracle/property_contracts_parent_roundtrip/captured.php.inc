<?php

namespace QualityPropertyParentContracts;

class ParentType {}

final class ChildType extends ParentType
{
    private $private;
    protected $protected;
    public $documented;
    public $nullable;

    public function __construct(parent $private, parent $protected, parent $documented, ?parent $nullable)
    {
        $this->private = $private;
        $this->protected = $protected;
        $this->documented = $documented;
        $this->nullable = $nullable;
    }
}
-----
<?php

namespace QualityPropertyParentContracts;

class ParentType {}

final class ChildType extends ParentType
{
    private parent $private;
    protected parent $protected;
    /**
     * @var static
     */
    public $documented;
    public $nullable;

    public function __construct(parent $private, parent $protected, parent $documented, ?parent $nullable)
    {
        $this->private = $private;
        $this->protected = $protected;
        $this->documented = $documented;
        $this->nullable = $nullable;
    }
}
