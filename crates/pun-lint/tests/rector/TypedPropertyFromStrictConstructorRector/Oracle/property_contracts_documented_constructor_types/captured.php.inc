<?php

namespace QualityPropertyDocumentedConstructorTypes;

final class DocumentedParameters
{
    public $float;
    public $boolean;
    public $closure;
    public $object;
    public $iterable;
    /** @var float */
    private $floatDoc;
    /** @var bool */
    private $boolDoc;
    private $bothBooleans;
    private $bothFloats;

    public function __construct(float $float, bool $boolean, \Closure $closure, object $object, iterable $iterable, bool $condition)
    {
        $this->float = $float;
        $this->boolean = $boolean;
        $this->closure = $closure;
        $this->object = $object;
        $this->iterable = $iterable;
        $this->floatDoc = $float;
        $this->boolDoc = $boolean;
        if ($condition) {
            $this->bothBooleans = true;
            $this->bothFloats = 1.5;
        } else {
            $this->bothBooleans = false;
            $this->bothFloats = 2.5;
        }
    }
}
-----
<?php

namespace QualityPropertyDocumentedConstructorTypes;

final class DocumentedParameters
{
    /**
     * @var float
     */
    public $float;
    /**
     * @var bool
     */
    public $boolean;
    /**
     * @var \Closure
     */
    public $closure;
    /**
     * @var object
     */
    public $object;
    /**
     * @var iterable
     */
    public $iterable;
    private float $floatDoc;
    private bool $boolDoc;
    private bool $bothBooleans;
    private float $bothFloats;

    public function __construct(float $float, bool $boolean, \Closure $closure, object $object, iterable $iterable, bool $condition)
    {
        $this->float = $float;
        $this->boolean = $boolean;
        $this->closure = $closure;
        $this->object = $object;
        $this->iterable = $iterable;
        $this->floatDoc = $float;
        $this->boolDoc = $boolean;
        if ($condition) {
            $this->bothBooleans = true;
            $this->bothFloats = 1.5;
        } else {
            $this->bothBooleans = false;
            $this->bothFloats = 2.5;
        }
    }
}
